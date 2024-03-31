.PHONY: count
count:
	@ find . -name tests -prune -o -type f -name '*.rs' | xargs wc -l