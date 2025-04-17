.PHONY: install install-manpages install-completions uninstall uninstall-manpages uninstall-completions clean-residue

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
	@echo "Installing amazeing bin ......"
	@cargo install --path amazeing --quiet

	@echo
	@$(MAKE) install-manpages

	@echo
	@$(MAKE) install-completions

install-manpages:
	@echo "Installing manpages ......"
	@echo "    $(MAN_DIR)"
	@mkdir -p $(MAN_DIR)
	@echo "        amazeing.1"
	@install -m 644 contrib/man/amazeing.1        $(MAN_DIR)/amazeing.1
	@echo "        amazeing-create.1"
	@install -m 644 contrib/man/amazeing-create.1 $(MAN_DIR)/amazeing-create.1
	@echo "        amazeing-view.1"
	@install -m 644 contrib/man/amazeing-view.1   $(MAN_DIR)/amazeing-view.1
	@echo "        amazeing-solve.1"
	@install -m 644 contrib/man/amazeing-solve.1  $(MAN_DIR)/amazeing-solve.1

install-completions:
	@echo "Installing shell completions ......"
	@mkdir -p $(ZSH_COMPLETIONS_DIR)
	@mkdir -p $(BASH_COMPLETIONS_DIR)
	@mkdir -p $(FISH_COMPLETIONS_DIR)
	@echo "    $(ZSH_COMPLETIONS_DIR)"
	@echo "        _amazeing"
	@install -m 644 contrib/completions/_amazeing $(ZSH_COMPLETIONS_DIR)/_amazeing
	@echo "    $(BASH_COMPLETIONS_DIR)"
	@echo "        amazeing.bash"
	@install -m 644 contrib/completions/amazeing.bash $(BASH_COMPLETIONS_DIR)/amazeing.bash
	@echo "    $(FISH_COMPLETIONS_DIR)"
	@echo "        amazeing.fish"
	@install -m 644 contrib/completions/amazeing.fish $(FISH_COMPLETIONS_DIR)/amazeing.fish

uninstall:
	@echo "Uninstalling amazeing bin ......"
	@cargo uninstall amazeing --quiet

	@echo
	@$(MAKE) clean-residue

uninstall-manpages:
	@echo "Removing manpages from $(MAN_DIR) ......"
	@echo "    $(MAN_DIR)"
	@echo "        amazeing.1"
	@rm -f $(MAN_DIR)/amazeing.1
	@echo "        amazeing-create.1"
	@rm -f $(MAN_DIR)/amazeing-create.1
	@echo "        amazeing-view.1"
	@rm -f $(MAN_DIR)/amazeing-view.1
	@echo "        amazeing-solve.1"
	@rm -f $(MAN_DIR)/amazeing-solve.1

uninstall-completions:
	@echo "Removing shell completions ......"
	@echo "    $(ZSH_COMPLETIONS_DIR)"
	@echo "        _amazeing"
	@rm -f $(ZSH_COMPLETIONS_DIR)/_amazeing
	@echo "    $(BASH_COMPLETIONS_DIR)"
	@echo "        amazeing.bash"
	@rm -f $(BASH_COMPLETIONS_DIR)/amazeing.bash
	@echo "    $(FISH_COMPLETIONS_DIR)"
	@echo "        amazeing.fish"
	@rm -f $(FISH_COMPLETIONS_DIR)/amazeing.fish

clean-residue:
	@$(MAKE) uninstall-manpages

	@echo
	@$(MAKE) uninstall-completions