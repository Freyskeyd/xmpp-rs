# PROTOCOL

- [ ] [RFC-6120](https://datatracker.ietf.org/doc/rfc6120/)
<details>
<summary>
Implementation details
</summary>

```
C: = a client
E: = any XMPP entity
I: = an initiating entity
P: = a peer server
R: = a receiving entity
S: = a server
S1: = server1
S2: = server2
```

- [x] TCP Binding
- [ ] XML Streams
    - [x] Opening a Stream
    - [ ] Stream Negotiation
        - [x] Restarts
        - [x] Resending Features
        - [ ] Completion of Stream Negotiation
        - [ ] Determination of Addresses
    - [ ] Closing a Stream
    - [ ] Directionality
    - [ ] Handling of Silent Peers
        - [ ] Dead Connection
        - [ ] Broken Stream
        - [ ] Idle Peer
        - [ ] Use of Checking Methods
    - [ ] Stream Attributes
        - [ ] from
        - [x] to
        - [x] id
        - [ ] xml:lang
        - [x] version
    - [ ] XML Namespaces
        - [ ] Stream Namespace
        - [ ] Content Namespace
        - [ ] XMPP Content Namespaces
        - [ ] Other Namespaces
        - [ ] Namespace Declarations and Prefixes
    - [ ] Stream Errors
        - [x] Rules
            - [x]  Stream Errors Are Unrecoverable
            - [x]  Stream Errors Can Occur During Setup
            - [x]  Stream Errors When the Host Is Unspecified or Unknown
            - [x]  Where Stream Errors Are Sent
        - [x] Syntax
        - [ ] Stream Error Conditions
            - [ ] bad-format -> not applied
            - [x] bad-namespace-prefix
            - [ ] conflict
            - [ ] connection-timeout
            - [ ] host-gone
            - [x] host-unknown
            - [ ] improper-addressing
            - [ ] internal-server-error
            - [ ] invalid-from
            - [x] invalid-namespace
            - [ ] invalid-xml
            - [x] not-authorized
            - [ ] not-well-formed
            - [ ] policy-violation
            - [ ] remote-connection-failed
            - [ ] reset
            - [ ] resource-constraint
            - [ ] restricted-xml
            - [ ] see-other-host
            - [ ] system-shutdown
            - [ ] undefined-condition
            - [x] unsupported-encoding
            - [ ] unsupported-feature
            - [ ] unsupported-stanza-type
            - [ ] unsupported-version
    - [ ] Application-Specific Conditions
    - [ ] Simplified Stream Examples

   - [ ] STARTTLS Negotiation
     - [x] Fundamentals
     - [x] Support
     - [ ] Stream Negotiation Rules
       - [ ] Mandatory-to-Negotiate
       - [ ] Restart
       - [ ] Data Formatting
       - [ ] Order of TLS and SASL Negotiations
       - [ ] TLS Renegotiation
       - [ ] TLS Extensions
     - [ ] Process
       - [ ] Exchange of Stream Headers and Stream Features
       - [ ] Initiation of STARTTLS Negotiation
         - [ ] STARTTLS Command
         - [ ] Failure Case
         - [ ] Proceed Case
       - [ ] TLS Negotiation
         - [ ] Rules
         - [ ] TLS Failure
         - [ ] TLS Success
   - [ ] SASL Negotiation
     - [ ] Fundamentals
     - [ ] Support
     - [ ] Stream Negotiation Rules
       - [ ] Mandatory-to-Negotiate
       - [ ] Restart
       - [ ] Mechanism Preferences
       - [ ] Mechanism Offers
       - [ ] Data Formatting
       - [ ] Security Layers
       - [ ] Simple User Name
       - [ ] Authorization Identity
       - [ ] Realms
       - [ ] Round Trips
     - [ ] Process
       - [ ] Exchange of Stream Headers and Stream Features
       - [ ] Initiation
       - [ ] Challenge-Response Sequence
       - [ ] Abort
       - [ ] SASL Failure
       - [ ] SASL Success
     - [ ] SASL Errors
       - [ ] aborted
       - [ ] account-disabled
       - [ ] credentials-expired
       - [ ] encryption-required
       - [ ] incorrect-encoding
       - [ ] invalid-authzid
       - [ ] invalid-mechanism
       - [ ] malformed-request
       - [ ] mechanism-too-weak
       - [ ] not-authorized
       - [ ] temporary-auth-failure
     - [ ] SASL Definition
   - [ ] Resource Binding
     - [ ] Fundamentals
     - [ ] Support
     - [ ] Stream Negotiation Rules
       - [ ] Mandatory-to-Negotiate
       - [ ] Restart
     - [ ] Advertising Support
     - [ ] Generation of Resource Identifiers
     - [ ] Server-Generated Resource Identifier
       - [ ] Success Case
       - [ ] Error Cases
         - [ ] Resource Constraint
         - [ ] Not Allowed
     - [ ] Client-Submitted Resource Identifier
       - [ ] Success Case
       - [ ] Error Cases
         - [ ] Bad Request
         - [ ] Conflict
       - [ ] Retries
   - [ ] XML Stanzas
     - [ ] Common Attributes
       - [ ] to
         - [ ] Client-to-Server Streams
         - [ ] Server-to-Server Streams
       - [ ] from
         - [ ] Client-to-Server Streams
         - [ ] Server-to-Server Streams
       - [ ] id
       - [ ] type
       - [ ] xml:lang
     - [ ] Basic Semantics
       - [ ] Message Semantics
       - [ ] Presence Semantics
       - [ ] IQ Semantics
     - [ ] Stanza Errors
       - [ ] Rules
       - [ ] Syntax
       - [ ] Defined Conditions
         - [ ] bad-request
         - [ ] conflict
         - [ ] feature-not-implemented
         - [ ] forbidden
         - [ ] gone
         - [ ] internal-server-error
         - [ ] item-not-found
         - [ ] jid-malformed
         - [ ] not-acceptable
         - [ ] not-allowed
         - [ ] not-authorized
         - [ ] policy-violation
         - [ ] recipient-unavailable
         - [ ] redirect
         - [ ] registration-required
         - [ ] remote-server-not-found
         - [ ] remote-server-timeout
         - [ ] resource-constraint
         - [ ] service-unavailable
         - [ ] subscription-required
         - [ ] undefined-condition
         - [ ] unexpected-request
       - [ ] Application-Specific Conditions
     - [ ] Extended Content
   - [ ] Detailed Examples
     - [ ] Client-to-Server Examples
       - [ ] TLS
       - [ ] SASL
       - [ ] Resource Binding
       - [ ] Stanza Exchange
       - [ ] Close
     - [ ] Server-to-Server Examples
       - [ ] TLS
       - [ ] SASL
       - [ ] Stanza Exchange
       - [ ] Close
   - [ ] Server Rules for Processing XML Stanzas
     - [ ] In-Order Processing
     - [ ] General Considerations
     - [ ] No 'to' Address
       - [ ] Message
       - [ ] Presence
       - [ ] IQ
     - [ ] Remote Domain
       - [ ] Existing Stream
       - [ ] No Existing Stream
       - [ ] Error Handling
     - [ ] Local Domain
       - [ ] domainpart
       - [ ] domainpart/resourcepart
       - [ ] localpart@domainpart
         - [ ] No Such User
         - [ ] User Exists
       - [ ] localpart@domainpart/resourcepart
   - [ ] XML Usage
     - [ ] XML Restrictions
     - [ ] XML Namespace Names and Prefixes
     - [ ] Well-Formedness
     - [ ] Validation
     - [ ] Inclusion of XML Declaration
     - [ ] Character Encoding
     - [ ] Whitespace
     - [ ] XML Versions
   - [ ] Internationalization Considerations
   - [ ] Security Considerations
     - [ ] Fundamentals
     - [ ] Threat Model
     - [ ] Order of Layers
     - [ ] Confidentiality and Integrity
     - [ ] Peer Entity Authentication
     - [ ] Strong Security
     - [ ] Certificates
       - [ ] Certificate Generation
         - [ ] General Considerations
         - [ ] Server Certificates
         - [ ] Client Certificates
         - [ ] XmppAddr Identifier Type
       - [ ] Certificate Validation
         - [ ] Server Certificates
         - [ ] Client Certificates
         - [ ] Checking of Certificates in Long-Lived Streams
         - [ ] Use of Certificates in XMPP Extensions
     - [ ] Mandatory-to-Implement TLS and SASL Technologies
       - [ ] For Authentication Only
       - [ ] For Confidentiality Only
       - [ ] For Confidentiality and Authentication with Passwords
       - [ ] For Confidentiality and Authentication without Passwords
     - [ ] Technology Reuse
       - [ ] Use of Base 64 in SASL
       - [ ] Use of DNS
       - [ ] Use of Hash Functions
       - [ ] Use of SASL
       - [ ] Use of TLS
       - [ ] Use of UTF-8
       - [ ] Use of XML
     - [ ] Information Leaks
       - [ ] IP Addresses
       - [ ] Presence Information
     - [ ] Directory Harvesting
     - [ ] Denial of Service
     - [ ] Firewalls
     - [ ] Interdomain Federation
     - [ ] Non-Repudiation
   - [ ] IANA Considerations
     - [ ] XML Namespace Name for TLS Data
     - [ ] XML Namespace Name for SASL Data
     - [ ] XML Namespace Name for Stream Errors
     - [ ] XML Namespace Name for Resource Binding
     - [ ] XML Namespace Name for Stanza Errors
     - [ ] GSSAPI Service Name
     - [ ] Port Numbers and Service Names
   - [ ] Conformance Requirements
   - [ ] References
     - [ ] Normative References
     - [ ] Informative References
   > Appendix A.  XML Schemas  . . . . . . . . . . . . . . . . . . . . 190
   >   A.1.   Stream Namespace . . . . . . . . . . . . . . . . . . . . 190
   >   A.2.   Stream Error Namespace . . . . . . . . . . . . . . . . . 192
   >   A.3.   STARTTLS Namespace . . . . . . . . . . . . . . . . . . . 193
   >   A.4.   SASL Namespace . . . . . . . . . . . . . . . . . . . . . 194
   >   A.5.   Client Namespace . . . . . . . . . . . . . . . . . . . . 196
   >   A.6.   Server Namespace . . . . . . . . . . . . . . . . . . . . 201
   >   A.7.   Resource Binding Namespace . . . . . . . . . . . . . . . 206
   >   A.8.   Stanza Error Namespace . . . . . . . . . . . . . . . . . 206
   > Appendix B.  Contact Addresses  . . . . . . . . . . . . . . . . . 208
   > Appendix C.  Account Provisioning . . . . . . . . . . . . . . . . 208
   > Appendix D.  Differences from RFC 3920  . . . . . . . . . . . . . 208
   > Appendix E.  Acknowledgements . . . . . . . . . . . . . . . . . . 210

</details>
