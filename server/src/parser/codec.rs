use crate::parser::sink::PacketSink;
use bytes::BufMut;
use circular::Buffer;
use log::trace;
use std::io::{self, Write};
use tokio_util::codec::{Decoder, Encoder};
use xml::ParserConfig;
use xmpp_proto::Packet;

/// XmppCodec deals with incoming bytes. You can feed the parser with bytes and try to detect new
/// event.
pub struct XmppCodec {
    pub sink: PacketSink,
}

impl XmppCodec {
    pub fn new() -> Self {
        Self { sink: Self::new_sink() }
    }

    fn new_sink() -> PacketSink {
        let mut cfg = ParserConfig::new().whitespace_to_characters(true);
        cfg.ignore_end_of_stream = true;
        PacketSink {
            parser: cfg.create_reader(Buffer::with_capacity(4096)),
        }
    }
}

impl Decoder for XmppCodec {
    type Item = Packet;

    type Error = io::Error;

    fn decode(&mut self, buf: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let _ = self.sink.parser.source_mut().write(&buf[..]);
        if !self.sink.parser.source().data().is_empty() {
            trace!("Buffer contains: {}", String::from_utf8_lossy(self.sink.parser.source().data()));
        }
        let event = match self.sink.next_packet() {
            Some(e) => {
                trace!("Decoded Packet: {:?}", e);
                Some(e)
            }
            _ => None,
        };
        let l = buf.len();
        let _ = buf.split_to(l);
        Ok(event)
    }
}

impl Encoder<Packet> for XmppCodec {
    type Error = io::Error;

    fn encode(&mut self, item: Packet, dst: &mut bytes::BytesMut) -> Result<(), Self::Error> {
        let _ = item.write_to_stream(dst.writer());

        Ok(())
    }
}
