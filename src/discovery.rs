//! Network discovery module
//!
//! Discovers Deltasafe servers on the local network with a best-effort TCP port scan.
//! mDNS is not implemented; use an explicit `--target` address when the peer is known.

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;

/// Default port range for scanning
const DEFAULT_PORT_RANGE: std::ops::Range<u16> = 12340..12350;

/// How many hosts of the local /24 the scan sweeps.
///
/// A full /24 across [`DEFAULT_PORT_RANGE`] would open thousands of sockets; this keeps the
/// best-effort scan bounded. Use `--target` for a peer outside this window.
const SCAN_HOST_LIMIT: usize = 10;

/// Per-connection connect timeout. Shorter than the overall scan budget so that one
/// unresponsive host does not consume the whole `--timeout` allowance.
const CONNECT_TIMEOUT: Duration = Duration::from_millis(200);

/// Information about a discovered server
#[derive(Debug, Clone)]
pub struct DiscoveredServer {
    pub address: SocketAddr,
    pub name: Option<String>,
}

/// Discovers Deltasafe servers on the LAN with a best-effort port scan.
///
/// This is a heuristic: an open port in [`DEFAULT_PORT_RANGE`] means something is
/// listening, not that it is a Deltasafe receiver. Results are a convenience, never a
/// verified peer identity.
pub async fn discover_servers(timeout_secs: u64) -> Result<Vec<DiscoveredServer>> {
    println!("[🔍] Searching for Deltasafe servers on the LAN...");

    let mut servers = Vec::new();

    match discover_via_port_scan(timeout_secs).await {
        Ok(mut scan_servers) => {
            println!("[🔎] Found {} server(s) via port scan", scan_servers.len());
            servers.append(&mut scan_servers);
        }
        Err(e) => {
            println!("[⚠️] Port scan failed: {}", e);
        }
    }

    // Remove duplicates
    servers = deduplicate_servers(servers);

    if servers.is_empty() {
        println!("[ℹ️] No servers found. Try specifying IP:port manually.");
    } else {
        println!(
            "[✅] Discovered {} unique server(s) in total",
            servers.len()
        );
        for (i, server) in servers.iter().enumerate() {
            println!("  {}. {}", i + 1, server.address);
        }
    }

    Ok(servers)
}

/// Server discovery via port scanning.
///
/// The whole sweep is bounded by `timeout_secs`, which is the value the `discover` command
/// exposes as `--timeout`. The CLI rejects zero; a zero passed in programmatically expires the
/// budget immediately and reports a timeout rather than silently scanning for a second.
async fn discover_via_port_scan(timeout_secs: u64) -> Result<Vec<DiscoveredServer>> {
    let local_network = get_local_network_range()?;
    let budget = timeout_secs;

    println!(
        "[🔎] Scanning up to {SCAN_HOST_LIMIT} host(s) on the local network with a {budget}s budget..."
    );

    let ips: Vec<Ipv4Addr> = local_network.iter().take(SCAN_HOST_LIMIT).collect();

    let scan = async move {
        // JoinSet aborts the outstanding probes if the budget below expires.
        let mut set = tokio::task::JoinSet::new();
        for ip in ips {
            for port in DEFAULT_PORT_RANGE {
                let addr = SocketAddr::new(IpAddr::V4(ip), port);
                set.spawn(async move { check_deltasafe_server(addr).await });
            }
        }

        let mut servers = Vec::new();
        while let Some(result) = set.join_next().await {
            if let Ok(Some(server)) = result {
                servers.push(server);
            }
        }
        servers
    };

    match tokio::time::timeout(Duration::from_secs(budget), scan).await {
        Ok(servers) => Ok(servers),
        Err(_) => anyhow::bail!("port scan timed out after {budget}s"),
    }
}

/// Checks whether something is listening at the given address.
///
/// An accepted TCP connection is not proof that the peer is a Deltasafe receiver, only that a
/// port is open; the receiving side still authenticates every frame.
async fn check_deltasafe_server(addr: SocketAddr) -> Option<DiscoveredServer> {
    match tokio::time::timeout(CONNECT_TIMEOUT, tokio::net::TcpStream::connect(addr)).await {
        Ok(Ok(_)) => Some(DiscoveredServer {
            address: addr,
            name: None,
        }),
        _ => None,
    }
}

