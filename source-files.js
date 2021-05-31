var N = null;var sourcesIndex = {};
sourcesIndex["server"] = {"name":"","files":["main.rs"]};
sourcesIndex["xmpp"] = {"name":"","files":["lib.rs"]};
sourcesIndex["xmpp_credentials"] = {"name":"","files":["lib.rs"]};
sourcesIndex["xmpp_proto"] = {"name":"","dirs":[{"name":"non_stanza","files":["auth.rs","bind.rs","close_stream.rs","open_stream.rs","proceed_tls.rs","sasl_success.rs","start_tls.rs","stream_error.rs","stream_features.rs"]},{"name":"stanza","files":["generic_iq.rs"]}],"files":["lib.rs","non_stanza.rs","ns.rs","packet.rs","stanza.rs"]};
sourcesIndex["xmpp_server"] = {"name":"","dirs":[{"name":"listeners","dirs":[{"name":"tcp","files":["listener.rs","session.rs"]}],"files":["tcp.rs","ws.rs"]},{"name":"parser","files":["codec.rs","sink.rs"]},{"name":"sessions","files":["manager.rs","state.rs","unauthenticated.rs"]}],"files":["authentication.rs","lib.rs","listeners.rs","parser.rs","router.rs","sessions.rs"]};
sourcesIndex["xmpp_xml"] = {"name":"","files":["children.rs","element.rs","error.rs","lib.rs","namespace.rs","options.rs","position.rs","qname.rs","xml_atom.rs"]};
createSourceSidebar();
