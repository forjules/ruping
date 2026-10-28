use std::net::IpAddr;

#[derive(Debug, Clone)]
pub struct Host {
    pub hostname: String,
    pub ip_address: IpAddr,
    pub host_is_ip: bool,
    pub port: u16,
    pub protocol_name: String,
}

impl Host {
    pub fn ip_address_string(&self) -> String {
        self.ip_address.to_string()
    }

    pub fn get_connect_info_string(&self) -> String {
        if self.host_is_ip {
            format!(
                "Connecting to ${}$ on ${} {}$:",
                self.hostname, self.protocol_name, self.port
            )
        } else {
            format!(
                "Connecting to ${}$ [${}$] on ${} {}$:",
                self.hostname,
                self.ip_address_string(),
                self.protocol_name,
                self.port
            )
        }
    }

    pub fn get_successful_connection_string(&self, time: f64) -> String {
        format!(
            "Connected to ${}$: time=${:.2}ms$ protocol=${}$ port=${}$",
            self.ip_address_string(),
            time,
            self.protocol_name,
            self.port
        )
    }
}
