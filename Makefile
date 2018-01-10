all: slides.md

deploy:
	rsync -avz images remark-latest.min.js slides.html slides.md eforce.binarytrance.com:/usr/local/www/www.wezm.net/temp/rust-intro-talk/

%.md: %.in.md
	sed 's/```rust,.*/```rust/' $< > $@

# .phony: deploy
