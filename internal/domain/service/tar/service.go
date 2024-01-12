package tar

import (
	"archive/tar"
	"compress/gzip"
	"fmt"
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

func NewTarService(osCli osClient, fsCli fsClient, ioCli ioClient) *TarService {
	return &TarService{
		osCli: osCli,
		fsCli: fsCli,
		ioCli: ioCli,
	}
}

func (ts *TarService) Compress(dirPath string, targetDir string, archiveName string) error {
	archiveFile, err := ts.osCli.Create(filepath.Join(targetDir, archiveName))
	if err != nil {
		return err
	}
	defer archiveFile.Close()

	gzipWriter := ts.fsCli.GzipWriter(archiveFile)
	defer gzipWriter.Close()

	tarWriter := ts.fsCli.TarWriter(gzipWriter)
	defer tarWriter.Close()

	err = filepath.Walk(dirPath, func(filePath string, fi os.FileInfo, err error) error {
		if err != nil {
			return err
		}

		relativePath, err := filepath.Rel(dirPath, filePath)
		if err != nil {
			return err
		}

		if fi.IsDir() {
			if err := ts.processDirToTar(fi, tarWriter, relativePath); err != nil {
				return fmt.Errorf("failed to process directory '%s' into archive: %v", relativePath, err)
			}

			return nil
		}

		if err := ts.processFileToTar(filePath, tarWriter, relativePath); err != nil {
			return fmt.Errorf("failed to process file '%s' into archive: %v", relativePath, err)
		}

		return nil
	})

	if err != nil {
		return fmt.Errorf("error occurred while walking through the directory '%s': %v", dirPath, err)
	}

	return nil
}

func (ts *TarService) processDirToTar(fi os.FileInfo, writer *tar.Writer, relativePath string) error {
	header := &tar.Header{
		Name:    relativePath + "/",
		Mode:    int64(fi.Mode()),
		ModTime: fi.ModTime(),
	}

	return writer.WriteHeader(header)
}

func (ts *TarService) processFileToTar(filePath string, writer *tar.Writer, relativePath string) error {
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
		Name:    relativePath,
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
