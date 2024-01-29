package command

import (
	"fmt"
	"log"
	"path/filepath"
	"runtime"

	"github.com/4rchr4y/goray/internal/domain/rayfile"
	"github.com/4rchr4y/goray/internal/domain/service/toml"
	"github.com/4rchr4y/goray/internal/infra/syswrap"
	"github.com/spf13/cobra"
)

var confCmd = &cobra.Command{
	Use:   "conf",
	Short: "",
	Long:  "",
	Run:   runConfCmd,
}

func init() {
	RootCmd.AddCommand(confCmd)
}

var (
	_, b, _, _ = runtime.Caller(0)
	basepath   = filepath.Dir(b)
)

func runConfCmd(cmd *cobra.Command, args []string) {
	osWrap := new(syswrap.OsWrapper)

	file, err := osWrap.Open(fmt.Sprintf("%s/../../internal/domain/rayfile/testdata/test_rayfile.toml", basepath))
	if err != nil {
		log.Fatal(err)
		return
	}

	ts := toml.NewTomlService()
	rs := rayfile.NewRayfileService(ts)

	ioWrap := new(syswrap.IoWrapper)
	conf, err := rs.Parse(ioWrap, file)

	if err != nil {
		log.Fatalf(err.Error())
	}

	fmt.Printf("\n%#v\n", conf)
	fmt.Printf("%#v\n", conf.Workspace)

}
