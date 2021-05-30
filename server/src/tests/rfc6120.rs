use std::{error::Error, io, path::PathBuf};

use crate::{
    sessions::{manager::SessionManager, state::SessionState, SessionManagementPacket, SessionManagementPacketResult},
    Server,
};
use bytes::BytesMut;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::mpsc::{self, Receiver, Sender},
};
use uuid::Uuid;
use xmpp_proto::{NonStanza, Packet};
use xmpp_proto::{OpenStream, OpenStreamBuilder};

#[tokio::test]
async fn should_return_an_open_stream() {
    let handler = SessionManager::default();

    let (referer, mut rx): (Sender<SessionManagementPacketResult>, Receiver<SessionManagementPacketResult>) = mpsc::channel(32);
    let response = handler.handle_packet(SessionManagementPacket {
        session_state: SessionState::Opening,
        packet: OpenStreamBuilder::default().lang("en").version("1.0").id(Uuid::new_v4()).build().unwrap().into(),
        referer,
    });

    assert!(response.is_ok());

    if let Some(result) = rx.recv().await {
        assert_eq!(result.session_state, SessionState::Opening);
        assert!(
            matches!(result.packets.as_slice(), [Packet::NonStanza(open_stream), Packet::NonStanza(features)] if matches!(**open_stream, NonStanza::OpenStream(_))
                && matches!(**features, NonStanza::StreamFeatures(_)))
        );
    } else {
        panic!("Should have respond something");
    }
}

#[actix::test]
async fn should_return_an_open_stream_2() -> Result<(), Box<dyn Error>> {
    actix_rt::spawn(async move {
        let _ = Server::build().cert("./src/tests/fixtures/server.crt").keys("./src/tests/fixtures/server.key").launch().await;
    });

    std::thread::sleep_ms(100);
    let mut expected = String::new();

    let mut stream = tokio::net::TcpStream::connect("localhost:5222").await?;

    stream.write_all(b"hello world!").await?;
    expected.push_str("SENT:\nhello world!");

    let mut buf = BytesMut::with_capacity(4096);

    loop {
        stream.readable().await.unwrap();

        match stream.read_buf(&mut buf).await {
            Ok(0) => break,
            Ok(size) => {
                let readed = buf.split_to(size);
                println!("{:?}", readed);
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                println!("err: {:?}", e);
                break;
            }
        }
    }

    Ok(())
}
