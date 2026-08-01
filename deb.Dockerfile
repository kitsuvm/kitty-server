FROM rust:1.97.1-slim-trixie
RUN apt-get update && apt-get install -y dpkg-dev
RUN cargo install cargo-deb
WORKDIR /app
COPY . /app
RUN chmod +x /app/ssh/debian/postinst
RUN chmod +x /app/ssh/debian/postrm
RUN cargo deb --package kitty-server-daemon --output /app/output
RUN cargo deb --package kitty-server-ssh --output /app/output
