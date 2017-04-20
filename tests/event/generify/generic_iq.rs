use std::str::FromStr;
use xmpp_proto::events::PresenceType;
use xmpp_proto::{Jid};
use xmpp_proto::events::GenericIq;
use xmpp_proto::events::IqType;

#[test]
fn create_a_generic_iq() {
    let mut g = GenericIq::new("123456", IqType::Get);

    // GenericIq should have an ID
    // The 'id' attribute is REQUIRED for IQ stanzas.
    g.set_id("12345");

    assert_eq!(g.get_id(), "12345");

    // The 'type' attribute is REQUIRED for IQ stanzas. The value MUST be one of the following:
    //    get -- The stanza is a request for information or requirements.
    //    set -- The stanza provides required data, sets new values, or replaces existing values.
    //    result -- The stanza is a response to a successful get or set request.
    //    error -- An error has occurred regarding processing or delivery of a previously-sent get or set (see Stanza Errors).
    g.set_type(IqType::Get);

    // An entity that receives an IQ request of type "get" or "set" MUST reply with an IQ response of type "result" or "error" (the response MUST preserve the 'id' attribute of the request).
    // An entity that receives a stanza of type "result" or "error" MUST NOT respond to the stanza by sending a further IQ response of type "result" or "error"; however, as shown above, the requesting entity MAY send another request (e.g., an IQ of type "set" in order to provide required information discovered through a get/result pair).
    // An IQ stanza of type "get" or "set" MUST contain one and only one child element that specifies the semantics of the particular request or response.
    // An IQ stanza of type "result" MUST include zero or one child elements.
    // An IQ stanza of type "error" SHOULD include the child element contained in the associated "get" or "set" and MUST include an <error/> child; for details, see Stanza Errors.
}

#[test]
fn create_bind() {
    let mut g = GenericIq::new("12345", IqType::Set);

    assert_eq!(g.to_string(), "<iq id=\"12345\" type=\"set\" />");
}
