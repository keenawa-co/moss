.PHONY: count
count:
	@ find . -type f -name '*.rs' | grep -v '/target/' | xargs wc -l


.PHONY: cleanup
cleanup:
	git branch -vv | grep ': gone]' | awk '{print $1}'
