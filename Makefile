.PHONY: run
run:
	cd testdata && go vet ./...
#	go run example/main.go

.PHONY: validate
validate:
	cd testdata && go vet ./...


.PHONY: count
count:
	find . -name tests -prune -o -type f -name '*.go' | xargs wc -l