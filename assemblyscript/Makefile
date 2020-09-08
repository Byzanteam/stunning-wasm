ENTRIES := $(shell find src/packages -type f -name index.ts)
FILES := ${ENTRIES:src/packages/%/index.ts=dist/%.wasm}

all: ${FILES}

${FILES}:
	@echo Compiling $@ ...
	@npx asc $(patsubst dist/%.wasm,src/packages/%/index.ts,$@) -b $@

clean:
	@echo Cleanup artifacts ...
	@rm -rf dist

.PHONY: all
