# xmpp-rs

`xmpp-rs` is an XMPP client library (for now). Under active development.

[![Build Status](https://travis-ci.org/Freyskeyd/xmpp-rs.svg?branch=master)](https://travis-ci.org/Freyskeyd/xmpp-rs)

## Install

This library is splitted in 3 crates (`client`, `server`, `proto`). As defined by the naming, you can use every crate
independently.

- `xmpp-client`: will allow you to build a client.
- `xmpp-server`: is an implementation of XMPP on server side.
- `xmpp-rs`: Grab both client and server in a single crate.

**Cargo.toml**

```toml
xmpp-rs = "0.1"
```

```rust
// Client
extern crate xmpp;

```
## Build

To build xmpp-rs:

`cargo build --release`

## ROADMAP

### Client roadmap

- [ ] :rocket: Root implementation
    - [ ] **TCP Connection**: Base of all the interaction, we need to provide a way to connect to different kind of
      servers.
        - [x] Can connect to IP
        - [ ] Can connect to domain
        - [ ] Can connect to untrusted domain
    - [x] **TLS Connection and negociation**: TLS connection is mandatory.
    - [ ] **PLAIN authentication**: Authentification with a PLAIN mechanism must be possible.
    - [ ] **Ping IQ client**: We need to be able to send a ping to the server and listen for the anwser.
    - [ ] **Send First presence**: We need to be able to send our presence to the server.
- [ ] :satellite: Components
    - [ ] **SASL**: Handle every needed SASL auth mechanisms
- [ ] :electric_plug: Plugins
    - [ ] **Message**: Handle all incomming/outcomming user or server message.
    - [ ] **IQ**: Handle all IQ requests/responses.
    - [ ] **Presence**: Offer a way to manage user's presence.
    - [ ] **MUC**: Activation option to deal with groupchat.
    - [ ] **MAM**: Activation option to retrieve history.

### Server roadmap

- [ ] :rocket: Root implementation
    - [ ] **TCP Connection**: Able to accept TCP connection from clients
    - [ ] **Open stream reading**: Listen to an open `stream` stanza and respond to it

## XEP

- [ ] [XEP-0199: XMPP Ping](https://xmpp.org/extensions/xep-0199.html)

## License

xmpp-rs is primarily distributed under the terms of both the MIT license
and the Apache License (Version 2.0), with portions covered by various
BSD-like licenses.

See LICENSE-APACHE, and LICENSE-MIT for details.
