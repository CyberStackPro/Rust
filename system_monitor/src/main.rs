use pnet::datalink::Channel::Ethernet;
use pnet::datalink::{self, NetworkInterface};
use pnet::packet::arp::{ArpHardwareTypes, ArpOperation, ArpPacket, MutableArpPacket};
use pnet::packet::ethernet::{EtherTypes, MutableEthernetPacket};
use pnet::packet::{MutablePacket, Packet};
use pnet::util::MacAddr;
use std::env;
use std::net::{IpAddr, Ipv4Addr};
use std::process::Command;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: {} <ip_range> <interface>", args[0]);
        println!("Example: {} 192.168.1.0/24 eth0", args[0]);
        return;
    }

    let range = &args[1];
    let interface_name = &args[2];

    // Parse CIDR notation
    let parts: Vec<&str> = range.split('/').collect();
    if parts.len() != 2 {
        println!("[-] Invalid CIDR notation. Example: 192.168.1.0/24");
        return;
    }

    let base_ip = parts[0];
    let mask: u8 = match parts[1].parse() {
        Ok(m) => m,
        Err(_) => {
            println!("[-] Invalid subnet mask");
            return;
        }
    };

    if mask != 24 {
        println!("[-] This simple scanner only supports /24 networks");
        return;
    }

    // Get network interface
    let interfaces = datalink::interfaces();
    let interface = match interfaces
        .iter()
        .find(|iface| iface.name == *interface_name)
    {
        Some(i) => i.clone(),
        None => {
            println!("[-] Interface {} not found", interface_name);
            return;
        }
    };

    println!("[+] Scanning network range: {}", range);

    // Extract network base for /24
    let ip_parts: Vec<&str> = base_ip.split('.').collect();
    if ip_parts.len() != 4 {
        println!("[-] Invalid IP format");
        return;
    }

    let network_base = format!("{}.{}.{}", ip_parts[0], ip_parts[1], ip_parts[2]);

    println!("\nDiscovered devices:");
    println!("IP Address\t\tMAC Address");
    println!("----------------------------------------");

    // Create a channel to send/receive packets
    let (mut tx, _) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => {
            println!("[-] Unhandled channel type");
            return;
        }
        Err(e) => {
            println!("[-] Error creating datalink channel: {}", e);
            return;
        }
    };

    // Get source MAC address
    let source_mac = match interface.mac {
        Some(mac) => mac,
        None => {
            println!("[-] Could not get MAC address for interface");
            return;
        }
    };

    // Get source IP address
    let source_ip = match interface.ips.iter().find(|ip| ip.is_ipv4()) {
        Some(ip) => match ip.ip() {
            IpAddr::V4(ipv4) => ipv4,
            _ => {
                println!("[-] Could not get IPv4 address for interface");
                return;
            }
        },
        None => {
            println!("[-] No IPv4 address found for interface");
            return;
        }
    };

    // Scan range
    for i in 1..255 {
        let target_ip_str = format!("{}.{}", network_base, i);
        let target_ip = match Ipv4Addr::from_str(&target_ip_str) {
            Ok(ip) => ip,
            Err(_) => continue,
        };

        // Send an ARP request
        let mut ethernet_buffer = [0u8; 42]; // Ethernet + ARP
        let mut ethernet_packet = MutableEthernetPacket::new(&mut ethernet_buffer).unwrap();

        ethernet_packet.set_destination(MacAddr::broadcast());
        ethernet_packet.set_source(source_mac);
        ethernet_packet.set_ethertype(EtherTypes::Arp);

        let mut arp_buffer = [0u8; 28];
        let mut arp_packet = MutableArpPacket::new(&mut arp_buffer).unwrap();

        arp_packet.set_hardware_type(ArpHardwareTypes::Ethernet);
        arp_packet.set_protocol_type(EtherTypes::Ipv4);
        arp_packet.set_hw_addr_len(6);
        arp_packet.set_proto_addr_len(4);
        arp_packet.set_operation(ArpOperation::new(1));
        arp_packet.set_sender_hw_addr(source_mac);
        arp_packet.set_sender_proto_addr(source_ip);
        arp_packet.set_target_hw_addr(MacAddr::zero());
        arp_packet.set_target_proto_addr(target_ip);

        ethernet_packet.set_payload(arp_packet.packet());

        match tx.send_to(ethernet_packet.packet(), None) {
            Some(_) => {}
            None => {
                println!("[-] Error sending ARP request to {}", target_ip_str);
                continue;
            }
        }

        // We need to use an external command to check the ARP table
        // since we're not setting up a proper receiver in this example
        thread::sleep(Duration::from_millis(10));
    }

    // Give time for ARP responses to populate the ARP cache
    thread::sleep(Duration::from_secs(1));

    // Check ARP table
    let output = Command::new("arp")
        .arg("-a")
        .output()
        .expect("Failed to execute ARP command");

    let arp_table = String::from_utf8_lossy(&output.stdout);

    // Filter and display results for our network
    for line in arp_table.lines() {
        if line.contains(&network_base) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let ip = parts[1].trim_matches(|c| c == '(' || c == ')');
                let mac = parts[3];
                if mac != "<incomplete>" {
                    println!("{}\t\t{}", ip, mac);
                }
            }
        }
    }
}

// extern crate glib;
// extern crate gtk;
// extern crate sysinfo;

// mod system_info;
// use glib::{ControlFlow, clone};
// use gtk::prelude::*;
// use gtk::{Application, ApplicationWindow, Box, Label};
// use std::cell::RefCell;
// use std::rc::Rc;
// use system_info::SystemInfo;

// fn build_ui(application: &gtk::Application) {
//     let window = ApplicationWindow::new(application);
//     window.set_title("System Monitor");
//     window.set_default_size(350, 70);
//     let vbox = Box::new(gtk::Orientation::Vertical, 10);
//     let cpu_label = Label::new(Some("CPU Usage:"));
//     let memory_label = Label::new(Some("Memory Usage:"));
//     vbox.add(&cpu_label);
//     vbox.add(&memory_label);
//     window.add(&vbox);
//     window.show_all();
//     let system_info = Rc::new(RefCell::new(SystemInfo::new()));
//     glib::timeout_add_seconds_local(
//         1,
//         clone!(@strong system_info, @strong cpu_label, @strong memory_label => move || {
//             let system_info = system_info.borrow();
//             cpu_label.set_text(&format!("CPU Usage: {:.2}%", system_info.get_cpu_usage()));
//             let (used_memory, total_memory) = system_info.get_memory_usage();
//             memory_label.set_text(&format!("Memory Usage: {} / {}", used_memory, total_memory));
//             ControlFlow::Continue
//         }),
//     );
// }

// fn main() {
//     let application = Application::new(Some("com.example.system_monitor"), Default::default());
//     application.connect_activate(build_ui);
//     application.run();
// }
