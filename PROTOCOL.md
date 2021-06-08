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
        - [ ] Rules
            - [ ]  Stream Errors Are Unrecoverable
            - [ ]  Stream Errors Can Occur During Setup
            - [ ]  Stream Errors When the Host Is Unspecified or Unknown
            - [ ]  Where Stream Errors Are Sent
        - [ ] Syntax
        - [ ] Stream Error Conditions
            - [ ] bad-format
            - [ ] bad-namespace-prefix
            - [ ] conflict
            - [ ] connection-timeout
            - [ ] host-gone
            - [x] host-unknown
            - [ ] improper-addressing
            - [ ] internal-server-error
            - [ ] invalid-from
            - [ ] invalid-namespace
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

   > 5.  STARTTLS Negotiation  . . . . . . . . . . . . . . . . . . . .  69
   >   5.1.   Fundamentals . . . . . . . . . . . . . . . . . . . . . .  69
   >   5.2.   Support  . . . . . . . . . . . . . . . . . . . . . . . .  70
   >   5.3.   Stream Negotiation Rules . . . . . . . . . . . . . . . .  70
   >     5.3.1.   Mandatory-to-Negotiate . . . . . . . . . . . . . . .  70
   >     5.3.2.   Restart  . . . . . . . . . . . . . . . . . . . . . .  70
   >     5.3.3.   Data Formatting  . . . . . . . . . . . . . . . . . .  70
   >     5.3.4.   Order of TLS and SASL Negotiations . . . . . . . . .  71
   >     5.3.5.   TLS Renegotiation  . . . . . . . . . . . . . . . . .  71
   >     5.3.6.   TLS Extensions . . . . . . . . . . . . . . . . . . .  72
   >   5.4.   Process  . . . . . . . . . . . . . . . . . . . . . . . .  72
   >     5.4.1.   Exchange of Stream Headers and Stream Features . . .  72
   >     5.4.2.   Initiation of STARTTLS Negotiation . . . . . . . . .  73
   >       5.4.2.1.  STARTTLS Command  . . . . . . . . . . . . . . . .  73
   >       5.4.2.2.  Failure Case  . . . . . . . . . . . . . . . . . .  73
   >       5.4.2.3.  Proceed Case  . . . . . . . . . . . . . . . . . .  74
   >     5.4.3.   TLS Negotiation  . . . . . . . . . . . . . . . . . .  74
   >       5.4.3.1.  Rules . . . . . . . . . . . . . . . . . . . . . .  74
   >       5.4.3.2.  TLS Failure . . . . . . . . . . . . . . . . . . .  75
   >       5.4.3.3.  TLS Success . . . . . . . . . . . . . . . . . . .  76
   > 6.  SASL Negotiation  . . . . . . . . . . . . . . . . . . . . . .  77
   >   6.1.   Fundamentals . . . . . . . . . . . . . . . . . . . . . .  77
   >   6.2.   Support  . . . . . . . . . . . . . . . . . . . . . . . .  77
   >   6.3.   Stream Negotiation Rules . . . . . . . . . . . . . . . .  77
   >     6.3.1.   Mandatory-to-Negotiate . . . . . . . . . . . . . . .  77
   >     6.3.2.   Restart  . . . . . . . . . . . . . . . . . . . . . .  78
   >     6.3.3.   Mechanism Preferences  . . . . . . . . . . . . . . .  78
   >     6.3.4.   Mechanism Offers . . . . . . . . . . . . . . . . . .  78
   >     6.3.5.   Data Formatting  . . . . . . . . . . . . . . . . . .  79
   >     6.3.6.   Security Layers  . . . . . . . . . . . . . . . . . .  80
   >     6.3.7.   Simple User Name . . . . . . . . . . . . . . . . . .  80
   >     6.3.8.   Authorization Identity . . . . . . . . . . . . . . .  80
   >     6.3.9.   Realms . . . . . . . . . . . . . . . . . . . . . . .  81
   >     6.3.10.  Round Trips  . . . . . . . . . . . . . . . . . . . .  81
   >   6.4.   Process  . . . . . . . . . . . . . . . . . . . . . . . .  82
   >     6.4.1.   Exchange of Stream Headers and Stream Features . . .  82
   >     6.4.2.   Initiation . . . . . . . . . . . . . . . . . . . . .  83
   >     6.4.3.   Challenge-Response Sequence  . . . . . . . . . . . .  84
   >     6.4.4.   Abort  . . . . . . . . . . . . . . . . . . . . . . .  84
   >     6.4.5.   SASL Failure . . . . . . . . . . . . . . . . . . . .  85
   >     6.4.6.   SASL Success . . . . . . . . . . . . . . . . . . . .  86
   >   6.5.   SASL Errors  . . . . . . . . . . . . . . . . . . . . . .  87
   >     6.5.1.   aborted  . . . . . . . . . . . . . . . . . . . . . .  88
   >     6.5.2.   account-disabled . . . . . . . . . . . . . . . . . .  88
   >     6.5.3.   credentials-expired  . . . . . . . . . . . . . . . .  88
   >     6.5.4.   encryption-required  . . . . . . . . . . . . . . . .  89
   >     6.5.5.   incorrect-encoding . . . . . . . . . . . . . . . . .  89
   >     6.5.6.   invalid-authzid  . . . . . . . . . . . . . . . . . .  89
   >     6.5.7.   invalid-mechanism  . . . . . . . . . . . . . . . . .  90
   >     6.5.8.   malformed-request  . . . . . . . . . . . . . . . . .  90
   >     6.5.9.   mechanism-too-weak . . . . . . . . . . . . . . . . .  90
   >     6.5.10.  not-authorized . . . . . . . . . . . . . . . . . . .  91
   >     6.5.11.  temporary-auth-failure . . . . . . . . . . . . . . .  91
   >   6.6.   SASL Definition  . . . . . . . . . . . . . . . . . . . .  91
   > 7.  Resource Binding  . . . . . . . . . . . . . . . . . . . . . .  92
   >   7.1.   Fundamentals . . . . . . . . . . . . . . . . . . . . . .  92
   >   7.2.   Support  . . . . . . . . . . . . . . . . . . . . . . . .  93
   >   7.3.   Stream Negotiation Rules . . . . . . . . . . . . . . . .  93
   >     7.3.1.   Mandatory-to-Negotiate . . . . . . . . . . . . . . .  93
   >     7.3.2.   Restart  . . . . . . . . . . . . . . . . . . . . . .  93
   >   7.4.   Advertising Support  . . . . . . . . . . . . . . . . . .  93
   >   7.5.   Generation of Resource Identifiers . . . . . . . . . . .  94
   >   7.6.   Server-Generated Resource Identifier . . . . . . . . . .  94
   >     7.6.1.   Success Case . . . . . . . . . . . . . . . . . . . .  94
   >     7.6.2.   Error Cases  . . . . . . . . . . . . . . . . . . . .  95
   >       7.6.2.1.  Resource Constraint . . . . . . . . . . . . . . .  95
   >       7.6.2.2.  Not Allowed . . . . . . . . . . . . . . . . . . .  96
   >   7.7.   Client-Submitted Resource Identifier . . . . . . . . . .  96
   >     7.7.1.   Success Case . . . . . . . . . . . . . . . . . . . .  96
   >     7.7.2.   Error Cases  . . . . . . . . . . . . . . . . . . . .  97
   >       7.7.2.1.  Bad Request . . . . . . . . . . . . . . . . . . .  97
   >       7.7.2.2.  Conflict  . . . . . . . . . . . . . . . . . . . .  97
   >     7.7.3.   Retries  . . . . . . . . . . . . . . . . . . . . . .  99
   > 8.  XML Stanzas . . . . . . . . . . . . . . . . . . . . . . . . .  99
   >   8.1.   Common Attributes  . . . . . . . . . . . . . . . . . . . 100
   >     8.1.1.   to . . . . . . . . . . . . . . . . . . . . . . . . . 100
   >       8.1.1.1.  Client-to-Server Streams  . . . . . . . . . . . . 100
   >       8.1.1.2.  Server-to-Server Streams  . . . . . . . . . . . . 101
   >     8.1.2.   from . . . . . . . . . . . . . . . . . . . . . . . . 101
   >       8.1.2.1.  Client-to-Server Streams  . . . . . . . . . . . . 101
   >       8.1.2.2.  Server-to-Server Streams  . . . . . . . . . . . . 102
   >     8.1.3.   id . . . . . . . . . . . . . . . . . . . . . . . . . 103
   >     8.1.4.   type . . . . . . . . . . . . . . . . . . . . . . . . 103
   >     8.1.5.   xml:lang . . . . . . . . . . . . . . . . . . . . . . 103
   >   8.2.   Basic Semantics  . . . . . . . . . . . . . . . . . . . . 105
   >     8.2.1.   Message Semantics  . . . . . . . . . . . . . . . . . 105
   >     8.2.2.   Presence Semantics . . . . . . . . . . . . . . . . . 105
   >     8.2.3.   IQ Semantics . . . . . . . . . . . . . . . . . . . . 105
   >   8.3.   Stanza Errors  . . . . . . . . . . . . . . . . . . . . . 107
   >     8.3.1.   Rules  . . . . . . . . . . . . . . . . . . . . . . . 108
   >     8.3.2.   Syntax . . . . . . . . . . . . . . . . . . . . . . . 109
   >     8.3.3.   Defined Conditions . . . . . . . . . . . . . . . . . 110
   >       8.3.3.1.  bad-request . . . . . . . . . . . . . . . . . . . 110
   >       8.3.3.2.  conflict  . . . . . . . . . . . . . . . . . . . . 111
   >       8.3.3.3.  feature-not-implemented . . . . . . . . . . . . . 111
   >       8.3.3.4.  forbidden . . . . . . . . . . . . . . . . . . . . 112
   >       8.3.3.5.  gone  . . . . . . . . . . . . . . . . . . . . . . 113
   >       8.3.3.6.  internal-server-error . . . . . . . . . . . . . . 113
   >       8.3.3.7.  item-not-found  . . . . . . . . . . . . . . . . . 114
   >       8.3.3.8.  jid-malformed . . . . . . . . . . . . . . . . . . 114
   >       8.3.3.9.  not-acceptable  . . . . . . . . . . . . . . . . . 115
   >       8.3.3.10. not-allowed . . . . . . . . . . . . . . . . . . . 116
   >       8.3.3.11. not-authorized  . . . . . . . . . . . . . . . . . 116
   >       8.3.3.12. policy-violation  . . . . . . . . . . . . . . . . 117
   >       8.3.3.13. recipient-unavailable . . . . . . . . . . . . . . 117
   >       8.3.3.14. redirect  . . . . . . . . . . . . . . . . . . . . 118
   >       8.3.3.15. registration-required . . . . . . . . . . . . . . 119
   >       8.3.3.16. remote-server-not-found . . . . . . . . . . . . . 119
   >       8.3.3.17. remote-server-timeout . . . . . . . . . . . . . . 120
   >       8.3.3.18. resource-constraint . . . . . . . . . . . . . . . 121
   >       8.3.3.19. service-unavailable . . . . . . . . . . . . . . . 121
   >       8.3.3.20. subscription-required . . . . . . . . . . . . . . 122
   >       8.3.3.21. undefined-condition . . . . . . . . . . . . . . . 123
   >       8.3.3.22. unexpected-request  . . . . . . . . . . . . . . . 123
   >     8.3.4.   Application-Specific Conditions  . . . . . . . . . . 124
   >   8.4.   Extended Content . . . . . . . . . . . . . . . . . . . . 125
   > 9.  Detailed Examples . . . . . . . . . . . . . . . . . . . . . . 128
   >   9.1.   Client-to-Server Examples  . . . . . . . . . . . . . . . 128
   >     9.1.1.   TLS  . . . . . . . . . . . . . . . . . . . . . . . . 128
   >     9.1.2.   SASL . . . . . . . . . . . . . . . . . . . . . . . . 130
   >     9.1.3.   Resource Binding . . . . . . . . . . . . . . . . . . 132
   >     9.1.4.   Stanza Exchange  . . . . . . . . . . . . . . . . . . 133
   >     9.1.5.   Close  . . . . . . . . . . . . . . . . . . . . . . . 134
   >   9.2.   Server-to-Server Examples  . . . . . . . . . . . . . . . 134
   >     9.2.1.   TLS  . . . . . . . . . . . . . . . . . . . . . . . . 134
   >     9.2.2.   SASL . . . . . . . . . . . . . . . . . . . . . . . . 136
   >     9.2.3.   Stanza Exchange  . . . . . . . . . . . . . . . . . . 137
   >     9.2.4.   Close  . . . . . . . . . . . . . . . . . . . . . . . 137
   > 10. Server Rules for Processing XML Stanzas . . . . . . . . . . . 138
   >   10.1.  In-Order Processing  . . . . . . . . . . . . . . . . . . 138
   >   10.2.  General Considerations . . . . . . . . . . . . . . . . . 140
   >   10.3.  No 'to' Address  . . . . . . . . . . . . . . . . . . . . 141
   >     10.3.1.  Message  . . . . . . . . . . . . . . . . . . . . . . 141
   >     10.3.2.  Presence . . . . . . . . . . . . . . . . . . . . . . 141
   >     10.3.3.  IQ . . . . . . . . . . . . . . . . . . . . . . . . . 141
   >   10.4.  Remote Domain  . . . . . . . . . . . . . . . . . . . . . 142
   >     10.4.1.  Existing Stream  . . . . . . . . . . . . . . . . . . 142
   >     10.4.2.  No Existing Stream . . . . . . . . . . . . . . . . . 142
   >     10.4.3.  Error Handling . . . . . . . . . . . . . . . . . . . 143
   >   10.5.  Local Domain . . . . . . . . . . . . . . . . . . . . . . 143
   >     10.5.1.  domainpart . . . . . . . . . . . . . . . . . . . . . 143
   >     10.5.2.  domainpart/resourcepart  . . . . . . . . . . . . . . 143
   >     10.5.3.  localpart@domainpart . . . . . . . . . . . . . . . . 143
   >       10.5.3.1. No Such User  . . . . . . . . . . . . . . . . . . 144
   >       10.5.3.2. User Exists . . . . . . . . . . . . . . . . . . . 144
   >     10.5.4.  localpart@domainpart/resourcepart  . . . . . . . . . 144
   > 11. XML Usage . . . . . . . . . . . . . . . . . . . . . . . . . . 145
   >   11.1.  XML Restrictions . . . . . . . . . . . . . . . . . . . . 145
   >   11.2.  XML Namespace Names and Prefixes . . . . . . . . . . . . 146
   >   11.3.  Well-Formedness  . . . . . . . . . . . . . . . . . . . . 146
   >   11.4.  Validation . . . . . . . . . . . . . . . . . . . . . . . 147
   >   11.5.  Inclusion of XML Declaration . . . . . . . . . . . . . . 147
   >   11.6.  Character Encoding . . . . . . . . . . . . . . . . . . . 147
   >   11.7.  Whitespace . . . . . . . . . . . . . . . . . . . . . . . 148
   >   11.8.  XML Versions . . . . . . . . . . . . . . . . . . . . . . 148
   > 12. Internationalization Considerations . . . . . . . . . . . . . 148
   > 13. Security Considerations . . . . . . . . . . . . . . . . . . . 148
   >   13.1.  Fundamentals . . . . . . . . . . . . . . . . . . . . . . 148
   >   13.2.  Threat Model . . . . . . . . . . . . . . . . . . . . . . 149
   >   13.3.  Order of Layers  . . . . . . . . . . . . . . . . . . . . 150
   >   13.4.  Confidentiality and Integrity  . . . . . . . . . . . . . 150
   >   13.5.  Peer Entity Authentication . . . . . . . . . . . . . . . 151
   >   13.6.  Strong Security  . . . . . . . . . . . . . . . . . . . . 151
   >   13.7.  Certificates . . . . . . . . . . . . . . . . . . . . . . 152
   >     13.7.1.  Certificate Generation . . . . . . . . . . . . . . . 152
   >       13.7.1.1. General Considerations  . . . . . . . . . . . . . 152
   >       13.7.1.2. Server Certificates . . . . . . . . . . . . . . . 153
   >       13.7.1.3. Client Certificates . . . . . . . . . . . . . . . 156
   >       13.7.1.4. XmppAddr Identifier Type  . . . . . . . . . . . . 156
   >     13.7.2.  Certificate Validation . . . . . . . . . . . . . . . 157
   >       13.7.2.1. Server Certificates . . . . . . . . . . . . . . . 158
   >       13.7.2.2. Client Certificates . . . . . . . . . . . . . . . 158
   >       13.7.2.3. Checking of Certificates in Long-Lived Streams  . 160
   >       13.7.2.4. Use of Certificates in XMPP Extensions  . . . . . 160
   >   13.8.  Mandatory-to-Implement TLS and SASL Technologies . . . . 160
   >     13.8.1.  For Authentication Only  . . . . . . . . . . . . . . 161
   >     13.8.2.  For Confidentiality Only . . . . . . . . . . . . . . 161
   >     13.8.3.  For Confidentiality and Authentication with
   >              Passwords  . . . . . . . . . . . . . . . . . . . . . 162
   >     13.8.4.  For Confidentiality and Authentication without
   >              Passwords  . . . . . . . . . . . . . . . . . . . . . 163
   >   13.9.  Technology Reuse . . . . . . . . . . . . . . . . . . . . 163
   >     13.9.1.  Use of Base 64 in SASL . . . . . . . . . . . . . . . 163
   >     13.9.2.  Use of DNS . . . . . . . . . . . . . . . . . . . . . 163
   >     13.9.3.  Use of Hash Functions  . . . . . . . . . . . . . . . 164
   >     13.9.4.  Use of SASL  . . . . . . . . . . . . . . . . . . . . 164
   >     13.9.5.  Use of TLS . . . . . . . . . . . . . . . . . . . . . 165
   >     13.9.6.  Use of UTF-8 . . . . . . . . . . . . . . . . . . . . 165
   >     13.9.7.  Use of XML . . . . . . . . . . . . . . . . . . . . . 166
   >   13.10. Information Leaks  . . . . . . . . . . . . . . . . . . . 166
   >     13.10.1. IP Addresses . . . . . . . . . . . . . . . . . . . . 166
   >     13.10.2. Presence Information . . . . . . . . . . . . . . . . 166
   >   13.11. Directory Harvesting . . . . . . . . . . . . . . . . . . 166
   >   13.12. Denial of Service  . . . . . . . . . . . . . . . . . . . 167
   >   13.13. Firewalls  . . . . . . . . . . . . . . . . . . . . . . . 169
   >   13.14. Interdomain Federation . . . . . . . . . . . . . . . . . 169
   >   13.15. Non-Repudiation  . . . . . . . . . . . . . . . . . . . . 169
   > 14. IANA Considerations . . . . . . . . . . . . . . . . . . . . . 170
   >   14.1.  XML Namespace Name for TLS Data  . . . . . . . . . . . . 170
   >   14.2.  XML Namespace Name for SASL Data . . . . . . . . . . . . 170
   >   14.3.  XML Namespace Name for Stream Errors . . . . . . . . . . 170
   >   14.4.  XML Namespace Name for Resource Binding  . . . . . . . . 171
   >   14.5.  XML Namespace Name for Stanza Errors . . . . . . . . . . 171
   >   14.6.  GSSAPI Service Name  . . . . . . . . . . . . . . . . . . 171
   >   14.7.  Port Numbers and Service Names . . . . . . . . . . . . . 171
   > 15. Conformance Requirements  . . . . . . . . . . . . . . . . . . 172
   > 16. References  . . . . . . . . . . . . . . . . . . . . . . . . . 181
   >   16.1.  Normative References . . . . . . . . . . . . . . . . . . 181
   >   16.2.  Informative References . . . . . . . . . . . . . . . . . 184
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
