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
	run-docs \
	run-web \
	count \
	cleanup \


run-desktop:
	@cd $(DESKTOP_DIR) && $(PNPM) tauri dev

run-desktop-web:
	@cd $(DESKTOP_DIR) && $(PNPM) vite dev

run-storybook:
	@cd $(STORYBOOK_DIR) && $(PNPM) dev

run-database:
	@cd $(DESKTOP_DIR) && $(SURREAL) start file:rocksdb

run-docs:
	@cd $(DOCS_DIR) && $(PNPM) dev

run-web:
	@cd $(WEB_DIR) && $(PNPM) dev
	

# Count lines of Rust code, excluding the 'target' directory
count:
	@find . -type f -name '*.rs' | grep -v '/target/' | xargs wc -l

# Clean up merged branches except master, main, and dev
cleanup:
	@git branch --merged | grep -Ev "(^\*|master|main|dev)" | xargs git branch -d
