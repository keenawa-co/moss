package loader

import (
	"fmt"

	gitService "github.com/4rchr4y/goray/internal/domain/service/git"
	"github.com/4rchr4y/goray/internal/ropa/types"
	"github.com/go-git/go-git/v5"
	"github.com/go-git/go-git/v5/plumbing"
	"github.com/go-git/go-git/v5/plumbing/object"
	"github.com/go-git/go-git/v5/storage/memory"
)

type gitClient interface {
	PlainClone(input *gitService.PlainCloneInput) (*git.Repository, error)
}

type bundleParser interface {
	Parse(input *ParseInput) (*types.Bundle, error)
}

type GitLoader struct {
	bparser bundleParser
	gitCli  gitClient
}

func NewGitLoader(gitClient gitClient, bparser bundleParser) *GitLoader {
	return &GitLoader{
		gitCli:  gitClient,
		bparser: bparser,
	}
}

type DownloadResult struct {
	Hash    string // commit hash
	Version string // github tag
	Bundle  *types.Bundle
}

func (loader *GitLoader) DownloadBundle(url string, tag string) (*DownloadResult, error) {
	repo, err := git.Clone(memory.NewStorage(), nil, &git.CloneOptions{URL: url})
	if err != nil {
		return nil, err
	}

	ref, err := getRef(repo, tag)
	if err != nil {
		return nil, err
	}

	commit, err := repo.CommitObject(ref.Hash())
	if err != nil {
		return nil, err
	}

	filesIter, err := commit.Files()
	if err != nil {
		return nil, err
	}

	files := make(map[string][]byte)
	err = filesIter.ForEach(func(f *object.File) error {
		content, err := f.Contents()
		if err != nil {
			return err
		}

		files[f.Name] = []byte(content)
		return nil
	})
	if err != nil {
		return nil, err
	}

	parserInput := &ParseInput{
		FileName: "test1234",
		Files:    files,
	}

	bundle, err := loader.bparser.Parse(parserInput)

	return &DownloadResult{
		Version: tag,
		Hash:    ref.Hash().String(),
		Bundle:  bundle,
	}, nil
}

func getRef(repo *git.Repository, tag string) (*plumbing.Reference, error) {
	if tag != "" {
		return findTag(repo, tag)
	}
	return repo.Head()
}

func findTag(repo *git.Repository, tag string) (*plumbing.Reference, error) {
	tags, err := repo.Tags()
	if err != nil {
		return nil, err
	}

	var foundTag *plumbing.Reference
	err = tags.ForEach(func(t *plumbing.Reference) error {
		if t.Name().Short() == tag {
			foundTag = t
			return nil
		}

		return nil
	})

	if err != nil {
		return nil, err
	}

	if foundTag == nil {
		return nil, fmt.Errorf("version '%s' is not found", tag)
	}

	return foundTag, nil
}
