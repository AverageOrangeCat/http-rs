use std::net::TcpStream;
use super::result;

pub type Call = fn(&TcpStream) -> result::Result<()>;
