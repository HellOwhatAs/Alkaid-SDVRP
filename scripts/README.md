# Comparison Scripts

This directory contains scripts for comparing the C++ and Rust implementations of the Alkaid SDVRP solver.

## compare_implementations.py

A comprehensive script to run and compare both implementations on SDVRP instances.

### Installation

No additional dependencies are required - the script uses only Python standard library.

### Usage

```bash
# Run with default settings on all instances
python scripts/compare_implementations.py

# Run on a specific instance set
python scripts/compare_implementations.py --set SET-1

# Run on a specific instance
python scripts/compare_implementations.py --instance data/SET-1/SD1.txt

# Set time limit and random seed
python scripts/compare_implementations.py --time-limit 60 --random-seed 42

# Run only C++ implementation
python scripts/compare_implementations.py --cpp-only

# Run only Rust implementation
python scripts/compare_implementations.py --rust-only

# Export results to CSV
python scripts/compare_implementations.py --output results.csv

# Use custom configuration
python scripts/compare_implementations.py --config custom-config.ini

# Skip building (use existing binaries)
python scripts/compare_implementations.py --no-build

# Limit number of instances
python scripts/compare_implementations.py --max-instances 5
```

### Command-Line Options

| Option | Description |
|--------|-------------|
| `--set NAME` | Run on instances from a specific set (e.g., SET-1, SET-2) |
| `--instance FILE` | Run on a specific instance file |
| `--time-limit SECONDS` | Time limit in seconds (default: 10) |
| `--random-seed INT` | Random seed for reproducibility (default: 42) |
| `--cpp-only` | Run only the C++ implementation |
| `--rust-only` | Run only the Rust implementation |
| `--config FILE` | Path to custom configuration file |
| `--output FILE` | Path to output CSV file for results |
| `--no-build` | Skip building the implementations |
| `--max-instances N` | Maximum number of instances to run |

### Output Format

The script outputs a formatted table showing:
- Instance name
- C++ objective value and runtime
- Rust objective value and runtime
- Objective difference (Rust - C++)
- Runtime ratio (Rust / C++)

### Configuration

You can customize solver parameters using a configuration file (INI format):

```ini
; Example configuration
time-limit = 60
blink-rate = 0.021
inter-operators = ["Relocate", "SwapStar", "SdSwapStar"]
intra-operators = ["Exchange", "OrOpt<1>"]
acceptance-rule-type = "LAHC"
acceptance-rule-args = ["length=83"]
ruin-method-type = "SISRs"
ruin-method-args = ["average_customers=36", "max_length=8", "split_rate=0.740", "preserved_probability=0.096"]
sorters = ["random=0.078", "demand=0.225", "far=0.942", "close=0.120"]
```

### Notes

- The C++ implementation must be built with CMake
- The Rust implementation is built with Cargo
- Build artifacts are placed in `build-compare/` (C++) and `alkaidsd-rs/target/` (Rust)
