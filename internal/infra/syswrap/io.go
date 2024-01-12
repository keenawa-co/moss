package syswrap

import "io"

// TODO: IoWrapper
type IoClient struct{}

func (IoClient) Copy(dst io.Writer, src io.Reader) (int64, error) {
	return io.Copy(dst, src)
}

func (IoClient) ReadAll(reader io.Reader) ([]byte, error) {
	return io.ReadAll(reader)
}
