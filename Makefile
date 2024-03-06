RAY_DIR := .ray
PROTOCOL_VERSION := 1.0

.PHONY: protoc
protoc:
	protoc -I ./protocol/plugin-proto/ \
		--go_out=./internal/proto/pluginproto/ --go_opt=paths=source_relative \
		--go-grpc_out=./internal/proto/pluginproto/ --go-grpc_opt=paths=source_relative \
		./protocol/plugin-proto/rayplugin${PROTOCOL_VERSION}.proto


.PHONY: count
count:
	@ find . -name tests -prune -o -type f -name '*.go' | xargs wc -l


.PHONY: noop_provider
noop_provider:
	go build -o ./.ray/provider/github.com/4rchr4y/ray-noop-provider@v0.0.1 ./example/noop-provider/main


