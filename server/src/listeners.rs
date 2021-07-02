// TCP
pub(crate) mod tcp;
// WS
pub(crate) mod ws;

use actix_codec::AsyncRead;
use tokio::io::AsyncWrite;

pub trait XmppStream: AsyncRead + AsyncWrite + Unpin + Send {}
pub struct XmppStreamHolder {
    inner: Box<dyn XmppStream>,
}
