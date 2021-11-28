use netstat2::{
    get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo, SocketInfo,
};

use std::collections::HashMap;
use sysinfo::{ProcessExt, System, SystemExt};

pub fn show_connections(with_process_info: bool) {
    match with_process_info {
        true => println!("Show network connections with process info:"),
        false => println!("Show network connections:"),
    }
    let processes_by_id = get_processes_by_id();
    let print_process_info = move |pid| {
        println!(
            "\t Process {}: {}",
            pid,
            processes_by_id
                .get(&pid)
                .unwrap_or(&"Not found".to_string())
        )
    };

    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;
    match get_sockets_info(af_flags, proto_flags) {
        Ok(sockets_info) => show_result(sockets_info, print_process_info),
        Err(e) => println!("Can't access information about sockets. Error: {:?}", e),
    }
    println!("End");
}

fn show_result(sockets_info: Vec<SocketInfo>, print_process_info: impl Fn(u32)) {
    for si in sockets_info {
        match si.protocol_socket_info {
            ProtocolSocketInfo::Tcp(tcp_si) => {
                println!(
                    "TCP {}:{} -> {}:{} {:?} - {}",
                    tcp_si.local_addr,
                    tcp_si.local_port,
                    tcp_si.remote_addr,
                    tcp_si.remote_port,
                    si.associated_pids,
                    tcp_si.state
                );
                for associated_pid in si.associated_pids {
                    print_process_info(associated_pid);
                }
            }
            ProtocolSocketInfo::Udp(udp_si) => {
                println!(
                    "UDP {}:{} -> *:* {:?}",
                    udp_si.local_addr, udp_si.local_port, si.associated_pids
                );
                for associated_pid in si.associated_pids {
                    print_process_info(associated_pid);
                }
            }
        }
    }
}

fn get_processes_by_id() -> HashMap<u32, String> {
    let mut sys = System::new_all();
    sys.refresh_all();
    sys.processes()
        .keys()
        .map(|pid| -> u32 { *pid as u32 })
        .into_iter()
        .zip(
            sys.processes()
                .values()
                .map(|process| process.exe().to_str().unwrap_or("").to_string()),
        )
        .collect()
}
