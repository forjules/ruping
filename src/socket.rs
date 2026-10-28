use std::ffi::{CStr, CString};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::time::{Duration, Instant};

use crate::host::Host;
use crate::standard::*;

unsafe extern "C" {
    fn gethostbyname(name: *const libc::c_char) -> *mut libc::hostent;
}

pub struct Socket;

impl Socket {
    pub fn resolve(destination: &str, port: u16) -> Result<Host, i32> {
        let c_destination = CString::new(destination).map_err(|_| ERROR_SOCKET_CANNOTRESOLVE)?;
        let hostent_ptr = unsafe { gethostbyname(c_destination.as_ptr()) };

        if !hostent_ptr.is_null() {
            let hostent = unsafe { &*hostent_ptr };
            if hostent.h_addr_list.is_null() {
                return Err(ERROR_SOCKET_CANNOTRESOLVE);
            }
            let first_addr = unsafe { *hostent.h_addr_list };
            if first_addr.is_null() {
                return Err(ERROR_SOCKET_CANNOTRESOLVE);
            }

            let canonical_name = unsafe {
                CStr::from_ptr(hostent.h_name)
                    .to_string_lossy()
                    .into_owned()
            };

            let raw_addr_ptr = first_addr as *const libc::in_addr;
            let s_addr = unsafe { (*raw_addr_ptr).s_addr };
            let ipv4 = Ipv4Addr::from(u32::from_be(s_addr));
            let host_is_ip = destination == canonical_name;

            return Ok(Host {
                hostname: canonical_name,
                ip_address: IpAddr::V4(ipv4),
                host_is_ip,
                port,
                protocol_name: "TCP".to_string(),
            });
        }

        // Fallback for IPv6 literals if passed
        if let Ok(ip) = destination.parse::<IpAddr>() {
            return Ok(Host {
                hostname: destination.to_string(),
                ip_address: ip,
                host_is_ip: true,
                port,
                protocol_name: "TCP".to_string(),
            });
        }

        Err(ERROR_SOCKET_CANNOTRESOLVE)
    }

    pub fn connect(host: &Host, timeout_ms: u64) -> Result<f64, i32> {
        let socket_addr = SocketAddr::new(host.ip_address, host.port);
        let timeout = Duration::from_millis(timeout_ms);

        let start = Instant::now();
        match TcpStream::connect_timeout(&socket_addr, timeout) {
            Ok(_) => {
                let duration = start.elapsed();
                let time_ms = duration.as_secs_f64() * 1000.0;
                Ok(time_ms)
            }
            Err(e) => {
                if let Some(code) = e.raw_os_error()
                    && matches!(code, libc::EMFILE | libc::ENFILE | libc::EACCES)
                {
                    return Err(ERROR_SOCKET_GENERALFAILURE);
                }
                Err(ERROR_SOCKET_TIMEOUT)
            }
        }
    }
}
