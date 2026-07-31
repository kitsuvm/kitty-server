param(
    # Defaults to target/debian if no argument is provided
    [string]$OutDir = "target/debian"
)

# 1. Convert Windows backslashes to Linux forward slashes for the container
$ContainerOutDir = $OutDir -replace '\\', '/'

# 2. Build the Docker environment
Write-Host "==> Building Docker image..." -ForegroundColor Cyan
docker build -t kitty-server-deb-builder -f deb.Dockerfile .
if ($LASTEXITCODE -ne 0) {
    Write-Host "Docker build failed." -ForegroundColor Red
    exit $LASTEXITCODE
}

# 3. Run the container and build the .deb
Write-Host "==> Building .deb inside container..." -ForegroundColor Cyan
docker run --rm `
    -v "${ContainerOutDir}:/output" `
    kitty-server-deb-builder `
    cp -r /app/output/. /output/

if ($LASTEXITCODE -ne 0) {
    Write-Host "Cargo deb build failed." -ForegroundColor Red
    exit $LASTEXITCODE
}

Write-Host "==> Done! Package saved to $OutDir" -ForegroundColor Green
