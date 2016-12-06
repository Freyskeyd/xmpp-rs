use std::net::TcpStream;
use std::io;
use std::io::{BufReader};

pub use super::{XmppStreamStatus, XmppSocket};

use super::handler::{XmppHandler};

pub struct XmppStream {
    pub streamStatus: XmppStreamStatus,
    handler: XmppHandler
}

impl XmppStream {
    pub fn disconnect(&self) -> &XmppStreamStatus {
        &self.streamStatus
    }

    /// Try to connect over TCP
    ///
    /// # Examples
    /// ```
    /// use xmpp::stream::XmppStream;
    ///
    /// let mut connection = XmppStream::new("alice", "127.0.0.1", "try");
    /// match connection.connect() {
    ///     Ok(_) => assert!(true, "connection etablished"),
    ///     Err(e) => assert!(false, e)
    /// };
    /// ```
    pub fn connect(&mut self) -> io::Result<()> {
        let addr: &str = &self.handler.domain[..];
        let port: &u16  = &self.handler.port;
        let stream = try!(TcpStream::connect(&(addr, *port)));

        self.handler.socket = XmppSocket::Tcp(BufReader::new(try!(stream.try_clone())), stream);

        Ok(())
    }

    pub fn new(username: &str, host: &str, pass: &str) -> XmppStream {
        XmppStream {
            streamStatus: XmppStreamStatus::new(),
            handler: XmppHandler {
                domain: host.to_string(),
                port: 5222,
                socket: XmppSocket::NoSock
            }
        }
    }
}
