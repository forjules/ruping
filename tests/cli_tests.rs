use std::process::Command;

fn run_ruping(args: &[&str]) -> (i32, String) {
    let output = Command::new(env!("CARGO_BIN_EXE_ruping"))
        .args(args)
        .output()
        .expect("Failed to execute ruping binary");

    let code = output.status.code().unwrap_or(-1);
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    (code, stdout)
}

#[test]
fn test_no_args_shows_usage_and_exits_200() {
    let (code, output) = run_ruping(&[]);
    assert_eq!(code, 200);
    assert!(output.contains("ruping v2 - Copyright (c) 2026 jSSH.app"));
    assert!(output.contains("Syntax: ruping [options] destination"));
    assert!(output.contains("-p, --port N\tset TCP port N (required)"));
}

#[test]
fn test_help_flag_exits_200() {
    let (code, output) = run_ruping(&["--help"]);
    assert_eq!(code, 200);
    assert!(output.contains("Syntax: ruping [options] destination"));

    let (code_q, output_q) = run_ruping(&["-?"]);
    assert_eq!(code_q, 200);
    assert_eq!(output, output_q);
}

#[test]
fn test_missing_port_exits_200() {
    let (code, output) = run_ruping(&["1.1.1.1"]);
    assert_eq!(code, 200);
    assert!(output.contains("Syntax: ruping [options] destination"));
}

#[test]
fn test_invalid_port_exits_200() {
    let (code, _) = run_ruping(&["1.1.1.1", "-p", "0"]);
    assert_eq!(code, 200);

    let (code2, _) = run_ruping(&["1.1.1.1", "-p", "-10"]);
    assert_eq!(code2, 200);

    let (code3, _) = run_ruping(&["1.1.1.1", "-p", "abc"]);
    assert_eq!(code3, 200);
}

#[test]
fn test_invalid_timeout_exits_200() {
    let (code, _) = run_ruping(&["1.1.1.1", "-p", "80", "-t", "0"]);
    assert_eq!(code, 200);

    let (code2, _) = run_ruping(&["1.1.1.1", "-p", "80", "-t", "-10"]);
    assert_eq!(code2, 200);
}

#[test]
fn test_multiple_destinations_exits_200() {
    let (code, _) = run_ruping(&["-p", "80", "1.1.1.1", "2.2.2.2"]);
    assert_eq!(code, 200);
}

#[test]
fn test_unresolvable_host_exits_101() {
    let (code, output) = run_ruping(&["this-domain-does-not-exist.test", "-p", "80", "--nocolor"]);
    assert_eq!(code, 101);
    assert!(output.contains("Cannot resolve host"));
}

#[test]
fn test_closed_port_fails_and_exits_1() {
    let (code, output) = run_ruping(&["127.0.0.1", "-p", "44444", "-c", "2", "-t", "200", "--nocolor"]);
    assert_eq!(code, 1);
    assert!(output.contains("Connecting to 127.0.0.1 on TCP 44444:"));
    assert!(output.contains("Connection timed out"));
    assert!(output.contains("Attempted = 2, Connected = 0, Failed = 2 (100.00%)"));
    assert!(output.contains("Minimum = 0.00ms, Maximum = 0.00ms, Average = 0.00ms"));
}

#[test]
fn test_successful_connect_exits_0() {
    // Start a temporary local listener
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind test listener");
    let port = listener.local_addr().unwrap().port().to_string();

    std::thread::spawn(move || {
        for _ in 0..2 {
            if let Ok((_stream, _)) = listener.accept() {}
        }
    });

    let (code, output) = run_ruping(&["127.0.0.1", "-p", &port, "-c", "2", "--nocolor"]);
    assert_eq!(code, 0);
    assert!(output.contains(&format!("Connecting to 127.0.0.1 on TCP {}:", port)));
    assert!(output.contains(&format!("Connected to 127.0.0.1: time=")));
    assert!(output.contains("Attempted = 2, Connected = 2, Failed = 0 (0.00%)"));
}
