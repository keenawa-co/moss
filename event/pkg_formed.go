package event

import "github.com/4rchr4y/go-compass/obj"

// e.eventCh <- &event.PackageFormedEvent{
// 	Package: e.parsePkg(fset, pkgAst, targetDir, info.ModuleName),
// }

type PackageFormedEvent struct {
	Package *obj.PackageObj
}

func (d *PackageFormedEvent) Name() string {
	return "PackageFormed"
}
