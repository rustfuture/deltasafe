//! Network discovery module
//!
//! This module automatically discovers Deltasafe servers on the LAN using
//! mDNS (Bonjour/Zeroconf) and simple port scanning.

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;

/// mDNS service type for the Deltasafe service
#[allow(dead_code)] // Allowed for now because the mDNS implementation is not complete yet
const DELTASAFE_SERVICE_TYPE: &str = "_deltasafe._tcp.local.";

/// Default port range for scanning
const DEFAULT_PORT_RANGE: std::ops::Range<u16> = 12340..12350;

/// Information about a discovered server
#[derive(Debug, Clone)]
pub struct DiscoveredServer {
    pub address: SocketAddr,
    pub name: Option<String>,
    pub discovery_method: DiscoveryMethod,
}

#[derive(Debug, Clone)]
#[allow(dead_code)] // Allowed for now because the mDNS implementation is not complete yet
pub enum DiscoveryMethod {
    MDns,
    PortScan,
}

/// Discovers Deltasafe servers on the LAN
pub async fn discover_servers(timeout_secs: u64) -> Result<Vec<DiscoveredServer>> {
    println!("[🔍] Searching for Deltasafe servers on the LAN...");

    let mut servers = Vec::new();

    // 1. Try discovery via mDNS
    match discover_via_mdns(timeout_secs).await {
        Ok(mut mdns_servers) => {
            println!("[📡] Found {} server(s) via mDNS", mdns_servers.len());
            servers.append(&mut mdns_servers);
        }
        Err(e) => {
            println!("[⚠️] mDNS discovery failed: {}", e);
        }
    }

    // 2. Discovery via port scanning
    match discover_via_port_scan().await {
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
            println!(
                "  {}. {} ({:?})",
                i + 1,
                server.address,
                server.discovery_method
            );
        }
    }

    Ok(servers)
}

/// Server discovery using mDNS
async fn discover_via_mdns(_timeout_secs: u64) -> Result<Vec<DiscoveredServer>> {
    // mDNS is a simple implementation for now; real mDNS is complex
    println!("[📡] Trying mDNS discovery... (simple implementation)");

    // Return an empty list for now; real mDNS will be added later
    tokio::time::sleep(Duration::from_millis(100)).await;

    Ok(Vec::new())
}

/// Server discovery via port scanning
async fn discover_via_port_scan() -> Result<Vec<DiscoveredServer>> {
    let local_network = get_local_network_range()?;
    let mut servers = Vec::new();

    println!("[🔎] Scanning ports on the local network...");

    // Parallel port scan (test only a few IPs to avoid too many)
    let mut tasks = Vec::new();
    let ips: Vec<Ipv4Addr> = local_network.iter().take(10).collect(); // First 10 IPs

    for ip in ips {
        for port in DEFAULT_PORT_RANGE {
            let addr = SocketAddr::new(IpAddr::V4(ip), port);
            let task = tokio::spawn(async move { check_deltasafe_server(addr).await });
            tasks.push(task);
        }
    }

    // Wait for all scans
    for task in tasks {
        if let Ok(Some(server)) = task.await {
            servers.push(server);
        }
    }

    Ok(servers)
}

/// Checks whether a Deltasafe server is present at the given address
async fn check_deltasafe_server(addr: SocketAddr) -> Option<DiscoveredServer> {
    // Use a Tokio TcpStream
    match tokio::time::timeout(
        Duration::from_millis(100),
        tokio::net::TcpStream::connect(addr),
    )
    .await
    {
        Ok(Ok(_)) => {
            // Connection succeeded; there is probably a server
            Some(DiscoveredServer {
                address: addr,
                name: None,
                discovery_method: DiscoveryMethod::PortScan,
            })
        }
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
        // Merge servers at the same address, preferring mDNS
        match unique_servers.get(&server.address) {
            Some(_existing) => {
                if matches!(server.discovery_method, DiscoveryMethod::MDns) {
                    unique_servers.insert(server.address, server);
                }
            }
            None => {
                unique_servers.insert(server.address, server);
            }
        }
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
        println!(
            "  {}. {} ({:?})",
            i + 1,
            server.address,
            server.discovery_method
        );
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
pub fn select_best_server_auto(servers: &[DiscoveredServer]) -> Option<&DiscoveredServer> {
    if servers.is_empty() {
        return None;
    }

    // Prefer servers found via mDNS; otherwise take the first one
    let selected = servers
        .iter()
        .find(|s| matches!(s.discovery_method, DiscoveryMethod::MDns))
        .or_else(|| servers.first())?;

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
                discovery_method: DiscoveryMethod::PortScan,
            },
            DiscoveredServer {
                address: addr,
                name: Some("test".to_string()),
                discovery_method: DiscoveryMethod::MDns,
            },
        ];

        let unique = deduplicate_servers(servers);
        assert_eq!(unique.len(), 1);
        assert!(matches!(unique[0].discovery_method, DiscoveryMethod::MDns));
    }
}
