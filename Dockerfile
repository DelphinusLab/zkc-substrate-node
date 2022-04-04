FROM docker.io/paritytech/ci-linux:production as builder

WORKDIR /delphinuslab
COPY . /delphinuslab
ENV CARGO_HOME /delphinuslab/.cargo-home
RUN --mount=type=cache,target=/delphinuslab/.cargo-home \
    --mount=type=cache,target=/delphinuslab/packages/substrate-node/target \
    cargo build --manifest-path=/delphinuslab/packages/substrate-node/Cargo.toml --locked --release && install -Dt ./bin/ /delphinuslab/packages/substrate-node/target/release/node-swap

# This is the 2nd stage: a very small image where we copy the delphinuslab node binary."
FROM docker.io/library/ubuntu:20.04
LABEL description="Zhenxunge node"

COPY --from=builder /delphinuslab/bin/ /usr/local/bin/

RUN useradd -m -u 1000 -U -s /bin/sh -d /delphinuslab delphinuslab && \
	mkdir -p /data /delphinuslab/.local/share && \
	chown -R delphinuslab:delphinuslab /data /delphinuslab && \
	ln -s /data /delphinuslab/.local/share/node-swap && \
# Sanity checks
	/usr/local/bin/node-swap --version

USER delphinuslab
EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]
ENTRYPOINT ["/usr/local/bin/node-swap"]
CMD ["--dev", "--ws-external"]
