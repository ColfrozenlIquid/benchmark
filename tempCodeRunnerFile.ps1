# ---- RUST BUILD & RUN ----
# Log "`n[INFO] Building Rust Benchmark..."
# Set-Location ./rust-performance-test

# cargo build --release | Out-File -Append $outputFile
# if ($LASTEXITCODE -ne 0) { Log "[ERROR] Rust build failed!"; exit 1 }

# Log "`n[INFO] Running Rust Benchmark..."
# cargo bench | Out-File -Append $outputFile
# if ($LASTEXITCODE -ne 0) { Log "[ERROR] Rust benchmark failed!"; exit 1 }

# Set-Location .. 