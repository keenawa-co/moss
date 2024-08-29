.PHONY: count
count:
	@ find . -type f -name '*.rs' | grep -v '/target/' | xargs wc -l


.PHONY: cleanup
cleanup:
	git branch --merged | grep -Ev "(^\*|master|main|dev)" | xargs git branch -d

DESKTOP_DIR = view/desktop
STORYBOOK_DIR = view/storybook
DOCS_DIR = view/docs
WEB_DIR = view/web

run-desktop:
	cd $(DESKTOP_DIR) && pnpm tauri dev

run-desktop-web:
	cd $(DESKTOP_DIR) && pnpm vite dev

run-storybook:
	cd $(STORYBOOK_DIR) && pnpm dev

run-database:
	cd $(DESKTOP_DIR) && surreal start file:rocksdb

run-docs:
	cd $(DOCS_DIR) && pnpm dev

run-web:
	cd $(WEB_DIR) && pnpm dev