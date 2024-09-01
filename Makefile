DESKTOP_DIR = view/desktop
STORYBOOK_DIR = view/storybook
DOCS_DIR = view/docs
WEB_DIR = view/web


PNPM = pnpm
SURREAL = surreal

.DEFAULT_GOAL := run-desktop

.PHONY: \
	run-desktop \
	run-desktop-web \
	run-storybook \
	run-database \
	stop-database \
	run-docs \
	run-web \
	check-db \
	count \
	cleanup \


run-desktop: check-db
	@cd $(DESKTOP_DIR) && $(PNPM) tauri dev

run-desktop-web:
	@cd $(DESKTOP_DIR) && $(PNPM) vite dev

run-storybook:
	@cd $(STORYBOOK_DIR) && $(PNPM) dev

run-database:
	@cd $(DESKTOP_DIR) && $(SURREAL) start file:rocksdb &

stop-database:
	@pkill -x surreal

run-docs:
	@cd $(DOCS_DIR) && $(PNPM) dev

run-web:
	@cd $(WEB_DIR) && $(PNPM) dev


# Check if the database is running, if not, start it in the background
check-db:
ifeq ($(OS),Windows_NT)
	@powershell -Command "if (!(Get-Process surreal -ErrorAction SilentlyContinue)) { Start-Process -NoNewWindow -FilePath 'make' -ArgumentList 'run-database' }"
else
	@if ! pgrep -x "surreal" > /dev/null; then \
		$(MAKE) run-database; \
	fi
endif
	
# Count lines of Rust code, excluding the 'target' directory
count:
ifeq ($(OS),Windows_NT)
	@powershell -Command "Get-ChildItem -Recurse -Filter *.rs | Where-Object { $$_.FullName -notmatch '\\target\\' } | ForEach-Object { (Get-Content $$_.FullName | Measure-Object -Line).Lines } | Measure-Object -Sum | Select-Object -ExpandProperty Sum"
else
	find . -type f -name '*.rs' | grep -v '/target/' | xargs wc -l | tail -n 1 | awk '{print $$1}'
endif

# Clean up merged branches except master, main, and dev
cleanup:
ifeq ($(OS),Windows_NT)
	@git branch --merged | findstr /V /C:"*" /C:"master" /C:"main" /C:"dev" | for /F "tokens=*" %i in ('more') do git branch -d %i
else
	@git branch --merged | grep -Ev "(^\*|master|main|dev)" | xargs git branch -d
endif
