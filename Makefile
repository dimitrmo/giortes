VERSION=$(shell cargo pkgid | cut -d\# -f2 | cut -d: -f2)

export RUST_BACKTRACE=1

.PHONY: tag
tag:
	git tag -a v${VERSION}

.PHONY: prod
prod:
	cargo build --release
	ls -lah target/release/giortes
	upx -9 target/release/giortes
	ls -lah target/release/giortes

.PHONY: run
run:
	PORT=9091 \
	RUST_LOG=giortes=debug \
		cargo run

.PHONY: docker-build
docker-build:
	docker build --squash -t giortes:${VERSION} .

.PHONY: docker-push
docker-push:
	docker tag giortes:${VERSION} rg.fr-par.scw.cloud/dimitrmo/giortes:${VERSION}
	docker push rg.fr-par.scw.cloud/dimitrmo/giortes:${VERSION}

.PHONY: docker-run
docker-run:
	docker run --rm -it -p 18080:8080 -e RUST_LOG=giortes=debug giortes:${VERSION}
