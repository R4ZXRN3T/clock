.PHONY: build clean

UNAME_S := $(shell uname -s)

build:
ifeq ($(UNAME_S),Linux)
	@bash scripts/build-linux.sh
else ifeq ($(UNAME_S),Darwin)
	@bash scripts/build-macos.sh
else ifeq ($(OS),Windows_NT)
	@powershell -ExecutionPolicy Bypass -File scripts/build-windows.ps1
else
	@echo "Unsupported operating system"
	@exit 1
endif

clean:
	rm -rf ./final ./target
