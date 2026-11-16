
FROM rust:1-bookworm

ENV CARGO_HOME=/usr/local/cargo
ENV PATH="${CARGO_HOME}/bin:${PATH}"

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    pkg-config \
    libssl-dev \
    git \
    curl \
    #    ca-certificates \
    #    vim \
    && rm -rf /var/lib/apt/lists/*

RUN rustup component add rustfmt clippy rust-analyzer

WORKDIR /modelApp

CMD ["bash"]
