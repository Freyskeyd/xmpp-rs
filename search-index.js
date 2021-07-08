var searchIndex = JSON.parse('{\
"server":{"doc":"","t":[5],"n":["main"],"q":["server"],"d":[""],"i":[0],"f":[[[]]],"p":[]},\
"xmpp":{"doc":"xmpp-rs is an implementation of the Extensible Messaging …","t":[],"n":[],"q":[],"d":[],"i":[],"f":[],"p":[]},\
"xmpp_credentials":{"doc":"","t":[3,11,11,11,11,11,11,11,11,11,11,12,11,12,11,11,11,11],"n":["Credentials","borrow","borrow_mut","clone","clone_into","default","eq","fmt","format","from","into","jid","ne","password","to_owned","try_from","try_into","type_id"],"q":["xmpp_credentials","","","","","","","","","","","","","","","","",""],"d":["Define Credentials used to authenticate a user","","","","","","","","","","","","","","","","",""],"i":[0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],"f":[null,[[]],[[]],[[],["credentials",3]],[[]],[[],["credentials",3]],[[["credentials",3]],["bool",15]],[[["formatter",3]],["result",6]],[[],["saslcredentials",3]],[[]],[[]],null,[[["credentials",3]],["bool",15]],null,[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]]],"p":[[3,"Credentials"]]},\
"xmpp_proto":{"doc":"","t":[3,13,13,3,13,13,3,13,13,16,16,4,13,8,3,13,13,13,13,13,13,13,4,13,13,4,13,8,13,3,13,3,4,4,13,3,13,13,3,13,13,4,13,3,13,13,3,13,4,3,13,3,8,13,13,13,13,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,12,11,11,11,0,11,11,11,11,11,11,12,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,17,17,17,17,17,17,17,17,17,17],"n":["Auth","Auth","BadNamespacePrefix","Bind","Bind","Bind","CloseStream","CloseStream","Error","Error","Error","Features","Final","FromXmlElement","GenericIq","Get","HostUnknown","IQ","InvalidNamespace","InvalidNamespace","InvalidPacket","Io","IqType","Mechanisms","Message","NonStanza","NonStanza","NonStanzaTrait","NotAuthorized","OpenStream","OpenStream","OpenStreamBuilder","Packet","PacketParsingError","Presence","ProceedTls","ProceedTls","Result","SASLSuccess","SASLSuccess","Set","Stanza","Stanza","StartTls","StartTls","StartTls","StreamError","StreamError","StreamErrorKind","StreamFeatures","StreamFeatures","StreamFeaturesBuilder","ToXmlElement","Unknown","Unknown","UnsupportedEncoding","UnsupportedVersion","Xml","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","build","build","challenge","clone","clone","clone","clone","clone","clone","clone","clone","clone","clone","clone","clone","clone","clone","clone","clone","clone","clone","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","default","default","default","default","default","default","default","default","default","default","default","eq","eq","features","features","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from_element","from_element","from_element","from_element","from_element","from_element","from_element","from_start_element","from_str","get_element","get_from","get_id","get_to","get_type","id","id","into","into","into","into","into","into","into","into","into","into","into","into","into","into","into","into","into","into","into","kind","lang","lang","mechanism","ne","new","ns","parse","set_from","set_id","set_to","set_type","to","to","to_element","to_element","to_element","to_element","to_element","to_element","to_element","to_element","to_element","to_element","to_element","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_string","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","unique_id","version","version","write_to_stream","BIND","CLIENT","PING","SASL","SERVER","SESSION","STANZAS","STREAM","TLS","XML_URI"],"q":["xmpp_proto","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","xmpp_proto::ns","","","","","","","","",""],"d":["","","","","","","","","","","","","","FromXmlElement is used to transform an Element to an …","","","","","","","","","","","","Define a sub part of a Packet, a NonStanza is the …","Represent a packet which is an XML Stream","","","Define an OpenStream NonStanza packet.","","Builder for <code>OpenStream</code>.","","","","","","","","","","Define a sub part of a Packet, a Stanza is the …","Represent a packet which isn’t an XML Stanza","","","","","","","","","Builder for <code>StreamFeatures</code>.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Builds a new <code>OpenStream</code>.","Builds a new <code>StreamFeatures</code>.","Get a reference to the auth’s challenge.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","An Id generated by the server.","An Id generated by the server.","","","","","","","","","","","","","","","","","","","","","The ‘xml:lang’ attribute specifies an entity’s …","The ‘xml:lang’ attribute specifies an entity’s …","Get a reference to the auth’s mechanism.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","The inclusion of the version attribute set to a value of …","The inclusion of the version attribute set to a value of …","","","","","","","","","","",""],"i":[0,1,2,0,3,1,0,1,4,5,6,0,7,0,0,4,2,8,2,7,9,7,0,3,8,0,9,0,2,0,1,0,0,0,8,0,1,4,0,1,4,0,9,0,3,1,0,1,0,0,1,0,0,3,7,2,2,7,10,11,12,13,14,15,16,17,18,2,19,20,3,1,9,7,21,4,8,10,11,12,13,14,15,16,17,18,2,19,20,3,1,9,7,21,4,8,13,19,10,10,11,12,13,14,15,16,17,18,2,19,20,3,1,9,21,4,8,10,11,12,13,14,15,16,17,18,2,19,20,3,1,9,21,4,8,10,11,13,14,15,16,17,19,20,3,21,3,4,19,20,10,11,12,14,15,16,17,18,2,20,3,1,9,7,21,4,4,8,10,11,12,13,13,14,15,16,17,18,2,19,20,3,3,1,9,9,9,9,9,9,9,9,9,9,9,9,9,9,7,7,7,21,4,8,14,5,10,11,15,17,20,21,14,4,21,21,21,21,21,13,14,10,11,12,13,14,15,16,17,18,2,19,20,3,1,9,7,21,4,8,18,13,14,10,3,21,0,9,21,21,21,21,13,14,6,11,12,14,15,16,18,20,1,21,8,10,11,12,13,14,15,16,17,18,2,19,20,3,1,9,21,4,8,4,10,11,12,13,14,15,16,17,18,2,19,20,3,1,9,9,7,21,4,8,10,11,12,13,14,15,16,17,18,2,19,20,3,1,9,7,21,4,8,10,11,12,13,14,15,16,17,18,2,19,20,3,1,9,7,21,4,8,21,13,14,9,0,0,0,0,0,0,0,0,0,0],"f":[null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],[["string",3],["result",4,["openstream","string"]],["openstream",3]]],[[],[["string",3],["result",4,["streamfeatures","string"]],["streamfeatures",3]]],[[],["option",4]],[[],["auth",3]],[[],["bind",3]],[[],["closestream",3]],[[],["openstreambuilder",3]],[[],["openstream",3]],[[],["proceedtls",3]],[[],["saslsuccess",3]],[[],["starttls",3]],[[],["streamerror",3]],[[],["streamerrorkind",4]],[[],["streamfeaturesbuilder",3]],[[],["streamfeatures",3]],[[],["features",4]],[[],["nonstanza",4]],[[],["packet",4]],[[],["genericiq",3]],[[],["iqtype",4]],[[],["stanza",4]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["auth",3]],[[],["bind",3]],[[],["openstreambuilder",3]],[[],["openstream",3]],[[],["proceedtls",3]],[[],["saslsuccess",3]],[[],["starttls",3]],[[],["streamfeaturesbuilder",3]],[[],["streamfeatures",3]],[[]],[[]],[[["features",4]],["bool",15]],[[["iqtype",4]],["bool",15]],[[["into",8,["features"]],["features",4]]],null,[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[["into",8,["option"]],["option",4,["jid"]]]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[["str",15]]],[[]],[[["proceedtls",3]]],[[["streamerror",3]]],[[["openstream",3]]],[[["closestream",3]]],[[["bind",3]]],[[["auth",3]]],[[["stanza",4]]],[[["streamfeatures",3]]],[[["saslsuccess",3]]],[[["starttls",3]]],[[["genericiq",3]]],[[]],[[]],[[["nonstanza",4]]],[[["error",3]]],[[]],[[["error",4]]],[[]],[[]],[[]],null,[[["element",3]],["result",4]],[[["element",3]],["result",4]],[[["element",3]],["result",4]],[[["element",3]],["result",4]],[[["element",3]],["result",4]],[[["element",3]],["result",4]],[[["element",3]],[["error",3],["result",4,["error"]]]],[[["ownedattribute",3],["vec",3,["ownedattribute"]]],[["packetparsingerror",4],["result",4,["packetparsingerror"]]]],[[["str",15]],["result",4]],[[],[["option",4,["element"]],["element",3]]],[[],[["option",4,["jid"]],["jid",4]]],[[],["str",15]],[[],[["option",4,["jid"]],["jid",4]]],[[],["iqtype",4]],[[["option",4,["string"]],["into",8,["option"]]]],null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],null,[[["into",8,["string"]],["string",3]]],null,[[],[["str",15],["option",4,["str"]]]],[[["features",4]],["bool",15]],[[["iqtype",4]],["genericiq",3]],null,[[["ownedattribute",3],["eventreader",3],["vec",3,["ownedattribute"]],["namespace",3],["ownedname",3]],[["packetparsingerror",4],["result",4,["packetparsingerror"]]]],[[["jid",4],["option",4,["jid"]]]],[[]],[[["jid",4],["option",4,["jid"]]]],[[["iqtype",4]]],[[["into",8,["option"]],["option",4,["jid"]]]],null,[[],[["result",4,["element"]],["element",3]]],[[],[["error",3],["result",4,["element","error"]],["element",3]]],[[],[["error",3],["result",4,["element","error"]],["element",3]]],[[],[["result",4,["element"]],["element",3]]],[[],[["error",3],["result",4,["element","error"]],["element",3]]],[[],[["result",4,["element"]],["element",3]]],[[],[["error",3],["result",4,["element","error"]],["element",3]]],[[],[["error",3],["result",4,["element","error"]],["element",3]]],[[],[["result",4,["element"]],["element",3]]],[[],[["result",4,["element"]],["element",3]]],[[],[["result",4,["element"]],["element",3]]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["string",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[["element",3]],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["string",3]],[[["into",8,["string"]],["string",3]]],null,[[["write",8]],[["error",3],["result",4,["error"]]]],null,null,null,null,null,null,null,null,null,null],"p":[[4,"NonStanza"],[4,"StreamErrorKind"],[4,"Features"],[4,"IqType"],[8,"FromXmlElement"],[8,"ToXmlElement"],[4,"PacketParsingError"],[4,"Stanza"],[4,"Packet"],[3,"Auth"],[3,"Bind"],[3,"CloseStream"],[3,"OpenStreamBuilder"],[3,"OpenStream"],[3,"ProceedTls"],[3,"SASLSuccess"],[3,"StartTls"],[3,"StreamError"],[3,"StreamFeaturesBuilder"],[3,"StreamFeatures"],[3,"GenericIq"]]},\
"xmpp_server":{"doc":"","t":[3,8,16,3,3,8,11,11,11,11,11,11,11,11,11,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["AuthenticationRequest","Authenticator","Config","Server","ServerBuilder","Service","add_authenticator","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","build","cert","create_with_config","default","from","from","from","into","into","into","keys","launch","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","vzip","vzip","vzip"],"q":["xmpp_server","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[0,0,1,0,0,0,2,3,4,2,3,4,2,3,2,1,2,3,4,2,3,4,2,2,2,3,4,2,3,4,2,3,4,2,3,4,2],"f":[null,null,null,null,null,null,[[["str",15],["authenticationrequest",3],["recipient",3,["authenticationrequest"]]]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["serverbuilder",3]],[[["into",8,["string"]],["string",3]]],[[]],[[],["serverbuilder",3]],[[]],[[]],[[]],[[]],[[]],[[]],[[["into",8,["string"]],["string",3]]],[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[]],[[]],[[]]],"p":[[8,"Service"],[3,"ServerBuilder"],[3,"Server"],[3,"AuthenticationRequest"]]},\
"xmpp_xml":{"doc":"","t":[8,3,13,3,3,13,3,4,3,3,13,13,3,3,3,13,13,13,13,13,3,4,4,11,11,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12],"n":["AsQName","Attrs","Borrowed","Children","ChildrenMut","DuplicateNamespacePrefix","Element","Error","FindChildren","FindChildrenMut","Io","MalformedXml","NamespaceMap","Position","QName","Shared","UnexpectedEvent","Utf8","Version10","Version11","WriteOptions","XmlAtom","XmlProlog","append_child","append_new_child","as_qname","attr_count","attrs","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","cause","child_count","children","children_mut","clone","clone","clone","clone","clone","clone_into","clone_into","clone_into","clone_into","clone_into","cmp","cmp","cmp","column","column","default","default","deref","description","eq","eq","eq","find","find_all","find_all_mut","find_mut","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from_name","from_ns_name","from_reader","from_start_element","from_xml_start_element","get_attr","get_child","get_child_mut","get_namespace_prefix","get_prefix","hash","into","into","into","into","into","into","into","into","into","into","into","into","into","into_iter","into_iter","into_iter","into_iter","into_iter","line","line","name","navigate","ne","new","new","new","new","new_with_namespaces","next","next","next","next","next","ns","partial_cmp","partial_cmp","partial_cmp","position","register_if_missing","register_namespace","remove_attr","remove_child","set_attr","set_namespace_prefix","set_prefix","set_tag","set_tail","set_text","set_write_end_tag","set_xml_prolog","share","tag","tail","text","to_owned","to_owned","to_owned","to_owned","to_owned","to_string","to_string","to_string","to_string","to_writer","to_writer_with_options","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","write_end_tag","msg","msg","pos","pos"],"q":["xmpp_xml","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","xmpp_xml::Error","","",""],"d":["Convenience trait to get a <code>QName</code> from an object.","An iterator over attributes of an element.","","An iterator over children of an element.","A mutable iterator over children of an element.","A namespace prefix was already used","Represents an XML element.","Errors that can occur parsing XML","An iterator over matching children.","A mutable iterator over matching children.","An IO Error","The XML is invalid","","Represents a position in the source.","A <code>QName</code> represents a qualified name.","","This library is unable to process this XML. This can …","A UTF-8 Error","","","A struct that define write options.","","Xml Prolog version handle by xmpp_xml","Appends a new child and returns a reference to self.","Appends a new child to the element and returns a …","Returns a Cow’ed <code>QName</code> from the given object.","Count the attributes","Returns an iterator over all attributes","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns the number of children","Returns an iterator over all children.","Returns a mutable iterator over all children.","","","","","","","","","","","","","","Returns the column of the error or 0 if unknown","Returns the column of the position","","","","","","","","Finds the first matching child","Returns all children with the given name.","Returns all children with the given name.","Finds the first matching child and returns a mut ref","","","","","","","","","","","","","","","","","","","","","","","","Creates a qualified name from a given string.","","Creates a qualified name from a given string without …","Creates a qualified name from a namespace and name.","Parses some XML data into an <code>Element</code> from a reader.","","","Look up an attribute by qualified name.","Returns the nth child.","Returns the nth child as a mutable reference.","Returns the assigned prefix for a namespace.","","","","","","","","","","","","","","","","","","","","","Returns the line number of the error or 0 if unknown","Returns the line number of the position","Returns the name portion of the qualified name.  This is …","Finds the first element that match a given path downwards","","","Creates a new element without any children but a given …","","Creates a new position.","Creates a new element without any children but inheriting …","","","","","","Returns the optional namespace of this element.  This is …","","","","Returns the position of the error if known","","Registers a namespace with the internal namespace map.","Removes an attribute and returns the stored string.","Removes a child.","Sets a new attribute.","Sets a specific namespace prefix.  This will also …","","Sets a new tag for the element.","Sets a new tail text value for the tag.","Sets a new text value for the tag.","Define if we write the end tag of an element.","Define which xml prolog will be displayed when rendering …","Creates a shared <code>QName</code> with static lifetime from an …","The tag of the element as qualified name.","Returns the tail text of a tag.","Returns the text of a tag.","","","","","","Dump an element as XML document into a string","","","","Dump an element as XML document into a writer.","Dump an element as XML document into a writer with option.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[0,0,1,0,0,2,0,0,0,0,2,2,0,0,0,1,2,2,3,3,0,0,0,4,4,5,4,4,6,7,8,9,10,3,11,1,1,4,2,12,13,14,6,7,8,9,10,3,11,1,4,2,12,13,14,2,4,4,4,1,4,12,13,14,1,4,12,13,14,1,13,14,2,13,11,12,1,2,1,13,14,4,4,4,4,1,4,2,2,12,13,13,14,14,6,7,8,9,10,3,11,1,4,2,2,2,12,13,14,14,14,14,4,4,4,4,4,4,4,12,14,6,7,8,9,10,3,11,1,4,2,12,13,14,6,7,8,9,10,2,13,14,4,13,11,4,12,13,4,6,7,8,9,10,14,1,13,14,2,12,4,4,4,4,4,12,4,4,4,11,11,14,4,4,4,1,4,12,13,14,4,2,13,14,4,4,6,7,8,9,10,3,11,1,4,2,12,13,14,6,7,8,9,10,3,11,1,4,2,12,13,14,6,7,8,9,10,3,11,1,4,2,12,13,14,4,15,16,15,16],"f":[null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[["element",3]],["element",3]],[[["asqname",8]],["element",3]],[[],[["cow",4,["qname"]],["qname",3]]],[[],["usize",15]],[[],["attrs",3]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["str",15]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],[["option",4,["error"]],["error",8]]],[[],["usize",15]],[[],["children",3]],[[],["childrenmut",3]],[[],["xmlatom",4]],[[],["element",3]],[[],["namespacemap",3]],[[],["position",3]],[[],["qname",3]],[[]],[[]],[[]],[[]],[[]],[[["xmlatom",4]],["ordering",4]],[[["position",3]],["ordering",4]],[[["qname",3]],["ordering",4]],[[],["u64",15]],[[],["u64",15]],[[],["writeoptions",3]],[[],["namespacemap",3]],[[],["str",15]],[[],["str",15]],[[["xmlatom",4]],["bool",15]],[[["position",3]],["bool",15]],[[["qname",3]],["bool",15]],[[["asqname",8]],[["element",3],["option",4,["element"]]]],[[["asqname",8]],["findchildren",3]],[[["asqname",8]],["findchildrenmut",3]],[[["asqname",8]],[["option",4,["element"]],["element",3]]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[["xmlwriteerror",4]],["error",4]],[[["xmlreaderror",3]],["error",4]],[[]],[[]],[[["str",15]],["qname",3]],[[]],[[["str",15]],["qname",3]],[[["option",4,["str"]],["str",15]],["qname",3]],[[["read",8]],[["element",3],["error",4],["result",4,["element","error"]]]],[[["option",4,["arc"]],["ownedname",3],["arc",3,["namespacemap"]],["eventreader",3],["ownedattribute",3],["vec",3,["ownedattribute"]],["xmlnamespacemap",3]],[["element",3],["error",4],["result",4,["element","error"]]]],[[["xmlevent",4],["eventreader",3]],[["result",4,["error"]],["error",4]]],[[["asqname",8]],[["option",4,["str"]],["str",15]]],[[["usize",15]],[["element",3],["option",4,["element"]]]],[[["usize",15]],[["option",4,["element"]],["element",3]]],[[["str",15]],[["str",15],["option",4,["str"]]]],[[["str",15]],[["str",15],["option",4,["str"]]]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["u64",15]],[[],["u64",15]],[[],["str",15]],[[],[["element",3],["option",4,["element"]]]],[[["position",3]],["bool",15]],[[],["writeoptions",3]],[[["asqname",8]],["element",3]],[[],["namespacemap",3]],[[["u64",15]],["position",3]],[[["asqname",8],["element",3]],["element",3]],[[],[["element",3],["option",4,["element"]]]],[[],[["option",4,["element"]],["element",3]]],[[],["option",4]],[[],[["element",3],["option",4,["element"]]]],[[],[["option",4,["element"]],["element",3]]],[[],[["str",15],["option",4,["str"]]]],[[["xmlatom",4]],[["option",4,["ordering"]],["ordering",4]]],[[["position",3]],[["ordering",4],["option",4,["ordering"]]]],[[["qname",3]],[["option",4,["ordering"]],["ordering",4]]],[[],[["position",3],["option",4,["position"]]]],[[["str",15],["option",4,["str"]]],["bool",15]],[[["str",15],["option",4,["str"]]]],[[["asqname",8]],[["string",3],["option",4,["string"]]]],[[["usize",15]],[["element",3],["option",4,["element"]]]],[[["string",3],["asqname",8],["into",8,["string"]]],["element",3]],[[["str",15]],[["error",4],["result",4,["error"]]]],[[["str",15]],[["error",4],["result",4,["error"]]]],[[["qname",3]],["element",3]],[[["string",3],["into",8,["string"]]],["element",3]],[[["string",3],["into",8,["string"]]],["element",3]],[[["bool",15]]],[[["option",4,["xmlprolog"]],["xmlprolog",4]]],[[],["qname",3]],[[],["qname",3]],[[],["str",15]],[[],["str",15]],[[]],[[]],[[]],[[]],[[]],[[],[["result",4,["string","error"]],["string",3],["error",4]]],[[],["string",3]],[[],["string",3]],[[],["string",3]],[[["write",8]],[["error",4],["result",4,["error"]]]],[[["write",8],["writeoptions",3]],[["error",4],["result",4,["error"]]]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[["bool",15]]],null,null,null,null],"p":[[4,"XmlAtom"],[4,"Error"],[4,"XmlProlog"],[3,"Element"],[8,"AsQName"],[3,"Children"],[3,"ChildrenMut"],[3,"Attrs"],[3,"FindChildren"],[3,"FindChildrenMut"],[3,"WriteOptions"],[3,"NamespaceMap"],[3,"Position"],[3,"QName"],[13,"MalformedXml"],[13,"UnexpectedEvent"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};