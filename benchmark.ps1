$ErrorActionPreference = "Stop"
$timestamp = Get-Date -Format "yyyyMMdd-HHmmss"
$outputFile = "$PSScriptRoot\benchmark_results_$timestamp.txt"
"" | Out-File $outputFile

function Log($message) {
    Write-Host $message
    $message | Out-File -Append $outputFile
}

Log "=== Starting Benchmarks at $(Get-Date) ==="

# ---- RUST BUILD & RUN ----
Log "`n[INFO] Building Rust Benchmark..."
Set-Location ./rust-performance-test

cargo build --release | Out-File -Append $outputFile
if ($LASTEXITCODE -ne 0) { Log "[ERROR] Rust build failed!"; exit 1 }

Log "`n[INFO] Running Rust Benchmark..."
cargo bench | Out-File -Append $outputFile
if ($LASTEXITCODE -ne 0) { Log "[ERROR] Rust benchmark failed!"; exit 1 }

Set-Location .. 

# ---- C++ BUILD & RUN ----
Log "`n[INFO] Building C++ Benchmark..."
Set-Location ./cpp-performance-test

if (!(Test-Path "build")) { mkdir build }
Set-Location build

cmake .. | Out-File -Append $outputFile
if ($LASTEXITCODE -ne 0) { Log "[ERROR] CMake configuration failed!"; exit 1 }

cmake --build . --config Release | Out-File -Append $outputFile
if ($LASTEXITCODE -ne 0) { Log "[ERROR] C++ build failed!"; exit 1 }

Set-Location ./Release

Log "`n[INFO] Running C++ Benchmark..."
./benchmark_test.exe | Out-File -Append $outputFile
if ($LASTEXITCODE -ne 0) { Log "[ERROR] C++ benchmark failed!"; exit 1 }

Set-Location ../../.. 

Log "`n=== Benchmarks Completed at $(Get-Date) ==="

Get-Content $outputFile