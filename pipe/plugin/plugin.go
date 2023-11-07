package plugin

import "context"

type Plugin interface {
	Name() string
	Next() Plugin
	SetNext(Plugin)
	IsTerminal() bool
	Execute(ctx context.Context, input interface{}) interface{}
}
