package command

import (
	"fmt"
	"log"

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

func runConfCmd(cmd *cobra.Command, args []string) {
	osWrap := new(syswrap.OsWrapper)

	file, err := osWrap.Open("./internal/domain/rayfile/testdata/test_rayfile.toml")
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
