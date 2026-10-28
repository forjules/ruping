# ruPING - paping built in rust for linux

[![CI & Packaging](https://github.com/forjules/ruping/actions/workflows/ci.yml/badge.svg)](https://github.com/forjules/ruping/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Platform: Linux](https://img.shields.io/badge/Platform-Linux-orange.svg)](#installation)
[![Rust: 2024](https://img.shields.io/badge/Rust-2024%20Edition-black.svg)](https://www.rust-lang.org)

**ruPING** is a modern, high-performance, and memory-safe Linux reimplementation of the classic **`paping`** TCP ping tool, written in Rust.

Standard ICMP `ping` only tests whether a remote host replies to ICMP echo requests—often blocked by firewalls or cloud security groups. **ruPING** measures TCP SYN/ACK connection handshake latency to any specific port, allowing you to accurately test service reachability, firewall rules, and application response latency.

---

## ⚡ Features

- **Strict CLI Compatibility**: Complete drop-in replacement for `paping` with matching arguments, output syntax, and exit codes.
- **Accurate Sub-Millisecond Timing**: Measures TCP SYN-ACK handshake round-trip times with microsecond-level precision.
- **Color-Coded Terminal Output**: Visual indicators for connection targets (Yellow), successful probes (Green), timeouts/errors (Red), and summary statistics (Blue).
- **Script-Friendly**: `--nocolor` flag strips all ANSI escape codes for clean piping to logs and monitoring scripts.
- **Canonical Hostname Detection**: Automatically resolves FQDNs, aliases (CNAMEs), and IPv4/IPv6 literals.
- **Signal-Aware**: Catches `SIGINT` (Ctrl+C) and immediately prints full connection statistics before exiting.
- **Native 64-bit Linux Binary**: No multiarch dependencies, no legacy C++ socket bugs, zero garbage collection pauses.

---

## 📦 Installation

### Method 1: Debian / Ubuntu / Pop!_OS (`.deb` Package)

Download and install the prebuilt `.deb` package:

```bash
# Direct install from the repo
sudo dpkg -i dist/ruping_2.0.0-1_amd64.deb

# Or download from latest GitHub releases
wget https://github.com/forjules/ruping/releases/latest/download/ruping_2.0.0-1_amd64.deb
sudo dpkg -i ruping_2.0.0-1_amd64.deb
```

To uninstall:
```bash
sudo apt remove ruping
```

---

### Method 2: One-Line Installer

Run the included installer to build and link `ruping` directly into `/usr/local/bin`:

```bash
git clone https://github.com/forjules/ruping.git
cd ruping
./install.sh
```

---

### Method 3: Install via Cargo

If you have Rust and Cargo installed:

```bash
cargo install --git https://github.com/forjules/ruping.git
```

Or from a local clone:

```bash
cargo install --path .
```

---

## 🚀 Syntax & Usage

```text
Syntax: ruping [options] destination

Options:
 -?, --help	display usage
 -p, --port N	set TCP port N (required)
     --nocolor	Disable color output
 -t, --timeout	timeout in milliseconds (default 1000)
 -c, --count N	set number of checks to N
```

---

## 💡 Examples

### 1. Test Web Server (Port 80) Continuously
```bash
ruping 1.1.1.1 -p 80
```
*Press `Ctrl+C` at any time to break and view connection statistics.*

```text
ruping v2 - Copyright (c) 2026 jSSH.app

Connecting to 1.1.1.1 on TCP 80:

Connected to 1.1.1.1: time=2.87ms protocol=TCP port=80
Connected to 1.1.1.1: time=2.94ms protocol=TCP port=80
Connected to 1.1.1.1: time=2.80ms protocol=TCP port=80

Connection statistics:
	Attempted = 3, Connected = 3, Failed = 0 (0.00%)
Approximate connection times:
	Minimum = 2.80ms, Maximum = 2.94ms, Average = 2.87ms
```

### 2. Test Domain Name with Limited Count and Custom Timeout
```bash
ruping google.com -p 443 -c 4 -t 500
```

### 3. Plain Text Output for Logs / Automation
```bash
ruping 1.1.1.1 -p 80 -c 2 --nocolor >> /var/log/network_check.log
```

### 4. Check Closed or Filtered Port
```bash
ruping 127.0.0.1 -p 44444 -c 2
```
```text
ruping v2 - Copyright (c) 2026 jSSH.app

Connecting to 127.0.0.1 on TCP 44444:

Connection timed out
Connection timed out

Connection statistics:
	Attempted = 2, Connected = 0, Failed = 2 (100.00%)
Approximate connection times:
	Minimum = 0.00ms, Maximum = 0.00ms, Average = 0.00ms
```

---

## 🚦 Exit Codes

`ruPING` returns standard POSIX exit codes, making it seamless for shell scripting and CI/CD pipelines:

| Exit Code | Constant | Meaning |
|:---------:|:---------|:--------|
| **`0`** | `SUCCESS` | All connection attempts completed successfully |
| **`1`** | `FAILURE` | One or more probes timed out or failed to connect |
| **`101`** | `ERROR_SOCKET_CANNOTRESOLVE` | Target hostname could not be resolved |
| **`200`** | `ERROR_INVALIDARGUMENTS` | Invalid arguments, missing required options, or help displayed |

---

## 🛠️ Development & Building

```bash
# Clone the repository
git clone https://github.com/forjules/ruping.git
cd ruping

# Run test suite (unit and CLI integration tests)
cargo test

# Run linter
cargo clippy

# Build optimized release binary
cargo build --release

# Build standalone .deb package
./scripts/build-deb.sh
```

---

## 📄 License & Credits

Distributed under the MIT License. See [LICENSE](LICENSE) for details.

- **ruPING** by [jSSH.app](https://jssh.app)
- Original **paping** by Mike Lovell
