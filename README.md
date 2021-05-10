# xmpp-rs

`xmpp-rs` is an XMPP Server.

[![CI](https://github.com/Freyskeyd/xmpp-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/Freyskeyd/xmpp-rs/actions/workflows/ci.yml)
[![Upload Documentation](https://github.com/Freyskeyd/xmpp-rs/actions/workflows/update-doc.yml/badge.svg)](https://github.com/Freyskeyd/xmpp-rs/actions/workflows/update-doc.yml)
[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bhttps%3A%2F%2Fgithub.com%2FFreyskeyd%2Fxmpp-rs.svg?type=shield)](https://app.fossa.io/projects/git%2Bhttps%3A%2F%2Fgithub.com%2FFreyskeyd%2Fxmpp-rs?ref=badge_shield)
## Install

*Under construction*

## Build

To build xmpp-rs:

`cargo build --release`

## ROADMAP

- [ ] :rocket: Root implementation
    - [ ] **TCP Connection**: Able to accept TCP connection from clients
    - [ ] **Open stream reading**: Listen to an open `stream` stanza and respond to it
    - [ ] **TLS Connection and negociation**
    - [ ] **PLAIN authentication**: Authentification with a PLAIN mechanism must be possible.

## XEPs

- [ ] [RFC-6122: (XMPP): Address Format](https://tools.ietf.org/html/rfc6122)
- [ ] [RFC-7590: Use of TLS](https://tools.ietf.org/html/rfc7590)
- [ ] [XEP-0368: SRV records for XMPP over TLS](https://xmpp.org/extensions/xep-0368.html)
- [ ] [XEP-0199: XMPP Ping](https://xmpp.org/extensions/xep-0199.html)
- [ ] [XEP-0004: Data Forms](https://xmpp.org/extensions/xep-0004.html)
- [ ] [XEP-0030: Service Discovery](https://xmpp.org/extensions/xep-0030.html)
- [ ] [XEP-0048: Bookmarks](https://xmpp.org/extensions/xep-0048.html)
- [ ] [XEP-0049: Private XML Storage](https://xmpp.org/extensions/xep-0049.html)

- [ ] [Stanza errors](https://tools.ietf.org/html/rfc6120#section-8.3)

## License

xmpp-rs is primarily distributed under the terms of both the MIT license
and the Apache License (Version 2.0), with portions covered by various
BSD-like licenses.

See LICENSE-APACHE, and LICENSE-MIT for details.
