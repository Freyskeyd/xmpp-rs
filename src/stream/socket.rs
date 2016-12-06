use std::io;
use std::net::TcpStream;
use std::io::{Write, BufReader};
use super::utils::{ReadString};

pub enum XmppSocket {
    Tcp(BufReader<TcpStream>, TcpStream),
    NoSock
}

impl Write for XmppSocket {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match *self {
            XmppSocket::Tcp(_, ref mut stream) => stream.write(buf),
            XmppSocket::NoSock => panic!("No socket yet")
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match *self {
            XmppSocket::Tcp(_, ref mut stream) => stream.flush(),
            XmppSocket::NoSock => panic!("No socket yet")
        }
    }
}

impl ReadString for XmppSocket {
    fn read_str(&mut self) -> io::Result<String> {
        match *self {
            XmppSocket::Tcp(ref mut stream, _) => stream.read_str(),
            XmppSocket::NoSock => panic!("Tried to read string before socket exists")
        }
    }
}
