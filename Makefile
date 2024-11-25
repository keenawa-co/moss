.DEFAULT_GOAL := run-desktop

# Detect Operating System
ifeq ($(OS),Windows_NT)
    DETECTED_OS := Windows
else
    DETECTED_OS := $(shell uname)
endif

# Directories
DESKTOP_DIR := view/desktop
STORYBOOK_DIR := view/storybook
DOCS_DIR := view/docs
WEB_DIR := view/web
THEME_GENERATOR_DIR := tools/themegen
ICONS_DIR := tools/icongen

DESKTOP_MODELS_DIR := internal/workbench/desktop/models
HTML_MODELS_DIR := crates/moss-html
UIKIT_MODELS_DIR := crates/moss-uikit

XTASK_DIR := tools/xtask
# Executables
PNPM := pnpm
SURREAL := surreal
CARGO := cargo
RUSTUP := rustup

# Database settings
DATABASE_FILE := file:rocksdb
SURREAL_PROCESS_NAME := surreal

# Source extensions and directories to exclude for loc
SRC_EXT := rs,ts
EXCLUDE_DIRS := target,node_modules

export RUSTFLAGS := -Awarnings
# Run Commands

## Run Desktop Application
.PHONY: run-desktop
run-desktop:
	@cd $(DESKTOP_DIR) && $(PNPM) tauri dev

## Run Desktop Application in Web Mode
.PHONY: run-desktop-web
run-desktop-web:
	@cd $(DESKTOP_DIR) && $(PNPM) vite dev

## Run Storybook
.PHONY: run-storybook
run-storybook:
	@cd $(STORYBOOK_DIR) && $(PNPM) dev

## Run Documentation
.PHONY: run-docs
run-docs:
	@cd $(DOCS_DIR) && $(PNPM) dev

## Run Web Application
.PHONY: run-web
run-web:
	@cd $(WEB_DIR) && $(PNPM) dev

# Database Commands

## Start the Database
.PHONY: run-database
run-database:
ifeq ($(DETECTED_OS),Windows)
	@cd $(DESKTOP_DIR) && start /B $(SURREAL) start $(DATABASE_FILE)
else
	@cd $(DESKTOP_DIR) && $(SURREAL) start $(DATABASE_FILE) &
endif

## Stop the Database
.PHONY: stop-database
stop-database:
ifeq ($(DETECTED_OS),Windows)
	@taskkill /IM $(SURREAL_PROCESS_NAME).exe /F
else
	@pkill -x $(SURREAL_PROCESS_NAME)
endif

## Check if the database is running, if not, start it in the background
.PHONY: check-db
check-db:
ifeq ($(DETECTED_OS),Windows)
	@tasklist /FI "IMAGENAME eq $(SURREAL_PROCESS_NAME).exe" | find /I "$(SURREAL_PROCESS_NAME).exe" > NUL
	@if errorlevel 1 ($(MAKE) run-database)
else
	@if ! pgrep -x "$(SURREAL_PROCESS_NAME)" > /dev/null; then \
		$(MAKE) run-database; \
	fi
endif

# Generation Commands

## Generate Themes
.PHONY: gen-themes
gen-themes:
	@cd $(THEME_GENERATOR_DIR) && $(PNPM) start

## Generate Icons
.PHONY: gen-icons
gen-icons:
	@cd $(ICONS_DIR) && $(PNPM) build

## Generate HTML Models
.PHONY: gen-html-models
gen-html-models:
	@$(CARGO) test --manifest-path $(HTML_MODELS_DIR)/Cargo.toml
	@$(CARGO) build --manifest-path $(HTML_MODELS_DIR)/Cargo.toml

## Generate UI Kit Models
.PHONY: gen-uikit-models
gen-uikit-models:
	@$(CARGO) test --manifest-path $(UIKIT_MODELS_DIR)/Cargo.toml
	@$(CARGO) build --manifest-path $(UIKIT_MODELS_DIR)/Cargo.toml

## Generate Desktop Models
.PHONY: gen-desktop-models
gen-desktop-models:
	@$(CARGO) test --manifest-path $(DESKTOP_MODELS_DIR)/Cargo.toml
	@$(CARGO) build --manifest-path $(DESKTOP_MODELS_DIR)/Cargo.toml

## Generate All Models
.PHONY: gen-models
gen-models: gen-html-models gen-uikit-models gen-desktop-models

# Utility Commands

## Count Lines of Code
.PHONY: loc
loc:
	@cloc --exclude-dir=$(EXCLUDE_DIRS) --include-ext=$(SRC_EXT) .

## Clean up merged Git branches except master, main, and dev
.PHONY: cleanup-git
cleanup-git:
ifeq ($(DETECTED_OS),Windows)
	# TODO: make this work on Windows
	# @for /F "tokens=*" %i in ('git branch --merged ^| findstr /V "master main dev"') do git branch -d %i
else
	@git branch --merged | grep -Ev "(^\*|master|main|dev)" | xargs git branch -d
endif

# Clean up unused pnpm packages in all directories and store
# pnpm does not support recursive prune
.PHONY: clean-pnpm
clean-pnpm:
	@cd $(DESKTOP_DIR) && $(PNPM) prune
	@cd $(STORYBOOK_DIR) && $(PNPM) prune
	@cd $(DOCS_DIR) && $(PNPM) prune
	@cd $(WEB_DIR) && $(PNPM) prune
	@cd $(THEME_GENERATOR_DIR) && $(PNPM) prune
	@cd $(ICONS_DIR) && $(PNPM) prune
	@cd $(DESKTOP_MODELS_DIR) && $(PNPM) prune
	@cd $(SHARED_MODELS_DIR) && $(PNPM) prune
	$(PNPM) store prune

# Clean up various artifacts across the project
.PHONY: clean
clean: cleanup-git clean-pnpm

# Generate license with xtask
.PHONY: gen-license
gen-license:
	@cd $(XTASK_DIR) && $(CARGO) run license

# Audit workspace dependency
.PHONY: workspace-audit
workspace-audit:
	@cd $(XTASK_DIR) && $(CARGO) run rwa

# Check unused dependency
.PHONY: check-unused-deps
check-unused-deps:
	$(CARGO) --quiet install cargo-udeps --locked
	$(RUSTUP) --quiet toolchain install nightly
	$(CARGO) +nightly udeps --quiet

# Runs a series of maintenance tasks to keep the project organized and up-to-date.
# TODO: output workspace-audit and check-unused-deps to file
.PHONY: tidy

tidy: gen-license workspace-audit check-unused-deps
	$(MAKE) clean

