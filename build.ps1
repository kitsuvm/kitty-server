param(
    [string]$OutDir = "target/dist"
)

$ContainerOutDir = $OutDir -replace '\\', '/'

Write-Host "==> Building Docker deb image..." -ForegroundColor Cyan
docker build -t kitty-server-deb-builder -f deb.Dockerfile .
if ($LASTEXITCODE -ne 0) {
    Write-Host "Docker deb build failed." -ForegroundColor Red
    exit $LASTEXITCODE
}

# 3. Run the container and build the .deb
Write-Host "==> Collecting from deb container..." -ForegroundColor Cyan
docker run --rm `
    -v "${ContainerOutDir}:/output" `
    kitty-server-deb-builder `
    cp -r /app/output/. /output/

if ($LASTEXITCODE -ne 0) {
    Write-Host "Can't collect from deb container." -ForegroundColor Red
    exit $LASTEXITCODE
}

Write-Host "==> Building Docker arch image..." -ForegroundColor Cyan
docker build -t kitty-server-arch-builder -f arch.Dockerfile .
if ($LASTEXITCODE -ne 0) {
    Write-Host "Docker arch build failed." -ForegroundColor Red
    exit $LASTEXITCODE
}

Write-Host "==> Collecting from arch container..." -ForegroundColor Cyan
docker run --rm `
    -v "${ContainerOutDir}:/output" `
    kitty-server-arch-builder `
    cp -r /app/output/. /output/

if ($LASTEXITCODE -ne 0) {
    Write-Host "Can't collect from arch container." -ForegroundColor Red
    exit $LASTEXITCODE
}

Write-Host "==> Building Docker rpm image..." -ForegroundColor Cyan
docker build -t kitty-server-rpm-builder -f rpm.Dockerfile .
if ($LASTEXITCODE -ne 0) {
    Write-Host "Docker rpm build failed." -ForegroundColor Red
    exit $LASTEXITCODE
}

Write-Host "==> Collecting from rpm container..." -ForegroundColor Cyan
docker run --rm `
    -v "${ContainerOutDir}:/output" `
    kitty-server-rpm-builder `
    cp -r /app/output/. /output/

if ($LASTEXITCODE -ne 0) {
    Write-Host "Can't collect from rpm container." -ForegroundColor Red
    exit $LASTEXITCODE
}

Write-Host "==> Done! Package saved to $OutDir" -ForegroundColor Green

Write-Host "==> Cleaning up Docker images..." -ForegroundColor Cyan
docker rmi kitty-server-deb-builder kitty-server-arch-builder kitty-server-rpm-builder
Write-Host "==> Cleanup complete." -ForegroundColor Green
