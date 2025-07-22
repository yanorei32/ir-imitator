FROM rust:1.88.0-bookworm AS build-env
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

FROM debian:bookworm-slim@sha256:2424c1850714a4d94666ec928e24d86de958646737b1d113f5b2207be44d37d8

WORKDIR /

COPY --chown=root:root --from=build-env \
	/usr/src/ir-imitator/CREDITS \
	/usr/src/ir-imitator/LICENSE \
	/usr/share/licenses/ir-imitator/

COPY --chown=root:root --from=build-env \
	/usr/src/ir-imitator/target/release/web-ir-remote \
	/usr/bin/web-ir-remote

CMD ["/usr/bin/web-ir-remote"]
