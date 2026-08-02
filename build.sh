#!/bin/sh

OUTDIR="${1:-target/dist}"

# Ensure the output directory exists on the host
mkdir -p "$OUTDIR"

echo "\033[36m==> Building Docker deb image...\033[0m"
docker build -t kitty-server-deb-builder -f deb.Dockerfile .
if [ $? -ne 0 ]; then
    echo "\033[31mDocker deb build failed.\033[0m"
    exit 1
fi

echo "\033[36m==> Collecting from deb container...\033[0m"
docker run --rm \
    -v "$(pwd)/${OUTDIR}:/output" \
    kitty-server-deb-builder \
    cp -r /app/output/. /output/

if [ $? -ne 0 ]; then
    echo "\033[31mCan't collect from deb container.\033[0m"
    exit 1
fi

echo "\033[36m==> Building Docker arch image...\033[0m"
docker build -t kitty-server-arch-builder -f arch.Dockerfile .
if [ $? -ne 0 ]; then
    echo "\033[31mDocker arch build failed.\033[0m"
    exit 1
fi

echo "\033[36m==> Collecting from arch container...\033[0m"
docker run --rm \
    -v "$(pwd)/${OUTDIR}:/output" \
    kitty-server-arch-builder \
    cp -r /app/output/. /output/

if [ $? -ne 0 ]; then
    echo "\033[31mCan't collect from arch container.\033[0m"
    exit 1
fi

echo "\033[36m==> Building Docker rpm image...\033[0m"
docker build -t kitty-server-rpm-builder -f rpm.Dockerfile .
if [ $? -ne 0 ]; then
    echo "\033[31mDocker rpm build failed.\033[0m"
    exit 1
fi

echo "\033[36m==> Collecting from rpm container...\033[0m"
docker run --rm \
    -v "$(pwd)/${OUTDIR}:/output" \
    kitty-server-rpm-builder \
    cp -r /app/output/. /output/

if [ $? -ne 0 ]; then
    echo "\033[31mCan't collect from rpm container.\033[0m"
    exit 1
fi

echo "\033[32m==> Done! Package saved to $OUTDIR\033[0m"

echo "\033[36m==> Cleaning up Docker images...\033[0m"
docker rmi kitty-server-deb-builder kitty-server-arch-builder kitty-server-rpm-builder
echo "\033[32m==> Cleanup complete.\033[0m"
