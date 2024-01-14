package bpm

type Bpm struct {
	registry commandRegistry
}

func NewBpm() *Bpm {
	return &Bpm{
		registry: make(commandRegistry),
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
