package convert

import (
	"github.com/4rchr4y/goray/interface/component"
	"github.com/4rchr4y/goray/internal/proto/protocomponent"
)

var FromComponentProtoStatus = [...]component.Status{
	protocomponent.Heartbeat_UNKNOWN: component.StatusUnknown,
	protocomponent.Heartbeat_OK:      component.StatusOK,
}

var ToComponentProtoStatus = [...]protocomponent.Heartbeat_Status{
	component.StatusOK:      protocomponent.Heartbeat_OK,
	component.StatusUnknown: protocomponent.Heartbeat_UNKNOWN,
}

// Panic here indicates a mismatch between the types in the
// protocol and in the code. Should never happen.
var (
	_ = [1]int{}[len(FromComponentProtoStatus)-len(ToComponentProtoStatus)]
	_ = [1]int{}[len(ToComponentProtoStatus)-len(FromComponentProtoStatus)]
)
