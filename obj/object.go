package obj

const NoPos int = 0

type Object interface {
	IsValid() bool
	IsExported() bool
}
