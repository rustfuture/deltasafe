//! Ağ keşif modülü
//! 
//! Bu modül LAN üzerindeki Deltasafe sunucularını otomatik olarak keşfetmek için
//! mDNS (Bonjour/Zeroconf) ve basit port tarama yöntemlerini kullanır.

use anyhow::{Result, Context};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::time::Duration;
use std::collections::HashMap;

/// Deltasafe servisi için mDNS service type
const DELTASAFE_SERVICE_TYPE: &str = "_deltasafe._tcp.local.";

/// Varsayılan port aralığı tarama için
const DEFAULT_PORT_RANGE: std::ops::Range<u16> = 12340..12350;

/// Keşfedilen sunucu bilgisi
#[derive(Debug, Clone)]
pub struct DiscoveredServer {
    pub address: SocketAddr,
    pub name: Option<String>,
    pub discovery_method: DiscoveryMethod,
}

#[derive(Debug, Clone)]
pub enum DiscoveryMethod {
    MDns,
    PortScan,
}

/// LAN'daki Deltasafe sunucularını keşfeder
pub async fn discover_servers(timeout_secs: u64) -> Result<Vec<DiscoveredServer>> {
    println!("[🔍] LAN'da Deltasafe sunucuları aranıyor...");
    
    let mut servers = Vec::new();
    
    // 1. mDNS ile keşif dene
    match discover_via_mdns(timeout_secs).await {
        Ok(mut mdns_servers) => {
            println!("[📡] mDNS ile {} sunucu bulundu", mdns_servers.len());
            servers.append(&mut mdns_servers);
        },
        Err(e) => {
            println!("[⚠️] mDNS keşfi başarısız: {}", e);
        }
    }
    
    // 2. Port tarama ile keşif
    match discover_via_port_scan().await {
        Ok(mut scan_servers) => {
            println!("[🔎] Port tarama ile {} sunucu bulundu", scan_servers.len());
            servers.append(&mut scan_servers);
        },
        Err(e) => {
            println!("[⚠️] Port tarama başarısız: {}", e);
        }
    }
    
    // Duplikatları temizle
    servers = deduplicate_servers(servers);
    
    if servers.is_empty() {
        println!("[ℹ️] Hiç sunucu bulunamadı. Manuel IP:port belirtmeyi deneyin.");
    } else {
        println!("[✅] Toplam {} benzersiz sunucu keşfedildi", servers.len());
        for (i, server) in servers.iter().enumerate() {
            println!("  {}. {} ({:?})", i + 1, server.address, server.discovery_method);
        }
    }
    
    Ok(servers)
}

/// mDNS kullanarak sunucu keşfi
async fn discover_via_mdns(timeout_secs: u64) -> Result<Vec<DiscoveredServer>> {
    // mDNS şimdilik basit implementasyon - gerçek mDNS karmaşık
    println!("[📡] mDNS keşfi deneniyor... (basit implementasyon)");
    
    // Şimdilik boş liste döndür, gelecekte gerçek mDNS eklenecek
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    Ok(Vec::new())
}

/// Port tarama ile sunucu keşfi
async fn discover_via_port_scan() -> Result<Vec<DiscoveredServer>> {
    let local_network = get_local_network_range()?;
    let mut servers = Vec::new();
    
    println!("[🔎] Yerel ağda port taraması yapılıyor...");
    
    // Paralel port tarama (sadece birkaç IP test et, çok fazla olmasın)
    let mut tasks = Vec::new();
    let ips: Vec<Ipv4Addr> = local_network.iter().take(10).collect(); // İlk 10 IP
    
    for ip in ips {
        for port in DEFAULT_PORT_RANGE {
            let addr = SocketAddr::new(IpAddr::V4(ip), port);
            let task = tokio::spawn(async move {
                check_deltasafe_server(addr).await
            });
            tasks.push(task);
        }
    }
    
    // Tüm taramaları bekle
    for task in tasks {
        if let Ok(Some(server)) = task.await {
            servers.push(server);
        }
    }
    
    Ok(servers)
}

