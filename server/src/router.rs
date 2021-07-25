use actix::{Actor, Context, Handler, Supervised, SystemService};
use log::trace;
use xmpp_proto::{FromXmlElement, GenericIq, Packet, Stanza};
use xmpp_xml::Element;

use crate::messages::{
    system::{SessionCommand, SessionCommandAction},
    StanzaEnvelope,
};

/// Manage to route packet on a node
#[derive(Default, Debug)]
pub struct Router {}

impl Router {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl Supervised for Router {}
impl SystemService for Router {}

impl Actor for Router {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        trace!("Router started");
    }
}

impl Handler<StanzaEnvelope> for Router {
    type Result = ();

    fn handle(&mut self, msg: StanzaEnvelope, ctx: &mut Self::Context) -> Self::Result {
        match msg.stanza {
            Stanza::IQ(iq) => {
                let e = if iq.get_id() == "roster" {
                    Element::from_reader(
                        format!(
                            "
                                <iq id='{}'
                                    to='{}'
                                    type='result'>
                                    <query xmlns='jabber:iq:roster' ver='ver9'/>
                                </iq>
                            ",
                            iq.get_id(),
                            "local@localhost"
                        )
                        .as_bytes(),
                    )
                    .unwrap()
                } else {
                    Element::from_reader(
                        format!(
                            r#"
                <iq xmlns="jabber:client"
                    type="error"
                    id="{}"
                    from="{}"
                    to="{}">
                    {}
                    <error code="503"
                        type="cancel">
                        <service-unavailable xmlns="urn:ietf:params:xml:ns:xmpp-stanzas"/>
                    </error>
                </iq>
                "#,
                            iq.get_id(),
                            "localhost",
                            "local@localhost",
                            match iq.get_element().unwrap().get_child(1) {
                                Some(e) => e.to_string().unwrap(),
                                None => String::new(),
                            }
                        )
                        .as_bytes(),
                    )
                    .unwrap()
                };
                let x = GenericIq::from_element(&e).unwrap();
                let response = Packet::Stanza(Box::new(Stanza::IQ(x)));

                let _ = msg.from.addr_session_command.unwrap().try_send(SessionCommand(SessionCommandAction::SendPacket(response)));
            }
            Stanza::Message(message) => {}
            Stanza::Presence(presence) => {}
        }

        ()
    }
}
// /// Manage to route packet when server is the target
// pub struct LocalRouter {}
// /// Manage to route packet based on pattern
// pub struct RegisteredRouteManager {}
