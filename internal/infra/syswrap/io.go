package syswrap

import "io"

type IoWrapper struct{}

func (IoWrapper) Copy(dst io.Writer, src io.Reader) (int64, error) {
	return io.Copy(dst, src)
}

func (IoWrapper) ReadAll(reader io.Reader) ([]byte, error) {
	return io.ReadAll(reader)
}
