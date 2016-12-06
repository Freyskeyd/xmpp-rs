extern crate xml;
use std::io;
use std::str;
use std::fmt;
use std::io::{BufReader};
use std::net::TcpStream;
use super::event::Event;
use super::socket::XmppSocket;
use super::utils::{XmppSend,ReadString};
use super::handler::XmppHandler;

pub struct XmppStreamStatus {
    pub connected: bool
}

#[derive(Debug)]
pub struct StreamEnd;

#[derive(Debug)]
pub struct StreamStart<'a> {
    pub to: &'a str
}

pub struct XmppStream {
    pub stream_status: XmppStreamStatus,
    pub parser: xml::Parser,
    pub builder: xml::ElementBuilder,
    pub handler: XmppHandler
}

impl XmppSend for StreamEnd {}

impl fmt::Display for StreamEnd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "</stream:stream>")
    }
}

impl<'a> fmt::Display for StreamStart<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<?xml version='1.0'?>\n\
               <stream:stream xmlns:stream='http://etherx.jabber.org/streams' xmlns='jabber:client' version='1.0' to='{}'>",  self.to)
    }
}

impl<'a> XmppSend for StreamStart<'a> {}

impl XmppStreamStatus {
    /// Return a boolean value that represent the connected state
    ///
    /// # Examples
    /// ```
    /// use xmpp::stream::XmppStreamStatus as XmppStatus;
    ///
    /// assert!(XmppStatus::new().connected());
    /// ```
    pub fn connected(&self) -> bool {
        self.connected
    }

    /// Return a new instance of XmppStreamStatus
    pub fn new() -> XmppStreamStatus {
        XmppStreamStatus {
            connected: true
        }
    }
}

impl XmppStream {

    /// Handle event stream
    pub fn handle_event(&mut self) -> Event {
        let builder = &mut self.builder;
        let handler = &mut self.handler;
        loop {
            let string =  match handler.socket.read_str() {
                Ok(s) => s,
                Err(_) => return Event::StreamClosed
            };

            self.parser.feed_str(&string);
            for event in &mut self.parser {
                match event {
                    Ok(xml::Event::ElementStart(xml::StartTag {
                        ref name,
                        ns: Some(ref ns),
                        ref prefix, ..
                    })) if *name == "stream" && *ns == "http://etherx.jabber.org/streams" => {
                        println!("In: Stream start");
                        match *prefix {
                            Some(ref prefix) => {
                                *builder = xml::ElementBuilder::new();
                                builder.set_default_ns("jabber:client".to_string());
                                builder.define_prefix(prefix.clone(), "http://etherx.jabber.org/streams".to_string());
                            }
                            None => {
                                *builder = xml::ElementBuilder::new();
                                builder.set_default_ns("http://etherx.jabber.org/streams".to_string());
                            }
                        }
                    }
                    event => match builder.handle_event(event) {
                        None => (),
                        Some(Ok(e)) => {
                            println!("In: {:#?}", e);
                        }

                        Some(Err(e)) => {
                            println!("{:#?}", e);
                            // Wait for remote to close stream
                            // TODO: Avoid waiting forever
                            continue;
                        }
                    }
                }
            }
        }
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
        let stream = {
            let addr: &str = &self.handler.domain[..];
            let port: &u16  = &self.handler.port;
            try!(TcpStream::connect(&(addr, *port)))
        };

        self.handler.socket = XmppSocket::Tcp(BufReader::new(try!(stream.try_clone())), stream);
        self.handler.start()
    }

    /// Create a new XmppStream
    ///
    /// # Examples
    /// ```
    /// use xmpp::stream::XmppStream;
    ///
    /// let connection = XmppStream::new("alice", "127.0.0.1", "try");
    /// ```
    pub fn new(username: &str, host: &str, pass: &str) -> XmppStream {
        XmppStream {
            stream_status: XmppStreamStatus::new(),
            parser: xml::Parser::new(),
            builder: xml::ElementBuilder::new(),
            handler: XmppHandler {
                username: username.to_string(),
                password: pass.to_string(),
                domain: host.to_string(),
                closed: true,
                port: 5222,
                socket: XmppSocket::NoSock
            }
        }
    }
}

