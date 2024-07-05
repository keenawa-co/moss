.PHONY: count
count:
	@ find . -type f -name '*.rs' | grep -v '/target/' | xargs wc -l
