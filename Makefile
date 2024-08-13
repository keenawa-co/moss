.PHONY: count
count:
	@ find . -type f -name '*.rs' | grep -v '/target/' | xargs wc -l


.PHONY: cleanup
cleanup:
	git branch --merged | grep -Ev "(^\*|master|main|dev)" | xargs git branch -d
