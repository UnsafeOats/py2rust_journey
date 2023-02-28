echo "Running Python implementations...\n"
python3 python/main.py

echo "\n\nRunning Rust implementations...\n"
cd rust
cargo run --quiet
cd ..

echo "\n\nRunning Go implementations...\n"
cd go
go run .
cd ..
