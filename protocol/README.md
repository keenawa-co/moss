rpp - ray plugin protocol

protoc -I ./protocol/plugin-proto/ \
  --go_out=./internal/proto/pluginproto/ --go_opt=paths=source_relative \
  --go-grpc_out=./internal/proto/pluginproto/ --go-grpc_opt=paths=source_relative \
  ./protocol/plugin-proto/rayplugin1.0.proto