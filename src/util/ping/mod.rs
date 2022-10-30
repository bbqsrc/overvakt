mod errors;
mod packet;

use std::io::Read;
use std::net::{IpAddr, SocketAddr};
use std::time::Duration;

use rand::random;
pub use socket2::Type;
use socket2::{Domain, Protocol, Socket};

use errors::Error;
use packet::{EchoReply, EchoRequest, IcmpV4, IcmpV6, IpV4Packet, ICMP_HEADER_SIZE};

const TOKEN_SIZE: usize = 24;
const ECHO_REQUEST_BUFFER_SIZE: usize = ICMP_HEADER_SIZE + TOKEN_SIZE;
type Token = [u8; TOKEN_SIZE];

pub fn ping(
    addr: IpAddr,
    timeout: Option<Duration>,
    ttl: Option<u32>,
    ident: Option<u16>,
    seq_cnt: Option<u16>,
    payload: Option<&Token>,
    socket_type: Type,
) -> Result<(), Error> {
    let timeout = match timeout {
        Some(timeout) => Some(timeout),
        None => Some(Duration::from_secs(4)),
    };

    let dest = SocketAddr::new(addr, 0);
    let mut buffer = [0; ECHO_REQUEST_BUFFER_SIZE];

    let default_payload: &Token = &random();

    let request = EchoRequest {
        ident: ident.unwrap_or_else(random),
        seq_cnt: seq_cnt.unwrap_or(1),
        payload: payload.unwrap_or(default_payload),
    };

    let mut socket = if dest.is_ipv4() {
        if request.encode::<IcmpV4>(&mut buffer[..]).is_err() {
            return Err(Error::InternalError);
        }
        Socket::new(Domain::IPV4, socket_type, Some(Protocol::ICMPV4))?
    } else {
        if request.encode::<IcmpV6>(&mut buffer[..]).is_err() {
            return Err(Error::InternalError);
        }
        Socket::new(Domain::IPV6, socket_type, Some(Protocol::ICMPV6))?
    };

    socket.set_ttl(ttl.unwrap_or(64))?;
    socket.set_write_timeout(timeout)?;
    socket.send_to(&buffer, &dest.into())?;
    socket.set_read_timeout(timeout)?;

    let mut buffer: [u8; 2048] = [0; 2048];
    socket.read_exact(&mut buffer)?;

    let _reply = if dest.is_ipv4() {
        let ipv4_packet = match IpV4Packet::decode(&buffer) {
            Ok(packet) => packet,
            Err(_) => return Err(Error::InternalError),
        };
        match EchoReply::decode::<IcmpV4>(ipv4_packet.data) {
            Ok(reply) => reply,
            Err(_) => return Err(Error::InternalError),
        }
    } else {
        match EchoReply::decode::<IcmpV6>(&buffer) {
            Ok(reply) => reply,
            Err(_) => return Err(Error::InternalError),
        }
    };

    Ok(())
}
