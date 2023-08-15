use std::io::Result;
use std::net::UdpSocket;

fn main() -> Result<()> {
    loop {
        let udp_sock = UdpSocket::bind("127.0.0.1:1111")?;
        let mut data = [0u8; u16::MAX as usize];
        let (len, from_addr) = udp_sock.recv_from(&mut data)?;
        let mut data = data[..len].to_vec();
        data.reverse();
        udp_sock.send_to(&data, from_addr)?;
    }
}
