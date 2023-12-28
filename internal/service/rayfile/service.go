package rayfile

type tomlService interface {
	Decode(data string, v interface{}) error
}

type RayfileService struct {
	toml tomlService
}

// func NewRayfile(reader io.Reader) *RayfileService {

// }
