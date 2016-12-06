use std::io::{BufReader};
use std::net::TcpStream;

pub enum XmppSocket {
    Tcp(BufReader<TcpStream>, TcpStream),
    NoSock
}

impl XmppSocket {

}
