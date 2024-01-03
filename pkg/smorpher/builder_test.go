package smorpher

import (
	"fmt"
	"reflect"
	"testing"
)

type testWorkspace struct {
	Path    string    `json:"path"`
	Targets []*string `json:"targets"`
	Emm     []struct {
		K1 []string `json:"k1"`
	} `json:"emm"`
}

type testData struct {
	Term      string           `json:"term"`
	Set       []string         `json:"set"`
	Workspace []*testWorkspace `json:"workspace"`
}

type testStruct struct {
	Version string    `json:"version"`
	User    *string   `json:"user"`
	Data    *testData `json:"data"`
}

func TestBuilder(t *testing.T) {
	user := "g10z3r"
	version := "1.1"
	m := map[string]interface{}{
		"version": &version,
		"user":    user,

		"data": map[string]interface{}{
			"term": "xterm-256color",
			// "set":  &user,
			"path": "src/dir",
			"set":  []string{"foo", "bar", "zip", "zap"},
			"workspace": []map[string]interface{}{
				{
					"path":    "root",
					"targets": []string{"t1", "t2", "t3"},
					"emm": []map[string]interface{}{
						{
							"k1": "v1",
						},
					},
				},
				{
					"path": "not-root",
				},
				{
					"path": "foo",
				},
			},
		},
	}

	ts := &testStruct{}
	v, err := NewBuilder(ts, m, WithMode(Autocomplete))
	if err != nil {
		fmt.Println(err)
	}

	Walk(v, &Field{
		Path:  nil,
		Value: m,
		Kind:  reflect.ValueOf(m).Kind(),
	})

	fmt.Println("version", ts.Version)
	fmt.Println("user", *ts.User)
	fmt.Println("set, pointer to string", ts.Data.Set)
	fmt.Println(ts.Data.Workspace[0].Emm[0].K1)
	fmt.Println("targets, string to pointer", ts.Data.Workspace[0].Targets)

	t.Fail()
}
