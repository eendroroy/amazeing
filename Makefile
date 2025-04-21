.PHONY: install install-completions uninstall uninstall-completions clean-residue

# Determine installation directories based on platform
PREFIX ?= /usr/local
UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Darwin)
 BASH_COMPLETIONS_DIR ?= $(PREFIX)/etc/bash_completion.d
 ZSH_COMPLETIONS_DIR ?= $(PREFIX)/share/zsh/site-functions
 FISH_COMPLETIONS_DIR ?= $(PREFIX)/share/fish/vendor_completions.d
else ifeq ($(UNAME_S),Linux)
 BASH_COMPLETIONS_DIR ?= $(PREFIX)/share/bash-completion/completions
 ZSH_COMPLETIONS_DIR ?= $(PREFIX)/share/zsh/vendor-completions
 FISH_COMPLETIONS_DIR ?= $(PREFIX)/share/fish/vendor_completions.d
else
 BASH_COMPLETIONS_DIR ?= $(PREFIX)/etc/bash_completion.d
 ZSH_COMPLETIONS_DIR ?= $(PREFIX)/share/zsh/site-functions
 FISH_COMPLETIONS_DIR ?= $(PREFIX)/share/fish/vendor_completions.d
endif

install:
	@echo "Installing amazeing bin ......"
	@cargo install --path amazeing --quiet

	@echo
	@$(MAKE) install-completions

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
	@$(MAKE) uninstall-completions