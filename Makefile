.PHONY: torus-smooth
torus-smooth:
	cargo run --release -- --width 500 --height 500 --scene scenes/torus-smooth.toml

.PHONY: torus-smooth-debug
torus-smooth-debug:
	cargo run -- --width 500 --height 500 --scene scenes/torus-smooth.toml

.PHONY: torus-flat
torus-flat:
	cargo run --release -- --width 500 --height 500 --scene scenes/torus-flat.toml --shading flat

.PHONY: torus-flat-debug
torus-flat-debug:
	cargo run -- --width 500 --height 500 --scene scenes/torus-flat.toml --shading flat

.PHONY: nefertiti
nefertiti:
	cargo run --release -- --width 800 --height 800 --scene scenes/nefertiti.toml

.PHONY: commit
commit: message ?= $(shell git diff --name-only --cached | sed -r 's;([^ /]+/)+([^/ ]+);\2;g')
commit:
	test -n "$(message)"
	git commit -m "$(message)"
