FROM rust:1.92.0-bookworm AS build-env
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

FROM debian:bookworm-slim@sha256:56ff6d36d4eb3db13a741b342ec466f121480b5edded42e4b7ee850ce7a418ee

WORKDIR /

COPY --chown=root:root --from=build-env \
	/usr/src/ir-imitator/CREDITS \
	/usr/src/ir-imitator/LICENSE \
	/usr/share/licenses/ir-imitator/

COPY --chown=root:root --from=build-env \
	/usr/src/ir-imitator/target/release/web-ir-remote \
	/usr/bin/web-ir-remote

CMD ["/usr/bin/web-ir-remote"]
