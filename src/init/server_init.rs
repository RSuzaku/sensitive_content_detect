use std::net::{IpAddr, Ipv4Addr};

use crate::{
    init::app_init::CONFIG,
    NewResult
};

pub async fn port_init() -> NewResult<u16> {

    let config = CONFIG.lock().await;

    let port_opt = config.get("port");

    match port_opt {
        Some(port_str) => {
            let server_port: Result<u16, _>= port_str.to_string().parse();
            match server_port {
                Ok(port) => {
                    Ok(port)
                },
                Err(_) => {
                    Ok(8080)
                }
            }
        },
        None => {
            Ok(8080)
        }
    }
}

// host 初始化
pub async fn host_init() -> NewResult<Vec<u32>> {
    let config = CONFIG.lock().await;

    let host_opt = config.get("bind_ip");

    match host_opt {
        Some(host_str) => {
            let host_array: Result<Vec<u32>, _> = host_str
                .split('.')
                .map(|s| s.parse::<u32>())
                .collect();

            match host_array {
                Ok(host) => {
                    Ok(host)
                },
                Err(_) => {
                    Ok(vec![127,0,0,1])
                }
            }
        },
        None => {
            Ok(vec![127,0,0,1])
        }
    }
}

// 服务初始化
pub async fn server_init() -> NewResult<(IpAddr, u16)> {
    let host = host_init().await.unwrap();
    // 将 host 转换为 IpAddr
    let ip_addr = IpAddr::V4(Ipv4Addr::new(
        host[0] as u8,
        host[1] as u8,
        host[2] as u8,
        host[3] as u8,
    ));
    Ok((ip_addr, port_init().await.unwrap()))
}