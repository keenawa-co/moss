.PHONY: count
count:
	find . -name tests -prune -o -type f -name '*.go' | xargs wc -l


.PHONY: pack
pack:
	tar -czvf bundle.tar.gz -C ./builtin/opa/lib/ .