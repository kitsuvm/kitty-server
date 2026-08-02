FROM archlinux:latest
RUN pacman -Syu --noconfirm && \
    pacman -S --noconfirm rustup gcc sudo fakeroot debugedit && \
    useradd -m -g users -G wheel builder && \
    echo "builder ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers
RUN rustup default stable
RUN cargo install cargo-packager
WORKDIR /app
COPY . /app
RUN cargo packager --formats pacman --release --packages kitty-server-app --out-dir /app/archpkg
RUN mkdir -p /app/output
RUN cp -r /app/archpkg /app/output/archpkg
RUN chmod -R o+rwx /app/archpkg
RUN su builder -c "cd /app/archpkg && makepkg -s --noconfirm"
RUN cp /app/archpkg/*.pkg.tar.zst /app/output/
RUN rm -r /app/output/archpkg/.cargo-packager
