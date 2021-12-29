(function() {var implementors = {};
implementors["async_channel"] = [{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"futures_core/stream/trait.FusedStream.html\" title=\"trait futures_core::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"async_channel/struct.Receiver.html\" title=\"struct async_channel::Receiver\">Receiver</a>&lt;T&gt;","synthetic":false,"types":["async_channel::Receiver"]}];
implementors["futures_channel"] = [{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"futures_core/stream/trait.FusedStream.html\" title=\"trait futures_core::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_channel/mpsc/struct.Receiver.html\" title=\"struct futures_channel::mpsc::Receiver\">Receiver</a>&lt;T&gt;","synthetic":false,"types":["futures_channel::mpsc::Receiver"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"futures_core/stream/trait.FusedStream.html\" title=\"trait futures_core::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_channel/mpsc/struct.UnboundedReceiver.html\" title=\"struct futures_channel::mpsc::UnboundedReceiver\">UnboundedReceiver</a>&lt;T&gt;","synthetic":false,"types":["futures_channel::mpsc::UnboundedReceiver"]}];
implementors["futures_core"] = [];
implementors["libp2p_swarm"] = [{"text":"impl&lt;TBehaviour&gt; <a class=\"trait\" href=\"futures_core/stream/trait.FusedStream.html\" title=\"trait futures_core::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"libp2p_swarm/struct.Swarm.html\" title=\"struct libp2p_swarm::Swarm\">Swarm</a>&lt;TBehaviour&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;TBehaviour: <a class=\"trait\" href=\"libp2p_swarm/trait.NetworkBehaviour.html\" title=\"trait libp2p_swarm::NetworkBehaviour\">NetworkBehaviour</a>,&nbsp;</span>","synthetic":false,"types":["libp2p_swarm::Swarm"]}];
implementors["sc_utils"] = [{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"futures_core/stream/trait.FusedStream.html\" title=\"trait futures_core::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"sc_utils/mpsc/struct.TracingUnboundedReceiver.html\" title=\"struct sc_utils::mpsc::TracingUnboundedReceiver\">TracingUnboundedReceiver</a>&lt;T&gt;","synthetic":false,"types":["sc_utils::mpsc::inner::TracingUnboundedReceiver"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()