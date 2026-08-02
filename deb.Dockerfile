FROM debian:trixie-slim
RUN apt-get update && apt-get upgrade -y && apt-get install -y dpkg-dev curl
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup default stable
RUN cargo install cargo-deb
RUN cargo install cargo-packager
WORKDIR /app
COPY . /app
RUN chmod +x /app/ssh/debian/postinst
RUN chmod +x /app/ssh/debian/postrm
RUN cargo deb --package kitty-server-daemon --output /app/output
RUN cargo deb --package kitty-server-ssh --output /app/output
RUN cargo packager --formats deb --release --packages kitty-server-app --out-dir /app/output
RUN rm -r /app/output/.cargo-packager
