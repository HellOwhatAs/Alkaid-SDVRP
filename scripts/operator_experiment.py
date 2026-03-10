#!/usr/bin/env python3
"""
Operator experiment script for debugging divergence between C++ and Rust.

Tests each inter-route operator individually and in combinations to identify
which operator(s) cause divergence between implementations.

Usage:
    python scripts/operator_experiment.py [OPTIONS]

Examples:
    # Test all operators individually on SD3
    python scripts/operator_experiment.py --instance data/SET-1/SD3.txt

    # Test specific operators
    python scripts/operator_experiment.py --operators Relocate SwapStar

    # Test with longer time limit
    python scripts/operator_experiment.py --time-limit 5.0

    # Test on multiple instances
    python scripts/operator_experiment.py --set SET-1

    # Test with custom intra-operators
    python scripts/operator_experiment.py --intra-operators Exchange "OrOpt<1>"
"""

import argparse
import os
import subprocess
import sys
import tempfile
from itertools import combinations
from pathlib import Path


# All available inter-route operators
ALL_INTER_OPERATORS = [
    "Relocate",
    "Swap<2, 0>",
    "Swap<2, 1>",
    "Swap<2, 2>",
    "Cross",
    "SwapStar",
    "SdSwapStar",
    "SdSwapOneOne",
    "SdSwapTwoOne",
]

# Default intra-route operators
DEFAULT_INTRA_OPERATORS = ["Exchange", "OrOpt<1>"]

# Default config values
DEFAULT_CONFIG = {
    "random-seed": 42,
    "blink-rate": 0.021,
    "acceptance-rule-type": "LAHC",
    "acceptance-rule-args": '["length=83"]',
    "ruin-method-type": "SISRs",
    "ruin-method-args": '["average_customers=36", "max_length=8", "split_rate=0.740", "preserved_probability=0.096"]',
    "sorters": '["random=0.078", "demand=0.225", "far=0.942", "close=0.120"]',
}


def build_implementations(repo_root):
    """Build both C++ and Rust implementations."""
    cpp_build_dir = repo_root / "build-compare"
    cpp_build_dir.mkdir(exist_ok=True)

    # Build C++
    print("Building C++ implementation...")
    subprocess.run(
        ["cmake", "-S", str(repo_root / "standalone"), "-B", str(cpp_build_dir),
         "-DCMAKE_BUILD_TYPE=Release"],
        check=True, capture_output=True, text=True, encoding='utf-8', errors='replace'
    )
    subprocess.run(
        ["cmake", "--build", str(cpp_build_dir), "--config", "Release", "-j"],
        check=True, capture_output=True, text=True, encoding='utf-8', errors='replace'
    )
    cpp_exe = cpp_build_dir / "AlkaidSD"
    print(f"  C++ build successful: {cpp_exe}")

    # Build Rust
    print("Building Rust implementation...")
    subprocess.run(
        ["cargo", "build", "--release", "--bin", "benchmark"],
        cwd=str(repo_root / "alkaidsd-rs"),
        check=True, capture_output=True, text=True, encoding='utf-8', errors='replace'
    )
    rust_exe = repo_root / "alkaidsd-rs" / "target" / "release" / "benchmark"
    print(f"  Rust build successful: {rust_exe}")

    return cpp_exe, rust_exe


def run_solver(exe, config_path, timeout=120):
    """Run a solver with the given config and return (objective, time) or None on error."""
    try:
        result = subprocess.run(
            [str(exe), f"--config={config_path}"],
            capture_output=True, text=True, timeout=timeout,
            encoding='utf-8', errors='replace'
        )
        # Parse output to get final objective
        for line in result.stdout.strip().split('\n'):
            if 'End at' in line:
                parts = line.split()
                obj = int(parts[-1].rstrip(':'))
                time_str = parts[2].rstrip('s:')
                return obj, float(time_str)
        return None, None
    except (subprocess.TimeoutExpired, Exception) as e:
        return None, None


def format_operators(operators):
    """Format operator list for config file."""
    return "[" + ", ".join(f'"{op}"' for op in operators) + "]"


def run_experiment(cpp_exe, rust_exe, instance_path, inter_operators,
                   intra_operators, time_limit, config_overrides=None):
    """Run a single experiment with the given operator configuration."""
    with tempfile.NamedTemporaryFile(mode='w', suffix='.ini', delete=False) as f:
        config = dict(DEFAULT_CONFIG)
        if config_overrides:
            config.update(config_overrides)

        f.write(f"input = {instance_path}\n")
        f.write(f"output = {os.path.join(tempfile.gettempdir(), 'experiment_solution.txt')}\n")
        f.write(f"random-seed = {config['random-seed']}\n")
        f.write(f"time-limit = {time_limit}\n")
        f.write(f"blink-rate = {config['blink-rate']}\n")
        f.write(f"inter-operators = {format_operators(inter_operators)}\n")
        f.write(f"intra-operators = {format_operators(intra_operators)}\n")
        f.write(f"acceptance-rule-type = {config['acceptance-rule-type']}\n")
        f.write(f"acceptance-rule-args = {config['acceptance-rule-args']}\n")
        f.write(f"ruin-method-type = {config['ruin-method-type']}\n")
        f.write(f"ruin-method-args = {config['ruin-method-args']}\n")
        f.write(f"sorters = {config['sorters']}\n")
        config_path = f.name

    cpp_obj, cpp_time = run_solver(cpp_exe, config_path)
    rust_obj, rust_time = run_solver(rust_exe, config_path)

    os.unlink(config_path)

    return cpp_obj, cpp_time, rust_obj, rust_time


