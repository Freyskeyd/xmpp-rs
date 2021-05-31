(function() {var implementors = {};
implementors["xmpp_proto"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"xmpp_proto/struct.Auth.html\" title=\"struct xmpp_proto::Auth\">Auth</a>&gt; for <a class=\"enum\" href=\"xmpp_proto/enum.Packet.html\" title=\"enum xmpp_proto::Packet\">Packet</a>","synthetic":false,"types":["xmpp_proto::packet::Packet"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"xmpp_proto/struct.Bind.html\" title=\"struct xmpp_proto::Bind\">Bind</a>&gt; for <a class=\"enum\" href=\"xmpp_proto/enum.Packet.html\" title=\"enum xmpp_proto::Packet\">Packet</a>","synthetic":false,"types":["xmpp_proto::packet::Packet"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"xmpp_proto/struct.CloseStream.html\" title=\"struct xmpp_proto::CloseStream\">CloseStream</a>&gt; for <a class=\"enum\" href=\"xmpp_proto/enum.Packet.html\" title=\"enum xmpp_proto::Packet\">Packet</a>","synthetic":false,"types":["xmpp_proto::packet::Packet"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"xmpp_proto/struct.OpenStream.html\" title=\"struct xmpp_proto::OpenStream\">OpenStream</a>&gt; for <a class=\"enum\" href=\"xmpp_proto/enum.Packet.html\" title=\"enum xmpp_proto::Packet\">Packet</a>","synthetic":false,"types":["xmpp_proto::packet::Packet"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"xmpp_proto/struct.ProceedTls.html\" title=\"struct xmpp_proto::ProceedTls\">ProceedTls</a>&gt; for <a class=\"enum\" href=\"xmpp_proto/enum.Packet.html\" title=\"enum xmpp_proto::Packet\">Packet</a>","synthetic":false,"types":["xmpp_proto::packet::Packet"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"xmpp_proto/struct.SASLSuccess.html\" title=\"struct xmpp_proto::SASLSuccess\">SASLSuccess</a>&gt; for <a class=\"enum\" href=\"xmpp_proto/enum.Packet.html\" title=\"enum xmpp_proto::Packet\">Packet</a>","synthetic":false,"types":["xmpp_proto::packet::Packet"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"xmpp_proto/struct.StartTls.html\" title=\"struct xmpp_proto::StartTls\">StartTls</a>&gt; for <a class=\"enum\" href=\"xmpp_proto/enum.Packet.html\" title=\"enum xmpp_proto::Packet\">Packet</a>","synthetic":false,"types":["xmpp_proto::packet::Packet"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"xmpp_proto/struct.StreamError.html\" title=\"struct xmpp_proto::StreamError\">StreamError</a>&gt; for <a class=\"enum\" href=\"xmpp_proto/enum.Packet.html\" title=\"enum xmpp_proto::Packet\">Packet</a>","synthetic":false,"types":["xmpp_proto::packet::Packet"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"xmpp_proto/struct.StreamFeatures.html\" title=\"struct xmpp_proto::StreamFeatures\">StreamFeatures</a>&gt; for <a class=\"enum\" href=\"xmpp_proto/enum.Packet.html\" title=\"enum xmpp_proto::Packet\">Packet</a>","synthetic":false,"types":["xmpp_proto::packet::Packet"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;'_ <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>&gt; for <a class=\"enum\" href=\"xmpp_proto/enum.Features.html\" title=\"enum xmpp_proto::Features\">Features</a>","synthetic":false,"types":["xmpp_proto::non_stanza::stream_features::Features"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;T&gt; for <a class=\"enum\" href=\"xmpp_proto/enum.Packet.html\" title=\"enum xmpp_proto::Packet\">Packet</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"xmpp_proto/trait.NonStanzaTrait.html\" title=\"trait xmpp_proto::NonStanzaTrait\">NonStanzaTrait</a>,&nbsp;</span>","synthetic":false,"types":["xmpp_proto::packet::Packet"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"xmpp_proto/enum.NonStanza.html\" title=\"enum xmpp_proto::NonStanza\">NonStanza</a>&gt; for <a class=\"enum\" href=\"xmpp_proto/enum.Packet.html\" title=\"enum xmpp_proto::Packet\">Packet</a>","synthetic":false,"types":["xmpp_proto::packet::Packet"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"xmpp_proto/enum.Stanza.html\" title=\"enum xmpp_proto::Stanza\">Stanza</a>&gt; for <a class=\"enum\" href=\"xmpp_proto/enum.Packet.html\" title=\"enum xmpp_proto::Packet\">Packet</a>","synthetic":false,"types":["xmpp_proto::packet::Packet"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"xmpp_xml/error/enum.Error.html\" title=\"enum xmpp_xml::error::Error\">Error</a>&gt; for <a class=\"enum\" href=\"xmpp_proto/enum.PacketParsingError.html\" title=\"enum xmpp_proto::PacketParsingError\">PacketParsingError</a>","synthetic":false,"types":["xmpp_proto::packet::PacketParsingError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt; for <a class=\"enum\" href=\"xmpp_proto/enum.PacketParsingError.html\" title=\"enum xmpp_proto::PacketParsingError\">PacketParsingError</a>","synthetic":false,"types":["xmpp_proto::packet::PacketParsingError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"xmpp_proto/struct.GenericIq.html\" title=\"struct xmpp_proto::GenericIq\">GenericIq</a>&gt; for <a class=\"enum\" href=\"xmpp_proto/enum.Packet.html\" title=\"enum xmpp_proto::Packet\">Packet</a>","synthetic":false,"types":["xmpp_proto::packet::Packet"]}];
implementors["xmpp_xml"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;Error&gt; for <a class=\"enum\" href=\"xmpp_xml/enum.Error.html\" title=\"enum xmpp_xml::Error\">Error</a>","synthetic":false,"types":["xmpp_xml::error::Error"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;EmitterError&gt; for <a class=\"enum\" href=\"xmpp_xml/enum.Error.html\" title=\"enum xmpp_xml::Error\">Error</a>","synthetic":false,"types":["xmpp_xml::error::Error"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()