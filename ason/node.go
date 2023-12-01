package ason

import (
	"go/token"
)

type NodeMod struct {
	Type string `json:"Type"`
}

type CommentMod struct {
	NodeMod

	Slash token.Pos `json:"Slash"`          // position of "/" starting the comment
	Text  string    `json:"Text,omitempty"` // comment text (excluding '\n' for //-style comments)
}

type ObjectMod struct {
	NodeMod

	Kind string `json:"Kind"`
	Name string `json:"Name,omitempty"`
	Decl any    `json:"Decl,omitempty"`
}

type IdentMod struct {
	NodeMod

	Pos        token.Pos  `json:"Pos"`
	End        token.Pos  `json:"End"`
	Name       string     `json:"Name,omitempty"`
	Obj        *ObjectMod `json:"Obj,omitempty"`
	IsExported bool       `json:"IsExported"`
}
