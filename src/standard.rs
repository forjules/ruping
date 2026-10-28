// Standard definitions and error codes matching paping original C++

pub const PROGRAM: &str = "ruping";
pub const VERSION: &str = "2";
pub const AUTHOR: &str = "jSSH.app";
pub const YEAR: &str = "2026";

// Error codes
pub const SUCCESS: i32 = 0;
pub const ERROR_POUTOFMEMORY: i32 = 100;
pub const ERROR_SOCKET_CANNOTRESOLVE: i32 = 101;
pub const ERROR_SOCKET_TIMEOUT: i32 = 102;
pub const ERROR_SOCKET_GENERALFAILURE: i32 = 103;
pub const ERROR_INVALIDARGUMENTS: i32 = 200;
