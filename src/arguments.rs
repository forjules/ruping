use crate::i18n::{self, STRING_USAGE};
use crate::standard::*;

#[derive(Debug, Clone)]
pub struct Arguments {
    pub destination: String,
    pub port: u16,
    pub timeout: u64,
    pub count: i32,
    pub continuous: bool,
    pub use_color: bool,
}

impl Arguments {
    pub fn print_banner() {
        println!("{} v{} - Copyright (c) {} {}\n", PROGRAM, VERSION, YEAR, AUTHOR);
    }

    pub fn print_usage() {
        println!("{}", i18n::get_string(STRING_USAGE));
    }

    pub fn process(args: &[String]) -> Result<Arguments, i32> {
        let mut port: i32 = 0;
        let mut count: i32 = 0;
        let mut timeout: i32 = 1000;
        let mut use_color = true;
        let mut destination: Option<String> = None;

        let mut i = 1;
        while i < args.len() {
            let arg = &args[i];

            if arg == "-?" || arg == "--help" {
                return Err(ERROR_INVALIDARGUMENTS);
            }

            if arg == "-p" || arg == "--port" {
                if i + 1 >= args.len() {
                    return Err(ERROR_INVALIDARGUMENTS);
                }
                port = args[i + 1].parse::<i32>().unwrap_or(0);
                i += 2;
                continue;
            }

            if arg == "-c" || arg == "--count" {
                if i + 1 >= args.len() {
                    return Err(ERROR_INVALIDARGUMENTS);
                }
                count = args[i + 1].parse::<i32>().unwrap_or(0);
                i += 2;
                continue;
            }

            if arg == "-t" || arg == "--timeout" {
                if i + 1 >= args.len() {
                    return Err(ERROR_INVALIDARGUMENTS);
                }
                timeout = args[i + 1].parse::<i32>().unwrap_or(0);
                i += 2;
                continue;
            }

            if arg == "--nocolor" {
                use_color = false;
                i += 1;
                continue;
            }

            if destination.is_none() {
                destination = Some(arg.clone());
                i += 1;
            } else {
                return Err(ERROR_INVALIDARGUMENTS);
            }
        }

        let destination = match destination {
            Some(d) => d,
            None => return Err(ERROR_INVALIDARGUMENTS),
        };

        if port <= 0 || port > 65535 {
            return Err(ERROR_INVALIDARGUMENTS);
        }

        if timeout <= 0 {
            return Err(ERROR_INVALIDARGUMENTS);
        }

        let continuous = count <= 0;
        let count = if count <= 0 { -1 } else { count };

        Ok(Arguments {
            destination,
            port: port as u16,
            timeout: timeout as u64,
            count,
            continuous,
            use_color,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_args_parsing() {
        let args = vec![
            "ruping".to_string(),
            "-p".to_string(),
            "80".to_string(),
            "example.com".to_string(),
            "-c".to_string(),
            "5".to_string(),
            "-t".to_string(),
            "500".to_string(),
            "--nocolor".to_string(),
        ];
        let parsed = Arguments::process(&args).unwrap();
        assert_eq!(parsed.destination, "example.com");
        assert_eq!(parsed.port, 80);
        assert_eq!(parsed.count, 5);
        assert!(!parsed.continuous);
        assert_eq!(parsed.timeout, 500);
        assert!(!parsed.use_color);
    }

    #[test]
    fn test_default_values() {
        let args = vec![
            "ruping".to_string(),
            "-p".to_string(),
            "443".to_string(),
            "1.1.1.1".to_string(),
        ];
        let parsed = Arguments::process(&args).unwrap();
        assert_eq!(parsed.destination, "1.1.1.1");
        assert_eq!(parsed.port, 443);
        assert_eq!(parsed.count, -1);
        assert!(parsed.continuous);
        assert_eq!(parsed.timeout, 1000);
        assert!(parsed.use_color);
    }
}
