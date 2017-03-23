#[derive(Debug, Clone)]
pub struct Handshake {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct ClientMessage(pub String);

#[derive(Debug)]
pub struct ServerMessage(pub String);
