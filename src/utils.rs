extern crate pnet;
extern crate ipnetwork;

use anyhow::{anyhow, Result};
use pnet::datalink;
use ipnetwork::Ipv6Network;

/// 获取本机的公网 IP
/// https://4.ipw.cn => 192.168.1.1
/// https://test.ipw.cn => 192.168.1.1
/// https://ipinfo.io/ip => 192.168.1.1
/// https://myip.biturl.top => 192.168.1.1
/// https://ipecho.net/plain => 192.168.1.1 (如果有 IPv6 地址的话, 则返回 IPv6)
/// https://httpbin.org/ip => {"origin": "192.168.1.1"}
/// http://6.ipw.cn => 有 IPv6 地址则返回, 否则出错
/*
pub fn get_public_ip() -> Result<String> {
    const URLS: &[&str] = &[
        "https://4.ipw.cn",
        "https://6.ipw.cn",
        "https://test.ipw.cn",
    ];
    let i = rand::random::<usize>() % URLS.len();
    let url = URLS[i];
    let res = reqwest::blocking::get(url)?;
    if !res.status().is_success() {
        return Err(anyhow!("HTTP status: {}", res.status()));
    }
    let ip = res.text()?;
    Ok(ip)
}
*/

pub fn get_ip(intf: &str, recordtype: &str) -> Result<String> {
    for iface in datalink::interfaces() {
        if iface.name == intf {
            // println!("{:?}", iface.ips);
            for ip in iface.ips {
                if ip.is_ipv4() && recordtype == "A" {
                    // println!("IPV4: {:?}", ip.ip())
                    return Ok(ip.ip().to_string());
                } else if ip.is_ipv6() && recordtype == "AAAA" {
                    // println!("IPV6: {:?}", ip.ip())
                    let ll_net: Ipv6Network = "fe80::/16".parse().unwrap();
                    let ipstr = ip.ip().to_string();
                    if ! ll_net.contains(ipstr.parse().unwrap()) {
                        return Ok(ipstr);
                    }
                }
            }
        }
    }
    Err(anyhow!("ERROR"))
}
