dice:
	RUST_LOG=d3 cargo run --release -- --scene scenes/dice.toml

.PHONY: commit
commit: message ?= $(shell git diff --name-only --cached | sed -r 's;([^ /]+/)+([^/ ]+);\2;g')
commit:
	test -n "$(message)"
	git commit -m "$(message)"
