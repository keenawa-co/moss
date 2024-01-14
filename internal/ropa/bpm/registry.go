package bpm

import "fmt"

type commandRegistry map[string]Command

func (cr commandRegistry) get(name string) (Command, error) {
	cmd, ok := cr[name]
	if !ok {
		return nil, fmt.Errorf("command '%s' is doesn't exists", name)
	}

	return cmd, nil
}

func (cr commandRegistry) set(command Command) error {
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
		cr[command.Name()].SetCommand(cmd)
	}

	return nil
}
