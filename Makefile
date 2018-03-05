CARGO = cargo

CARGO_OPTS =

all:
	$(MAKE) run

build:
	$(CARGO) $(CARGO_OPTS) build

clean:
	$(CARGO) $(CARGO_OPTS) clean

check:
	$(MAKE) build
	$(MAKE) test

test:
	$(CARGO) $(CARGO_OPTS) test

bench:
	$(CARGO) $(CARGO_OPTS) bench

doc:
	$(CARGO) $(CARGO_OPTS) doc

run:
	$(CARGO) $(CARGO_OPTS) run

.PHONY: all build clean check test bench doc run
