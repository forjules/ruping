use crate::standard::*;

pub const STRING_USAGE: i32 = 1000;
pub const STRING_CONNECT_INFO_FULL: i32 = 1001;
pub const STRING_CONNECT_INFO_IP: i32 = 1002;
pub const STRING_CONNECT_SUCCESS: i32 = 1003;
pub const STRING_STATS: i32 = 1004;

pub fn get_string(id: i32) -> &'static str {
    match id {
        STRING_USAGE => {
            "Syntax: ruping [options] destination\n\n\
             Options:\n\
             \x20-?, --help\tdisplay usage\n\
             \x20-p, --port N\tset TCP port N (required)\n\
             \x20    --nocolor\tDisable color output\n\
             \x20-t, --timeout\ttimeout in milliseconds (default 1000)\n\
             \x20-c, --count N\tset number of checks to N\n"
        }
        STRING_STATS => {
            "\nConnection statistics:\n\
             \tAttempted = $%d$, Connected = $%d$, Failed = $%d$ ($%.2f%%$)\n\
             Approximate connection times:\n\
             \tMinimum = $%.2fms$, Maximum = $%.2fms$, Average = $%.2fms$\n"
        }
        ERROR_POUTOFMEMORY => "Out of memory",
        ERROR_SOCKET_TIMEOUT => "$Connection timed out$",
        ERROR_SOCKET_GENERALFAILURE => "$General failure$",
        ERROR_SOCKET_CANNOTRESOLVE => "$Cannot resolve host$",
        STRING_CONNECT_INFO_FULL => "Connecting to $%s$ [$%s$] on $%s %d$:",
        STRING_CONNECT_INFO_IP => "Connecting to $%s$ on $%s %d$:",
        STRING_CONNECT_SUCCESS => "Connected to $%s$: time=$%.2fms$ protocol=$%s$ port=$%d$",
        _ => "[ERROR: No translation]",
    }
}
