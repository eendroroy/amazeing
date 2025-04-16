.PHONY: install uninstall

# Determine installation directories based on platform
PREFIX ?= /usr/local
UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Darwin)
 MAN_DIR ?= $(PREFIX)/share/man/man1
 BASH_COMPLETIONS_DIR ?= $(PREFIX)/etc/bash_completion.d
 ZSH_COMPLETIONS_DIR ?= $(PREFIX)/share/zsh/site-functions
 FISH_COMPLETIONS_DIR ?= $(PREFIX)/share/fish/vendor_completions.d
else ifeq ($(UNAME_S),Linux)
 MAN_DIR ?= $(PREFIX)/share/man/man1
 BASH_COMPLETIONS_DIR ?= $(PREFIX)/share/bash-completion/completions
 ZSH_COMPLETIONS_DIR ?= $(PREFIX)/share/zsh/vendor-completions
 FISH_COMPLETIONS_DIR ?= $(PREFIX)/share/fish/vendor_completions.d
else
 MAN_DIR ?= $(PREFIX)/man/man1
 BASH_COMPLETIONS_DIR ?= $(PREFIX)/etc/bash_completion.d
 ZSH_COMPLETIONS_DIR ?= $(PREFIX)/share/zsh/site-functions
 FISH_COMPLETIONS_DIR ?= $(PREFIX)/share/fish/vendor_completions.d
endif

install:
	@echo "Installing amazeing"
	cargo install --path amazeing

	@echo "Installing man-pages to $(MAN_DIR)"
	@mkdir -p $(MAN_DIR)
	@install -m 644 contrib/man/amazeing.1        $(MAN_DIR)/amazeing.1
	@install -m 644 contrib/man/amazeing-create.1 $(MAN_DIR)/amazeing-create.1
	@install -m 644 contrib/man/amazeing-view.1   $(MAN_DIR)/amazeing-view.1
	@install -m 644 contrib/man/amazeing-solve.1  $(MAN_DIR)/amazeing-solve.1

	@echo "Installing shell completions"
	@mkdir -p $(BASH_COMPLETIONS_DIR)
	@mkdir -p $(ZSH_COMPLETIONS_DIR)
	@mkdir -p $(FISH_COMPLETIONS_DIR)
	@install -m 644 contrib/completions/amazeing.bash $(BASH_COMPLETIONS_DIR)/amazeing.bash
	@install -m 644 contrib/completions/_amazeing     $(ZSH_COMPLETIONS_DIR)/_amazeing
	@install -m 644 contrib/completions/amazeing.fish $(FISH_COMPLETIONS_DIR)/amazeing.fish


uninstall:
	@echo "Uninstalling amazeing"
	cargo uninstall amazeing

	@echo "Removing man-pages from $(MAN_DIR)"
	@rm -f $(MAN_DIR)/amazeing.1
	@rm -f $(MAN_DIR)/amazeing-create.1
	@rm -f $(MAN_DIR)/amazeing-view.1
	@rm -f $(MAN_DIR)/amazeing-solve.1

	@echo "Removing shell completions"
	@rm -f $(ZSH_COMPLETIONS_DIR)/_amazeing
	@rm -f $(BASH_COMPLETIONS_DIR)/amazeing.bash
	@rm -f $(FISH_COMPLETIONS_DIR)/amazeing.fish

