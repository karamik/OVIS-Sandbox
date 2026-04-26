# Testing OVIS Implementations

This directory contains test vectors for validating OVIS-compatible implementations. The test suite checks that a given OVIS runtime (e.g., the Rust reference implementation) correctly enforces policies and produces expected green/red outcomes.

## Prerequisites

- Python 3.8+
- Docker (optional, if you want to test the Docker image)
- Or a compiled `ovis-sandbox` binary (from `cargo build --release`)

## Running Tests

### Using the Python test runner

```bash
# Clone the repository (if not already done)
git clone https://github.com/karamik/OVIS-Sandbox.git
cd OVIS-Sandbox

# Run the test script
python3 run_tests.py
