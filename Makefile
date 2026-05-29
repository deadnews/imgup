.PHONY: all clean default run build update up check lint pc test release

default: check

check: pc lint test
pc:
	prek run -a
lint:
	cargo fmt --all
	cargo clippy --fix --allow-dirty --all-targets -- -D warnings
lint-ci:
	cargo fmt --all --check
	cargo clippy --all-targets -- -D warnings
test:
	cargo llvm-cov nextest
test-ci: test
	cargo llvm-cov report --lcov --output-path lcov.info

update: up up-ci
up:
	cargo update --recursive --verbose
	cargo upgrade --incompatible
up-ci:
	prek auto-update --freeze
	pinact run --update

build:
	cargo build

bumped:
	git cliff --bumped-version

# make release TAG=$(git cliff --bumped-version)-alpha.0
release: check
	git cliff -o CHANGELOG.md --tag $(TAG)
	prek run --files CHANGELOG.md || prek run --files CHANGELOG.md
	git add CHANGELOG.md
	git commit -m "chore(release): prepare for $(TAG)"
	git push
	git tag -a $(TAG) -m "chore(release): $(TAG)"
	git push origin $(TAG)
