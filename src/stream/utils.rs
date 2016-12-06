use std::io::BufRead;
use std::io;
use std::str;
use std::fmt;

pub trait ReadString {
    fn read_str(&mut self) -> io::Result<String>;
}

pub trait XmppSend: fmt::Display {
    fn xmpp_send<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        write!(w, "{}", self)
    }
}

impl<T: BufRead> ReadString for T {
    fn read_str(&mut self) -> io::Result<String> {
        let available = try!(self.fill_buf());
        let res = str::from_utf8(&available[..]);
        res.map(|x| x.to_string())
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData,
                                            "stream did not contain valid UTF-8"))
    }
}
