package ason

import "go/token"

var tokens = map[string]token.Token{
	"ILLEGAL": token.ILLEGAL,

	"STRING": token.STRING,

	"var": token.VAR,
}
