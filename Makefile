PREFIX ?= /usr/local
BIN_NAME = sloth

RELEASE_BIN_PATH = target/release/$(BIN_NAME)
DEBUG_BIN_PATH = target/debug/$(BIN_NAME)
INSTALL_BIN_PATH = $(PREFIX)/bin/$(BIN_NAME)

debug: $(DEBUG_BIN_PATH)

release: $(RELEASE_BIN_PATH)

install: $(INSTALL_BIN_PATH)

all: release debug

clean:
	cargo clean

.PHONY: debug release install all clean

$(INSTALL_BIN_PATH): $(RELEASE_BIN_PATH) | $(PREFIX)/bin
	install -T "$<" "$@"

$(RELEASE_BIN_PATH): src/*
	cargo build --release

$(DEBUG_BIN_PATH): src/*
	cargo build

$(PREFIX)/bin:
	mkdir -p $@
