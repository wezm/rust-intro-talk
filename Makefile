
all: slides.md

%.md: %.in.md
	sed 's/```rust,.*/```rust/' $< > $@