def main():
    parser = argparse.ArgumentParser(
        description="Operator experiment for debugging C++/Rust divergence"
    )
    parser.add_argument("--instance", help="Specific instance file to test")
    parser.add_argument("--set", help="Instance set (e.g., SET-1)")
    parser.add_argument("--operators", nargs="+",
                        help="Specific operators to test (default: all)")
    parser.add_argument("--intra-operators", nargs="+",
                        default=DEFAULT_INTRA_OPERATORS,
                        help="Intra-route operators to use")
    parser.add_argument("--time-limit", type=float, default=1.0,
                        help="Time limit per run in seconds (default: 1.0). "
                             "Use 5-10s for convergence testing; 0.5-1s for quick divergence checks")
    parser.add_argument("--combinations", action="store_true",
                        help="Also test operator combinations")
    parser.add_argument("--max-combo-size", type=int, default=3,
                        help="Maximum operator combination size (default: 3)")
    parser.add_argument("--seed", type=int, default=42,
                        help="Random seed (default: 42)")

    args = parser.parse_args()

    # Determine repo root
    script_dir = Path(__file__).parent
    repo_root = script_dir.parent

    # Determine instances
    instances = []
    if args.instance:
        instances.append(Path(args.instance).resolve())
    elif args.set:
        data_dir = repo_root / "data" / args.set
        instances = sorted(data_dir.glob("*.txt"))
    else:
        # Default: test on a few instances of varying size
        for name in ["SD3.txt", "SD5.txt", "SD10.txt"]:
            p = repo_root / "data" / "SET-1" / name
            if p.exists():
                instances.append(p)

    if not instances:
        print("No instances found!")
        sys.exit(1)

    # Determine operators to test
    test_operators = args.operators if args.operators else ALL_INTER_OPERATORS

    # Build implementations
    cpp_exe, rust_exe = build_implementations(repo_root)

    # Run experiments
    print(f"\nTesting {len(test_operators)} operators on {len(instances)} instance(s)")
    print(f"Time limit: {args.time_limit}s, Seed: {args.seed}")
    print(f"Intra operators: {args.intra_operators}")
    print("=" * 100)

    # Phase 1: Individual operators
    print("\n--- Phase 1: Individual Operators ---\n")
    print(f"{'Operator':<20} {'Instance':<15} {'C++ Obj':>10} {'Rust Obj':>10} {'Diff':>8} {'Match':>6}")
    print("-" * 75)

    results = {}
    for instance_path in instances:
        instance_name = instance_path.name
        for op in test_operators:
            cpp_obj, _, rust_obj, _ = run_experiment(
                cpp_exe, rust_exe, str(instance_path),
                [op], args.intra_operators, args.time_limit,
                {"random-seed": args.seed}
            )

            if cpp_obj is not None and rust_obj is not None:
                diff = rust_obj - cpp_obj
                match = "✓" if diff == 0 else "✗"
                print(f"{op:<20} {instance_name:<15} {cpp_obj:>10} {rust_obj:>10} {diff:>8} {match:>6}")
                results[(op, instance_name)] = (cpp_obj, rust_obj, diff)
            else:
                print(f"{op:<20} {instance_name:<15} {'ERROR':>10} {'ERROR':>10} {'N/A':>8} {'N/A':>6}")

    # Phase 2: Operator combinations (optional)
    if args.combinations:
        print(f"\n--- Phase 2: Operator Combinations (up to {args.max_combo_size}) ---\n")
        print(f"{'Operators':<50} {'Instance':<15} {'C++ Obj':>10} {'Rust Obj':>10} {'Diff':>8}")
        print("-" * 100)

        for instance_path in instances:
            instance_name = instance_path.name
            for size in range(2, min(args.max_combo_size + 1, len(test_operators) + 1)):
                for combo in combinations(test_operators, size):
                    combo_name = " + ".join(combo)
                    if len(combo_name) > 47:
                        combo_name = combo_name[:44] + "..."

                    cpp_obj, _, rust_obj, _ = run_experiment(
                        cpp_exe, rust_exe, str(instance_path),
                        list(combo), args.intra_operators, args.time_limit,
                        {"random-seed": args.seed}
                    )

                    if cpp_obj is not None and rust_obj is not None:
                        diff = rust_obj - cpp_obj
                        print(f"{combo_name:<50} {instance_name:<15} {cpp_obj:>10} {rust_obj:>10} {diff:>8}")

    # Summary
    print("\n" + "=" * 100)
    print("SUMMARY")
    print("=" * 100)

    matching = sum(1 for _, _, d in results.values() if d == 0)
    diverging = sum(1 for _, _, d in results.values() if d != 0)
    total = len(results)

    print(f"Total experiments: {total}")
    print(f"Matching: {matching} ({100*matching/total:.1f}%)" if total > 0 else "")
    print(f"Diverging: {diverging} ({100*diverging/total:.1f}%)" if total > 0 else "")

    if diverging > 0:
        print("\nDiverging operators per instance:")
        for instance_path in instances:
            instance_name = instance_path.name
            div_ops = [op for op in test_operators
                       if (op, instance_name) in results
                       and results[(op, instance_name)][2] != 0]
            if div_ops:
                avg_diff = sum(abs(results[(op, instance_name)][2]) for op in div_ops) / len(div_ops)
                print(f"  {instance_name}: {', '.join(div_ops)} (avg |diff|={avg_diff:.0f})")


if __name__ == "__main__":
    main()
