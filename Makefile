.PHONY: torus-smooth
torus-smooth:
	cargo run --release -- --width 500 --height 500 --mesh objs/torus-smooth.obj --rotate

.PHONY: torus-smooth-debug
torus-smooth-debug:
	cargo run -- --width 500 --height 500 --mesh objs/torus-smooth.obj --rotate

.PHONY: torus-flat
torus-flat:
	cargo run --release -- --width 500 --height 500 --mesh objs/torus-flat.obj --shading flat --rotate

.PHONY: torus-flat-debug
torus-flat-debug:
	cargo run -- --width 500 --height 500 --mesh objs/torus-flat.obj --shading flat --rotate

.PHONY: nefertiti
nefertiti:
	cargo run --release -- --width 800 --height 800 --mesh objs/nefertiti.obj

.PHONY: commit
commit: message ?= $(shell git diff --name-only --cached | sed -r 's;([^ /]+/)+([^/ ]+);\2;g')
commit:
	test -n "$(message)"
	git commit -m "$(message)"
