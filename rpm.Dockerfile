FROM almalinux:10
RUN dnf install -y gcc
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup default stable
RUN cargo install cargo-generate-rpm
WORKDIR /app
COPY . /app
RUN cargo build --release --package kitty-server-app
RUN strip -s target/release/kitty-server-app
RUN cargo generate-rpm -p app
RUN mkdir -p /app/output
RUN cp target/generate-rpm/*.rpm /app/output/
