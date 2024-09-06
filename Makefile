DESKTOP_DIR = view/desktop
STORYBOOK_DIR = view/storybook
DOCS_DIR = view/docs
WEB_DIR = view/web
THEME_GENERATOR_DIR = tools/theme-generator

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

run-theme-generator:
	@cd $(THEME_GENERATOR_DIR) && $(PNPM) start

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
	@find . -type f -name '*.rs' | grep -v '/target/' | xargs wc -l

# Clean up merged branches except master, main, and dev
cleanup:
	@git branch --merged | grep -Ev "(^\*|master|main|dev)" | xargs git branch -d