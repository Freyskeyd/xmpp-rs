use std::io;
use super::stream::{StreamStart};
use super::utils::{XmppSend};
use std::io::{Write};
use super::socket::XmppSocket;

pub struct XmppHandler {
    pub domain: String,
    pub username: String,
    pub password: String,
    pub closed: bool,
    pub port: u16,
    pub socket: XmppSocket
}

impl XmppHandler {
    pub fn start(&mut self) -> io::Result<()> {
        let stream_start = StreamStart { to: "example.com" };
        println!("Out: {}", stream_start);
        try!(stream_start.xmpp_send(&mut self.socket));
        self.socket.flush()
    }
}
