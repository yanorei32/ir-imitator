FROM rust:1.89.0-bookworm AS build-env
LABEL maintainer="yanorei32"

SHELL ["/bin/bash", "-o", "pipefail", "-c"]

WORKDIR /usr/src
COPY . /usr/src/ir-imitator/
WORKDIR /usr/src/ir-imitator
RUN cargo build --release && cargo install cargo-license && cargo license \
	--authors \
	--do-not-bundle \
	--avoid-dev-deps \
	--avoid-build-deps \
	--filter-platform "$(rustc -vV | sed -n 's|host: ||p')" \
	> CREDITS

FROM debian:bookworm-slim@sha256:df52e55e3361a81ac1bead266f3373ee55d29aa50cf0975d440c2be3483d8ed3

WORKDIR /

COPY --chown=root:root --from=build-env \
	/usr/src/ir-imitator/CREDITS \
	/usr/src/ir-imitator/LICENSE \
	/usr/share/licenses/ir-imitator/

COPY --chown=root:root --from=build-env \
	/usr/src/ir-imitator/target/release/web-ir-remote \
	/usr/bin/web-ir-remote

CMD ["/usr/bin/web-ir-remote"]