/// Finds the local network IP range
fn get_local_network_range() -> Result<NetworkRange> {
    use std::net::UdpSocket;

    // Find the local IP
    let socket = UdpSocket::bind("0.0.0.0:0").context("Could not create UDP socket")?;
    socket
        .connect("8.8.8.8:80")
        .context("Could not establish a test connection")?;
    let local_addr = socket
        .local_addr()
        .context("Could not get the local address")?;

    if let IpAddr::V4(local_ip) = local_addr.ip() {
        // Assume a /24 subnet (255.255.255.0)
        let octets = local_ip.octets();
        let network_base = Ipv4Addr::new(octets[0], octets[1], octets[2], 1);
        let network_end = Ipv4Addr::new(octets[0], octets[1], octets[2], 254);

        Ok(NetworkRange::new(network_base, network_end))
    } else {
        anyhow::bail!("IPv6 is not supported yet")
    }
}

/// Iterator over an IP range
struct NetworkRange {
    current: u32,
    end: u32,
}

impl NetworkRange {
    fn new(start: Ipv4Addr, end: Ipv4Addr) -> Self {
        Self {
            current: u32::from(start),
            end: u32::from(end),
        }
    }

    fn iter(&self) -> NetworkRangeIter {
        NetworkRangeIter {
            current: self.current,
            end: self.end,
        }
    }
}

struct NetworkRangeIter {
    current: u32,
    end: u32,
}

impl Iterator for NetworkRangeIter {
    type Item = Ipv4Addr;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current <= self.end {
            let ip = Ipv4Addr::from(self.current);
            self.current += 1;
            Some(ip)
        } else {
            None
        }
    }
}

/// Removes duplicate servers
fn deduplicate_servers(servers: Vec<DiscoveredServer>) -> Vec<DiscoveredServer> {
    let mut unique_servers = HashMap::new();

    for server in servers {
        // First result for an address wins.
        unique_servers.entry(server.address).or_insert(server);
    }

    unique_servers.into_values().collect()
}

/// Prompts the user to choose a server
pub fn select_server_interactive(servers: &[DiscoveredServer]) -> Option<&DiscoveredServer> {
    if servers.is_empty() {
        return None;
    }

    if servers.len() == 1 {
        println!("[✅] Found a single server: {}", servers[0].address);
        return Some(&servers[0]);
    }

    // If there are multiple servers, ask the user
    println!("[🔍] Found {} server(s). Please choose one:", servers.len());
    for (i, server) in servers.iter().enumerate() {
        println!("  {}. {}", i + 1, server.address);
        if let Some(name) = &server.name {
            println!("     Service name: {}", name);
        }
    }

    // Read user input
    loop {
        print!("Your choice (1-{}): ", servers.len());
        use std::io::{self, Write};
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if let Ok(choice) = input.trim().parse::<usize>() {
                    if choice >= 1 && choice <= servers.len() {
                        println!("[✅] Selected {}", servers[choice - 1].address);
                        return Some(&servers[choice - 1]);
                    }
                }
                println!(
                    "[⚠️] Invalid choice. Enter a number between 1 and {}.",
                    servers.len()
                );
            }
            Err(_) => {
                println!("[⚠️] Input error. Please try again.");
            }
        }
    }
}

/// Selects a server automatically (without user interaction)
///
/// The scan has no reliable ordering signal, so this takes the first result. Use `--target`
/// when the choice of peer matters.
pub fn select_best_server_auto(servers: &[DiscoveredServer]) -> Option<&DiscoveredServer> {
    let selected = servers.first()?;

    if servers.len() > 1 {
        println!(
            "[ℹ️] Found {} server(s); automatically selected {}",
            servers.len(),
            selected.address
        );
        println!("[💡] Use the 'deltasafe discover' command to see all servers");
    }

    Some(selected)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_range() {
        let start = Ipv4Addr::new(192, 168, 1, 1);
        let end = Ipv4Addr::new(192, 168, 1, 3);
        let range = NetworkRange::new(start, end);

        let ips: Vec<Ipv4Addr> = range.iter().collect();
        assert_eq!(ips.len(), 3);
        assert_eq!(ips[0], Ipv4Addr::new(192, 168, 1, 1));
        assert_eq!(ips[2], Ipv4Addr::new(192, 168, 1, 3));
    }

    #[test]
    fn test_deduplicate_servers() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)), 12345);

        let servers = vec![
            DiscoveredServer {
                address: addr,
                name: None,
            },
            DiscoveredServer {
                address: addr,
                name: Some("test".to_string()),
            },
        ];

        let unique = deduplicate_servers(servers);
        assert_eq!(unique.len(), 1);
        // The first result for an address is kept.
        assert!(unique[0].name.is_none());
    }
}
