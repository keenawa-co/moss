package bpm

import (
	"fmt"
	"io"
)

type ioWrapper interface {
	ReadAll(r io.Reader) ([]byte, error)
}

type cmdRegistry map[string]Command

func (cr cmdRegistry) get(name string) (Command, error) {
	cmd, ok := cr[name]
	if !ok {
		return nil, fmt.Errorf("command '%s' is doesn't exists", name)
	}

	return cmd, nil
}

func (cr cmdRegistry) set(command Command) error {
	_, ok := cr[command.Name()]
	if ok {
		return fmt.Errorf("command '%s' is already exists", command.Name())
	}

	// register command
	cr[command.Name()] = command

	for i := range command.Requires() {
		cmd, ok := cr[command.Requires()[i]]
		if !ok {
			return fmt.Errorf("command '%s' is doesn't exists", command.Requires()[i])
		}

		// register nested command
		cr[command.Name()].setCommand(cmd)
	}

	return nil
}

type Bpm struct {
	registry cmdRegistry
}

func NewBpm() *Bpm {
	return &Bpm{
		registry: make(cmdRegistry),
	}
}

func (bpm *Bpm) RegisterCommand(commands ...Command) error {
	for i := range commands {
		if err := bpm.registry.set(commands[i]); err != nil {
			return err
		}
	}

	return nil
}

func (bpm *Bpm) Command(cmdName string) (Command, error) {
	return bpm.registry.get(cmdName)
}
