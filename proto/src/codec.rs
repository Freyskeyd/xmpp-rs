use bytes::BytesMut;
use std::str;
use std::io;
use tokio_io::codec::{Encoder, Decoder};
use xmpp_events::ToXmlElement;
use xmpp_events::Event;
use xmpp_events::Event::*;
use xmpp_events::NonStanzaEvent::*;
// use events::StanzaEvent::*;
// use events::IqEvent::*;
use parser::XmppParser;
use xmpp_xml::WriteOptions;
// use xmpp_xml::XmlProlog;

/// A codec that will transform I/O into Event
pub struct XMPPCodec {
    /// The parser hold our buffer and try to extract Event
    pub parser: XmppParser,
}

impl XMPPCodec {
    /// Return a new XMPPCodec with an initialized buffer inside
    pub fn new() -> XMPPCodec {
        XMPPCodec { parser: XmppParser::new() }
    }
}

impl Default for XMPPCodec {
    fn default() -> Self {
        Self::new()
    }
}

impl Decoder for XMPPCodec {
    type Item = Event;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, io::Error> {

        trace!("");
        trace!("==================================================");
        self.parser.feed(&buf[..]);

        trace!("Buffer contains: {}",
               str::from_utf8(self.parser.source().data()).unwrap());
        trace!("");
        let event = match self.parser.next_event() {
            Some(e) => {
                trace!("Decode: event: {:?}", e);
                Some(e)
            }
            _ => None,
        };
        let l = buf.len();
        buf.split_to(l);
        Ok(event)
    }
}

impl Encoder for XMPPCodec {
    type Item = Event;
    type Error = io::Error;

    fn encode(&mut self, frame: Event, buf: &mut BytesMut) -> Result<(), Self::Error> {
        trace!("will send frame: {:?}", frame);

        let f = match frame {
            Stanza(stanza) => {
                if let Ok(root) = stanza.to_element() {
                    let mut out: Vec<u8> = Vec::new();
                    let options = WriteOptions::new().set_xml_prolog(None);

                    root.to_writer_with_options(&mut out, options).unwrap();
                    String::from_utf8(out).unwrap()
                } else {
                    String::new()
                }
            }
            NonStanza(non_stanza) => {
                match *non_stanza {
                    AuthEvent(event) => {
                        if let Ok(root) = event.to_element() {
                            let mut out: Vec<u8> = Vec::new();
                            let options = WriteOptions::new().set_xml_prolog(None);

                            root.to_writer_with_options(&mut out, options).unwrap();
                            String::from_utf8(out).unwrap()
                        } else {
                            String::new()
                        }
                    }
                    StartTlsEvent(event) => {
                        if let Ok(root) = event.to_element() {
                            let mut out: Vec<u8> = Vec::new();
                            let options = WriteOptions::new().set_xml_prolog(None);

                            root.to_writer_with_options(&mut out, options).unwrap();
                            String::from_utf8(out).unwrap()
                        } else {
                            String::new()
                        }
                    }
                    CloseStreamEvent => String::from("</stream:stream>"),
                    OpenStreamEvent(event) => {
                        if let Ok(root) = event.to_element() {
                            let mut out: Vec<u8> = Vec::new();
                            // let options = WriteOptions::new().set_xml_prolog(Some(XmlProlog::Version10));
                            let options = WriteOptions::new().set_xml_prolog(None);

                            root.to_writer_with_options(&mut out, options).unwrap();

                            // TODO: remove ugly, deal with />
                            if out.ends_with(&[32, 47, 62]) {
                                let len = out.len();
                                out.remove(len - 2);
                                let len = out.len();
                                out.remove(len - 2);
                            }
                            String::from_utf8(out).unwrap()
                        } else {
                            String::new()
                        }
                    }
                    ProceedTlsEvent(_) |
                    SuccessTlsEvent(_) => String::new(),
                    StreamFeaturesEvent(event) => {
                        if let Ok(root) = event.to_element() {
                            let mut out: Vec<u8> = Vec::new();
                            let options = WriteOptions::new().set_xml_prolog(None);

                            root.to_writer_with_options(&mut out, options).unwrap();
                            String::from_utf8(out).unwrap()
                        } else {
                            String::new()
                        }
                    }
                }
            }
        };

        buf.extend(f.as_bytes());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use xmpp_events::*;
    use xmpp_config::{XMPPConfig};
    use bytes::BytesMut;

    #[test]
    fn decode_open_stream() {
        let mut codec = XMPPCodec::new();

        let _ = OpenStream::new(&XMPPConfig::new());

        let mut buffer = BytesMut::with_capacity(64);
        buffer.extend("<?xml version=\'1.0\'?><stream:stream version=\'1.0\' \
                      xmlns:stream=\'http://etherx.jabber.org/streams\' \
                      to=\'example.com\' xmlns=\'jabber:client\'>"
                              .as_bytes());

        match codec.decode(&mut buffer) {
            Ok(x) => {
                let event = x.unwrap();
                assert!(event.is_non_stanza());
                assert!(match event {
                            NonStanza(x) => {
                                match *x {
                                    OpenStreamEvent(_) => true,
                                    _ => false,
                                }
                            }
                            _ => false,
                        });
            }
            _ => {}
        };
    }

    #[test]
    fn encode_open_stream() {
        let mut codec = XMPPCodec::new();

        let e = OpenStream::new(&XMPPConfig::new());

        let mut buffer = BytesMut::with_capacity(64);

        match codec.encode(e.to_event(), &mut buffer) {
            Ok(_) => {
                let x = buffer.clone();

                let t = "<stream:stream \
                         xmlns=\"jabber:client\" \
                         xmlns:stream=\"http://etherx.jabber.org/streams\" \
                         to=\"example.com\" version=\"1.0\">";
                assert!(str::from_utf8(&x[..]).unwrap() == t,
                        format!("{} == {}", str::from_utf8(&x[..]).unwrap(), t));
            }
            _ => {}
        };
    }

    #[test]
    fn encode_close_stream() {
        let mut codec = XMPPCodec::new();

        let e = CloseStream::new();

        let mut buffer = BytesMut::with_capacity(64);

        match codec.encode(e.to_event(), &mut buffer) {
            Ok(_) => assert!(&buffer[..] == b"</stream:stream>"),
            _ => {}
        };
    }
}
