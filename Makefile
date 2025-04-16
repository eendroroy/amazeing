.PHONY: install uninstall

# Determine installation directories based on platform
PREFIX ?= /usr/local
UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Darwin)
	MANDIR ?= $(PREFIX)/share/man/man1
else ifeq ($(UNAME_S),Linux)
	MANDIR ?= $(PREFIX)/share/man/man1
else
	MANDIR ?= $(PREFIX)/man/man1
endif

install:
	@echo "Installing amazeing to $(PREFIX)/bin"
	cargo install --path amazeing

	@echo "Installing manpages to $(MANDIR)"
	@mkdir -p $(MANDIR)
	@install -m 644 contrib/man/amazeing.1 $(MANDIR)/amazeing.1
	@install -m 644 contrib/man/amazeing-create.1 $(MANDIR)/amazeing-create.1
	@install -m 644 contrib/man/amazeing-view.1 $(MANDIR)/amazeing-view.1
	@install -m 644 contrib/man/amazeing-solve.1 $(MANDIR)/amazeing-solve.1

uninstall:
	@echo "Removing amazeing from $(PREFIX)/bin"
	cargo uninstall amazeing

	@echo "Removing manpages from $(MANDIR)"
	@rm -f $(MANDIR)/amazeing.1
	@rm -f $(MANDIR)/amazeing-create.1
	@rm -f $(MANDIR)/amazeing-view.1
	@rm -f $(MANDIR)/amazeing-solve.1
