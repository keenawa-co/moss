.PHONY: run_testdata_db
run_testdata_db:
	surreal start --log trace file:mossdb

.PHONY: count
count:
	@ find . -name tests -prune -o -type f -name '*.rs' | xargs wc -l
