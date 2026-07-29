.PHONY: test run

HOSTINGS := beeimg catbox cloudinary fastpic filepost freeimage gofile gyazo imageban imagekit imgbb imgbox imgchest imghippo imglink imgur kappa lensdump pixeldrain pixhost pixvid postimages redacted sxcu thumbsnap tixte uplio uploadcare vgy zpic
TEST_IMG := tests/fixtures/image.png

run:
	cargo run --quiet -- --help

test:
	@cargo build --quiet
	@for h in $(HOSTINGS); do \
		printf "%-12s " "$$h:"; \
		./target/debug/imgup --hosting $$h --no-clipboard $(TEST_IMG); \
	done
