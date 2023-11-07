package plugin

import "github.com/4rchr4y/go-compass/event"

type JoinerPlugin struct {
	name    string
	next    Plugin
	eventCh chan event.Event

	mergeFunc func(chan interface{}) interface{}
}

func (p *JoinerPlugin) Name() string {
	return p.name
}

func (j *JoinerPlugin) IsTerminal() bool {
	return j.next != nil
}

func (e *JoinerPlugin) Next() Plugin {
	return e.next
}

func (s *JoinerPlugin) SetNext(p Plugin) {
	s.next = p
}

func (j *JoinerPlugin) Execute(input interface{}) interface{} {
	mergedData := j.mergeFunc(input.(chan interface{}))
	return mergedData
}
