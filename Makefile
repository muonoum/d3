sponza:
	cargo run --release --package d3 -- --scene scenes/sponza.toml --camera-light --scale 2 --width 800

sponza-fullscreen:
	cargo run --release --package d3 -- --scene scenes/sponza.toml --camera-light --scale 4 --fullscreen

dice:
	cargo run --release --package d3  -- --scene scenes/dice.toml

.PHONY: commit
commit: message ?= $(shell git diff --name-only --cached | sed -r 's;([^ /]+/)+([^/ ]+);\2;g')
commit:
	test -n "$(message)"
	git commit -m "$(message)"
