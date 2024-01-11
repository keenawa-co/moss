package tar

import (
	"archive/tar"
	"compress/gzip"
	"io"
	"os"
	"path/filepath"
)

type osClient interface {
	Open(name string) (*os.File, error)
	Create(name string) (*os.File, error)
}

type fsClient interface {
	GzipWriter(w io.Writer) *gzip.Writer
	TarWriter(w io.Writer) *tar.Writer
}

type ioClient interface {
	Copy(dst io.Writer, src io.Reader) (int64, error)
}

type TarService struct {
	osCli osClient
	fsCli fsClient
	ioCli ioClient
}

func (ts *TarService) Compress(files []string, targetDir string, archiveName string) error {
	archiveFile, err := ts.osCli.Create(filepath.Join(targetDir, archiveName))
	if err != nil {
		return err
	}
	defer archiveFile.Close()

	gzipWriter := ts.fsCli.GzipWriter(archiveFile)
	defer gzipWriter.Close()

	tarWriter := ts.fsCli.TarWriter(gzipWriter)
	defer tarWriter.Close()

	for _, file := range files {
		if err := ts.processFileToTar(file, tarWriter); err != nil {
			return err
		}
	}

	return nil
}

func (ts *TarService) processFileToTar(filePath string, writer *tar.Writer) error {
	file, err := ts.osCli.Open(filePath)
	if err != nil {
		return err
	}
	defer file.Close()

	stat, err := file.Stat()
	if err != nil {
		return err
	}

	header := &tar.Header{
		Name:    filepath.Base(filePath),
		Size:    stat.Size(),
		Mode:    int64(stat.Mode()),
		ModTime: stat.ModTime(),
	}

	if err := writer.WriteHeader(header); err != nil {
		return err
	}

	_, err = io.Copy(writer, file)
	return err
}
