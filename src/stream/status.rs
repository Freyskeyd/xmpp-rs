pub struct XmppStreamStatus {
    connected: bool
}

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
    ///
    /// # Examples
    /// ```
    /// use xmpp::stream::XmppStreamStatus as XmppStatus;
    ///
    /// let status: XmppStatus = XmppStatus::new();
    /// ```
    pub fn new() -> XmppStreamStatus {
        XmppStreamStatus {
            connected: true
        }
    }
}

