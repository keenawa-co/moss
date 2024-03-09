package grpcplugin

import pluginHCL "github.com/hashicorp/go-plugin"

const (
	ProtocolVersion = 1
)

var Handshake = pluginHCL.HandshakeConfig{
	ProtocolVersion:  ProtocolVersion,
	MagicCookieKey:   "RAY_MAGIC_COOKIE",
	MagicCookieValue: "dW5kZXJzdGFuZGluZyBob3cgbXkgY2FyIHdvcmtzIGhhcyBtYWRlIG1lIGEgYmV0dGVyIGRyaXZlci4=",
}
