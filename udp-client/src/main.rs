use std::io::Result;
use std::net::UdpSocket;

fn main() -> Result<()> {
    let udp_sock = UdpSocket::bind("0.0.0.0:0")?;
    let mut data = Vec::<u8>::new();
    data.extend_from_slice(&[1, 2]);
    data = data.repeat(751);
    udp_sock.connect("127.0.0.1:1111")?;
    udp_sock.send(&data)?;
    let mut back_data = [0u8; u16::MAX as usize];
    let len = udp_sock.recv(&mut back_data)?;
    let back_data = &back_data[..len];
    println!("back_data >>> {:?}", back_data);
    let back_data_len = back_data.len();
    println!("back_data_len >>> {}", back_data_len);
    Ok(())
}
