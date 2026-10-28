mod arguments;
mod host;
mod i18n;
mod print;
mod socket;
mod standard;
mod stats;

use std::env;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use arguments::Arguments;
use print::*;
use socket::Socket;
use standard::*;
use stats::Stats;

fn print_error(error: i32, use_color: bool) {
    let err_str = i18n::get_string(error);
    formatted_print(PRINT_COLOR_RED, err_str, use_color);
    println!();
}

fn print_connect_info(host: &host::Host, use_color: bool) {
    let info = host.get_connect_info_string();
    formatted_print(PRINT_COLOR_YELLOW, &info, use_color);
    println!("\n");
}

fn main() {
    let raw_args: Vec<String> = env::args().collect();

    Arguments::print_banner();

    let arguments = match Arguments::process(&raw_args) {
        Ok(args) => args,
        Err(_) => {
            Arguments::print_usage();
            std::process::exit(ERROR_INVALIDARGUMENTS);
        }
    };

    let host = match Socket::resolve(&arguments.destination, arguments.port) {
        Ok(h) => h,
        Err(err) => {
            print_error(err, arguments.use_color);
            std::process::exit(err);
        }
    };

    print_connect_info(&host, arguments.use_color);

    let stats = Arc::new(Mutex::new(Stats::new()));
    let exit_code = Arc::new(AtomicI32::new(SUCCESS));
    let printed_stats = Arc::new(AtomicBool::new(false));

    // Ctrl-C Signal Handler
    let stats_sig = Arc::clone(&stats);
    let exit_code_sig = Arc::clone(&exit_code);
    let printed_stats_sig = Arc::clone(&printed_stats);
    let use_color = arguments.use_color;

    let _ = ctrlc::set_handler(move || {
        if !printed_stats_sig.swap(true, Ordering::SeqCst) {
            let s = stats_sig.lock().unwrap();
            let stats_str = s.get_statistics_string();
            formatted_print(PRINT_COLOR_BLUE, &stats_str, use_color);
            println!();
        }
        std::process::exit(exit_code_sig.load(Ordering::SeqCst));
    });

    let mut i: u32 = 0;
    let mut time: f64;

    while arguments.continuous || i < arguments.count as u32 {
        let connect_result = Socket::connect(&host, arguments.timeout);

        let mut s = stats.lock().unwrap();
        s.attempts += 1;

        match connect_result {
            Ok(t) => {
                time = t;
                s.connects += 1;
                s.total += time;
                s.update_max_min(time);

                let success_str = host.get_successful_connection_string(time);
                formatted_print(PRINT_COLOR_GREEN, &success_str, arguments.use_color);
                println!();
            }
            Err(err) => {
                time = 0.0;
                exit_code.store(1, Ordering::SeqCst);
                s.failures += 1;

                let err_str = i18n::get_string(err);
                formatted_print(PRINT_COLOR_RED, err_str, arguments.use_color);
                println!();
            }
        }
        drop(s);

        if (time as i64) < 1000 {
            let sleep_micros = (1000 - (time as i64)) * 1000;
            thread::sleep(Duration::from_micros(sleep_micros as u64));
        }

        i += 1;
    }

    if !printed_stats.swap(true, Ordering::SeqCst) {
        let s = stats.lock().unwrap();
        let stats_str = s.get_statistics_string();
        formatted_print(PRINT_COLOR_BLUE, &stats_str, arguments.use_color);
        println!();
    }

    std::process::exit(exit_code.load(Ordering::SeqCst));
}
