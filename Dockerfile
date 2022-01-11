FROM docker.io/paritytech/ci-linux:production as builder

WORKDIR /zhenxunge
COPY . /zhenxunge
ENV CARGO_HOME /zhenxunge/.cargo-home
RUN --mount=type=cache,target=/zhenxunge/.cargo-home \
    --mount=type=cache,target=/zhenxunge/target \
    cargo build --locked --release && install -Dt ./bin/ target/release/node-swap

# This is the 2nd stage: a very small image where we copy the zhenxunge node binary."
FROM docker.io/library/ubuntu:20.04
LABEL description="Zhenxunge node"

COPY --from=builder /zhenxunge/bin/ /usr/local/bin/

RUN useradd -m -u 1000 -U -s /bin/sh -d /zhenxunge zhenxunge && \
	mkdir -p /data /zhenxunge/.local/share && \
	chown -R zhenxunge:zhenxunge /data /zhenxunge && \
	ln -s /data /zhenxunge/.local/share/node-swap && \
# Sanity checks
	/usr/local/bin/node-swap --version

USER zhenxunge
EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]
ENTRYPOINT ["/usr/local/bin/node-swap"]
CMD ["--dev", "--ws-external"]
