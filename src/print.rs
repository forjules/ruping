use std::io::{self, Write};

pub const PRINT_COLOR_NONE: i32 = 0;
pub const PRINT_COLOR_RED: i32 = 31;
pub const PRINT_COLOR_GREEN: i32 = 32;
pub const PRINT_COLOR_YELLOW: i32 = 33;
pub const PRINT_COLOR_BLUE: i32 = 34;

pub fn format_text(color: i32, data: &str, use_color: bool) -> String {
    let mut result = String::with_capacity(data.len() + 32);
    let parts: Vec<&str> = data.split('$').collect();

    for (i, part) in parts.iter().enumerate() {
        if i % 2 == 1 {
            // Content inside $ ... $
            if use_color && color != PRINT_COLOR_NONE {
                result.push_str(&format!("\x1b[01;{}m{}\x1b[22;0m", color, part));
            } else {
                result.push_str(part);
            }
        } else {
            // Content outside $ ... $
            result.push_str(part);
        }
    }

    result
}

pub fn formatted_print(color: i32, data: &str, use_color: bool) {
    let formatted = format_text(color, data, use_color);
    print!("{}", formatted);
    let _ = io::stdout().flush();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_text_no_color() {
        let input = "Connecting to $1.1.1.1$ on $TCP 80$:";
        let out = format_text(PRINT_COLOR_YELLOW, input, false);
        assert_eq!(out, "Connecting to 1.1.1.1 on TCP 80:");
    }

    #[test]
    fn test_format_text_with_color() {
        let input = "Connecting to $1.1.1.1$ on $TCP 80$:";
        let out = format_text(PRINT_COLOR_YELLOW, input, true);
        assert_eq!(
            out,
            "Connecting to \x1b[01;33m1.1.1.1\x1b[22;0m on \x1b[01;33mTCP 80\x1b[22;0m:"
        );
    }
}
