pub use super::{XmppStreamStatus};

pub struct XmppStream {
    streamStatus: XmppStreamStatus
}

impl XmppStream {
    pub fn disconnect(&self) -> &XmppStreamStatus {
        &self.streamStatus
    }

    pub fn connect(&self) -> &XmppStreamStatus {
        &self.streamStatus
    }

    pub fn new(username: &str, host: &str, pass: &str) -> XmppStream {
        XmppStream {
            streamStatus: XmppStreamStatus::new()
        }
    }
}