/// Belirli bir adreste Deltasafe sunucusu olup olmadığını kontrol eder
async fn check_deltasafe_server(addr: SocketAddr) -> Option<DiscoveredServer> {
    // Tokio TcpStream kullan
    match tokio::time::timeout(Duration::from_millis(100), tokio::net::TcpStream::connect(addr)).await {
        Ok(Ok(_)) => {
            // Bağlantı başarılı, muhtemelen bir sunucu var
            Some(DiscoveredServer {
                address: addr,
                name: None,
                discovery_method: DiscoveryMethod::PortScan,
            })
        },
        _ => None,
    }
}

/// Yerel ağ IP aralığını bulur
fn get_local_network_range() -> Result<NetworkRange> {
    use std::net::UdpSocket;
    
    // Yerel IP'yi bul
    let socket = UdpSocket::bind("0.0.0.0:0").context("UDP socket oluşturulamadı")?;
    socket.connect("8.8.8.8:80").context("Test bağlantısı kurulamadı")?;
    let local_addr = socket.local_addr().context("Yerel adres alınamadı")?;
    
    if let IpAddr::V4(local_ip) = local_addr.ip() {
        // /24 subnet varsay (255.255.255.0)
        let octets = local_ip.octets();
        let network_base = Ipv4Addr::new(octets[0], octets[1], octets[2], 1);
        let network_end = Ipv4Addr::new(octets[0], octets[1], octets[2], 254);
        
        Ok(NetworkRange::new(network_base, network_end))
    } else {
        anyhow::bail!("IPv6 henüz desteklenmiyor")
    }
}

/// IP aralığı için iterator
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

/// Duplikat sunucuları temizler
fn deduplicate_servers(servers: Vec<DiscoveredServer>) -> Vec<DiscoveredServer> {
    let mut unique_servers = HashMap::new();
    
    for server in servers {
        // Aynı adresteki sunucuları birleştir, mDNS'i tercih et
        match unique_servers.get(&server.address) {
            Some(_existing) => {
                if matches!(server.discovery_method, DiscoveryMethod::MDns) {
                    unique_servers.insert(server.address, server);
                }
            },
            None => {
                unique_servers.insert(server.address, server);
            }
        }
    }
    
    unique_servers.into_values().collect()
}

/// Kullanıcıya sunucu seçimi yaptırır
pub fn select_server_interactive(servers: &[DiscoveredServer]) -> Option<&DiscoveredServer> {
    if servers.is_empty() {
        return None;
    }
    
    if servers.len() == 1 {
        println!("[✅] Tek sunucu bulundu: {}", servers[0].address);
        return Some(&servers[0]);
    }
    
    // Birden fazla sunucu varsa kullanıcıya sor
    println!("[🔍] {} sunucu bulundu. Lütfen birini seçin:", servers.len());
    for (i, server) in servers.iter().enumerate() {
        println!("  {}. {} ({:?})", i + 1, server.address, server.discovery_method);
        if let Some(name) = &server.name {
            println!("     Servis adı: {}", name);
        }
    }
    
    // Kullanıcı girişi al
    loop {
        print!("Seçiminiz (1-{}): ", servers.len());
        use std::io::{self, Write};
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if let Ok(choice) = input.trim().parse::<usize>() {
                    if choice >= 1 && choice <= servers.len() {
                        println!("[✅] {} seçildi", servers[choice - 1].address);
                        return Some(&servers[choice - 1]);
                    }
                }
                println!("[⚠️] Geçersiz seçim. 1-{} arası bir sayı girin.", servers.len());
            }
            Err(_) => {
                println!("[⚠️] Giriş hatası. Tekrar deneyin.");
            }
        }
    }
}

/// Otomatik sunucu seçer (kullanıcı etkileşimi olmadan)
pub fn select_best_server_auto(servers: &[DiscoveredServer]) -> Option<&DiscoveredServer> {
    if servers.is_empty() {
        return None;
    }
    
    // mDNS ile bulunanları tercih et, yoksa ilkini al
    let selected = servers.iter()
        .find(|s| matches!(s.discovery_method, DiscoveryMethod::MDns))
        .or_else(|| servers.first())?;
    
    if servers.len() > 1 {
        println!("[ℹ️] {} sunucu bulundu, otomatik olarak {} seçildi", servers.len(), selected.address);
        println!("[💡] Tüm sunucuları görmek için 'deltasafe discover' komutunu kullanın");
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