#![deny(warnings)]
//! xmpp-rs is an implementation of the Extensible Messaging and Presence Protocol (XMPP).
//! Based on tokio-rs and futures-rs. It's goal is to be fully tested and usable.
//!
//! It allow you to create a client to talk with any XMPP server or to use the proto lib to make
//! your own plugins/component.
//!
//! This implementation focus is to be usable and tested.
//!
/// Reexport of XMPPClient
pub extern crate xmpp_client as client;
/// Reexport of XMPPProto
pub extern crate xmpp_proto as proto;
