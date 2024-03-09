RAY_DIR := .ray
PROTOCOL_VERSION := 1.0

.PHONY: protoc
protoc:
	@export PATH="${PATH}:$(shell go env GOPATH)/bin" ; \
	protoc -I ./protocol/ \
		--go_out=./internal/proto/ --go_opt=paths=source_relative \
		--go-grpc_out=./internal/proto/ --go-grpc_opt=paths=source_relative \
		protocol/protodriver/driver${PROTOCOL_VERSION}.proto \
		protocol/protoschema/schema1.0.proto

.PHONY: count
count:
	@ find . -name tests -prune -o -type f -name '*.go' | xargs wc -l


.PHONY: noop_driver
noop_driver:
	go build -o ./.ray/driver/github.com/4rchr4y/ray-noop-driver@v0.0.1 ./example/noop-driver/main


