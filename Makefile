VERSION=$(shell cargo pkgid | cut -d\# -f2 | cut -d: -f2)

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
	RUST_LOG=giortes=debug \
		cargo run

.PHONY: docker-build
docker-build:
	docker build --squash -t

.PHONY: docker-push
docker-push:
	docker tag cygaz:${VERSION} rg.fr-par.scw.cloud/dimitrmo/giortes:${VERSION}
	docker push rg.fr-par.scw.cloud/dimitrmo/giortes:${VERSION}

.PHONY: docker-run
docker-run:
	docker run --rm -it -p 18080:8080 -e RUST_LOG=cygaz=debug giortes:${VERSION}
