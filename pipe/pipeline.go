package pipe

import (
	"context"

	"github.com/4rchr4y/go-compass/event"
	"github.com/4rchr4y/go-compass/pipe/plugin"
)

type Pipeline struct {
	head    plugin.Plugin
	tail    plugin.Plugin
	eventCh chan event.Event
}

func (p *Pipeline) Add(plugin plugin.Plugin) {
	if p.head == nil {
		p.head = plugin
		p.tail = plugin
		return
	}

	p.tail.SetNext(plugin)

	if plugin.IsTerminal() {
		p.tail = plugin.Next()
		return
	}

	p.tail = plugin
}

func (p *Pipeline) Run(ctx context.Context, input interface{}) interface{} {
	current := p.head
	var output interface{} = input

	for current != nil {
		output = current.Execute(ctx, output)
		current = current.Next()
	}

	return output
}

func NewPipeline(eventCh chan event.Event) *Pipeline {
	return &Pipeline{
		eventCh: eventCh,
	}
}

// func main() {
// 	plugin1 := &ExamplePlugin{name: "Plugin1"}
// 	plugin2 := &ExamplePlugin{name: "Plugin2"}
// 	plugin3 := &ExamplePlugin{name: "Plugin3"}
// 	plugin4 := &ExamplePlugin{name: "Plugin4"}

// 	splitter := &Splitter{branches: []Plugin{plugin1, plugin2}}

// 	customMergeFunc := func(ch chan interface{}) interface{} {
// 		var result string
// 		for data := range ch {
// 			result += data.(string)
// 		}
// 		return result
// 	}

// 	joiner := &Joiner{next: plugin3, mergeFunc: customMergeFunc}

// 	pipeline := &Pipeline{}
// 	pipeline.Add(splitter)
// 	pipeline.Add(joiner)

// 	pipeline.Add(plugin4)
// 	// pipeline.Add(plugin3)
// 	for i := 0; i < 10; i++ {
// 		finalData := pipeline.Run("")
// 		fmt.Println("Final data:", finalData)
// 	}

// }
