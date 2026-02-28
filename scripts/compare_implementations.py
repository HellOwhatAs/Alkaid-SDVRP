#!/usr/bin/env python3
"""
Comparison script for Alkaid SDVRP Solver implementations.

This script runs and compares the C++ and Rust implementations of the 
Alkaid SDVRP solver on instances from the data folder.

Usage:
    python scripts/compare_implementations.py [OPTIONS]

Examples:
    # Run with default settings on a few instances
    python scripts/compare_implementations.py

    # Run on all SET-1 instances with longer time limit
    python scripts/compare_implementations.py --set SET-1 --time-limit 60

    # Run only C++ version
    python scripts/compare_implementations.py --cpp-only

    # Run only Rust version
    python scripts/compare_implementations.py --rust-only

    # Custom configuration
    python scripts/compare_implementations.py --config custom-config.ini
"""

import argparse
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
import time
from dataclasses import dataclass, field
from pathlib import Path
from typing import Dict, List, Optional, Tuple


@dataclass
class SolverResult:
    """Result from a single solver run."""
    implementation: str
    instance: str
    objective: int
    runtime_seconds: float
    success: bool
    error_message: str = ""
    updates: List[Tuple[float, int]] = field(default_factory=list)


@dataclass
class ComparisonResult:
    """Comparison result between C++ and Rust implementations."""
    instance: str
    cpp_result: Optional[SolverResult]
    rust_result: Optional[SolverResult]
    
    def objective_diff(self) -> Optional[int]:
        """Returns the difference in objectives (Rust - C++)."""
        if self.cpp_result and self.rust_result:
            return self.rust_result.objective - self.cpp_result.objective
        return None
    
    def runtime_ratio(self) -> Optional[float]:
        """Returns the runtime ratio (Rust / C++)."""
        if self.cpp_result and self.rust_result and self.cpp_result.runtime_seconds > 0:
            return self.rust_result.runtime_seconds / self.cpp_result.runtime_seconds
        return None


class AlkaidComparison:
    """Main comparison class for Alkaid SDVRP implementations."""
    
    def __init__(self, repo_root: Path):
        self.repo_root = repo_root
        self.data_dir = repo_root / "data"
        self.cpp_build_dir = repo_root / "build-compare"
        self.rust_dir = repo_root / "alkaidsd-rs"
        self.cpp_executable: Optional[Path] = None
        self.rust_executable: Optional[Path] = None
    
    def build_cpp(self) -> bool:
        """Build the C++ implementation."""
        print("Building C++ implementation...")
        
        # Create build directory
        self.cpp_build_dir.mkdir(exist_ok=True)
        
        # Configure with CMake
        cmake_cmd = [
            "cmake",
            "-S", str(self.repo_root / "standalone"),
            "-B", str(self.cpp_build_dir),
            "-DCMAKE_BUILD_TYPE=Release"
        ]
        
        try:
            subprocess.run(cmake_cmd, check=True, capture_output=True, text=True, encoding='utf-8', errors='replace')
        except subprocess.CalledProcessError as e:
            print(f"CMake configure failed: {e.stderr}")
            return False
        
        # Build
        build_cmd = ["cmake", "--build", str(self.cpp_build_dir), "--config", "Release", "-j"]
        
        try:
            subprocess.run(build_cmd, check=True, capture_output=True, text=True, encoding='utf-8', errors='replace')
        except subprocess.CalledProcessError as e:
            print(f"CMake build failed: {e.stderr}")
            return False
        
        self.cpp_executable = self.cpp_build_dir / "AlkaidSD"
        if not self.cpp_executable.exists():
            # Try Windows name
            self.cpp_executable = self.cpp_build_dir / "AlkaidSD.exe"
        if not self.cpp_executable.exists():
            # Try MSVC multi-config build (Release subdirectory)
            self.cpp_executable = self.cpp_build_dir / "Release" / "AlkaidSD.exe"
        
        if self.cpp_executable.exists():
            print(f"  C++ build successful: {self.cpp_executable}")
            return True
        else:
            print("  C++ executable not found after build")
            return False
    
    def build_rust(self) -> bool:
        """Build the Rust implementation."""
        print("Building Rust implementation...")
        
        # Build the benchmark binary
        cargo_cmd = ["cargo", "build", "--release", "--bin", "benchmark"]
        
        try:
            subprocess.run(
                cargo_cmd, 
                cwd=self.rust_dir, 
                check=True, 
                capture_output=True, 
                text=True,
                encoding='utf-8',
                errors='replace'
            )
        except subprocess.CalledProcessError as e:
            print(f"Cargo build failed: {e.stderr}")
            return False
        
        self.rust_executable = self.rust_dir / "target" / "release" / "benchmark"
        if not self.rust_executable.exists():
            self.rust_executable = self.rust_dir / "target" / "release" / "benchmark.exe"
        
        if self.rust_executable.exists():
            print(f"  Rust build successful: {self.rust_executable}")
            return True
        else:
            print("  Rust benchmark executable not found after build")
            return False
    
    def list_instances(self, set_name: Optional[str] = None) -> List[Path]:
        """List available SDVRP instances."""
        instances = []
        
        if set_name:
            set_dirs = [self.data_dir / set_name]
        else:
            set_dirs = [d for d in self.data_dir.iterdir() if d.is_dir() and d.name.startswith("SET")]
        
        for set_dir in set_dirs:
            if set_dir.exists():
                for instance_file in sorted(set_dir.iterdir()):
                    if instance_file.is_file() and not instance_file.name.endswith(".pdf"):
                        instances.append(instance_file)
        
        return instances
    
    def create_config_file(
        self,
        instance_path: Path,
        output_path: Path,
        random_seed: int,
        time_limit: float,
        config_overrides: Optional[Dict] = None
    ) -> Path:
        """Create a configuration file for the solver."""
        config = {
            "input": str(instance_path),
            "output": str(output_path),
            "random-seed": random_seed,
            "time-limit": time_limit,
            "blink-rate": 0.021,
            "inter-operators": '["Relocate", "Swap<2, 0>", "Swap<2, 1>", "Swap<2, 2>", "Cross", "SwapStar", "SdSwapStar"]',
            "intra-operators": '["Exchange", "OrOpt<1>"]',
            "acceptance-rule-type": "LAHC",
            "acceptance-rule-args": '["length=83"]',
            "ruin-method-type": "SISRs",
            "ruin-method-args": '["average_customers=36", "max_length=8", "split_rate=0.740", "preserved_probability=0.096"]',
            "sorters": '["random=0.078", "demand=0.225", "far=0.942", "close=0.120"]',
        }
        
        if config_overrides:
            config.update(config_overrides)
        
        config_path = output_path.parent / f"config_{output_path.stem}.ini"
        
        with open(config_path, "w") as f:
            for key, value in config.items():
                f.write(f"{key} = {value}\n")
        
        return config_path
    
    def run_cpp_solver(
        self,
        instance_path: Path,
        random_seed: int,
        time_limit: float,
        config_overrides: Optional[Dict] = None
    ) -> SolverResult:
        """Run the C++ solver on an instance."""
        if not self.cpp_executable or not self.cpp_executable.exists():
            return SolverResult(
                implementation="C++",
                instance=instance_path.name,
                objective=-1,
                runtime_seconds=0,
                success=False,
                error_message="C++ executable not available"
            )
        
        with tempfile.TemporaryDirectory() as tmpdir:
            output_path = Path(tmpdir) / "solution.txt"
            config_path = self.create_config_file(
                instance_path, output_path, random_seed, time_limit, config_overrides
            )
            
            cmd = [str(self.cpp_executable), f"--config={config_path}"]
            
            start_time = time.time()
            try:
                result = subprocess.run(
                    cmd,
                    capture_output=True,
                    text=True,
                    encoding='utf-8',
                    errors='replace',
                    timeout=time_limit + 30  # Add buffer for startup/shutdown
                )
                runtime = time.time() - start_time
                
                # Parse output for objective
                updates = []
                final_objective = -1
                
                for line in result.stdout.split('\n'):
                    if "Update at" in line:
                        match = re.search(r'Update at ([\d.]+)s: (\d+)', line)
                        if match:
                            updates.append((float(match.group(1)), int(match.group(2))))
                    elif "End at" in line:
                        match = re.search(r'End at ([\d.]+)s: (\d+)', line)
                        if match:
                            final_objective = int(match.group(2))
                
                return SolverResult(
                    implementation="C++",
                    instance=instance_path.name,
                    objective=final_objective,
                    runtime_seconds=runtime,
                    success=result.returncode == 0 and final_objective >= 0,
                    updates=updates
                )
                
            except subprocess.TimeoutExpired:
                return SolverResult(
                    implementation="C++",
                    instance=instance_path.name,
                    objective=-1,
                    runtime_seconds=time_limit + 30,
                    success=False,
                    error_message="Timeout"
                )
            except Exception as e:
                return SolverResult(
                    implementation="C++",
                    instance=instance_path.name,
                    objective=-1,
                    runtime_seconds=time.time() - start_time,
                    success=False,
                    error_message=str(e)
                )
    
    def run_rust_solver(
        self,
        instance_path: Path,
        random_seed: int,
        time_limit: float,
        config_overrides: Optional[Dict] = None
    ) -> SolverResult:
        """Run the Rust solver on an instance."""
        if not self.rust_executable or not self.rust_executable.exists():
            return SolverResult(
                implementation="Rust",
                instance=instance_path.name,
                objective=-1,
                runtime_seconds=0,
                success=False,
                error_message="Rust executable not available"
            )
        
        with tempfile.TemporaryDirectory() as tmpdir:
            output_path = Path(tmpdir) / "solution.txt"
            config_path = self.create_config_file(
                instance_path, output_path, random_seed, time_limit, config_overrides
            )
            
            cmd = [str(self.rust_executable), f"--config={config_path}"]
            
            start_time = time.time()
            try:
                result = subprocess.run(
                    cmd,
                    capture_output=True,
                    text=True,
                    encoding='utf-8',
                    errors='replace',
                    timeout=time_limit + 30  # Add buffer for startup/shutdown
                )
                runtime = time.time() - start_time
                
                # Parse output for objective
                updates = []
                final_objective = -1
                
                for line in result.stdout.split('\n'):
                    if "Update at" in line:
                        match = re.search(r'Update at ([\d.]+)s: (\d+)', line)
                        if match:
                            updates.append((float(match.group(1)), int(match.group(2))))
                    elif "End at" in line:
                        match = re.search(r'End at ([\d.]+)s: (\d+)', line)
                        if match:
                            final_objective = int(match.group(2))
                
                return SolverResult(
                    implementation="Rust",
                    instance=instance_path.name,
                    objective=final_objective,
                    runtime_seconds=runtime,
                    success=result.returncode == 0 and final_objective >= 0,
                    updates=updates
                )
                
            except subprocess.TimeoutExpired:
                return SolverResult(
                    implementation="Rust",
                    instance=instance_path.name,
                    objective=-1,
                    runtime_seconds=time_limit + 30,
                    success=False,
                    error_message="Timeout"
                )
            except Exception as e:
                return SolverResult(
                    implementation="Rust",
                    instance=instance_path.name,
                    objective=-1,
                    runtime_seconds=time.time() - start_time,
                    success=False,
                    error_message=str(e)
                )
    
    def compare_instance(
        self,
        instance_path: Path,
        random_seed: int,
        time_limit: float,
        run_cpp: bool = True,
        run_rust: bool = True,
        config_overrides: Optional[Dict] = None
    ) -> ComparisonResult:
        """Compare both implementations on a single instance."""
        cpp_result = None
        rust_result = None
        
        if run_cpp:
            print(f"  Running C++ on {instance_path.name}...")
            cpp_result = self.run_cpp_solver(instance_path, random_seed, time_limit, config_overrides)
            if cpp_result.success:
                print(f"    C++: objective={cpp_result.objective}, time={cpp_result.runtime_seconds:.2f}s")
            else:
                print(f"    C++: FAILED - {cpp_result.error_message}")
        
        if run_rust:
            print(f"  Running Rust on {instance_path.name}...")
            rust_result = self.run_rust_solver(instance_path, random_seed, time_limit, config_overrides)
            if rust_result.success:
                print(f"    Rust: objective={rust_result.objective}, time={rust_result.runtime_seconds:.2f}s")
            else:
                print(f"    Rust: FAILED - {rust_result.error_message}")
        
        return ComparisonResult(
            instance=instance_path.name,
            cpp_result=cpp_result,
            rust_result=rust_result
        )
    
    def run_comparison(
        self,
        instances: List[Path],
        random_seed: int = 42,
        time_limit: float = 10.0,
        run_cpp: bool = True,
        run_rust: bool = True,
        config_overrides: Optional[Dict] = None
    ) -> List[ComparisonResult]:
        """Run comparison on multiple instances."""
        results = []
        
        for i, instance in enumerate(instances, 1):
            print(f"[{i}/{len(instances)}] Processing {instance.name}...")
            result = self.compare_instance(
                instance, random_seed, time_limit, run_cpp, run_rust, config_overrides
            )
            results.append(result)
        
        return results
    
    def print_results_table(self, results: List[ComparisonResult]):
        """Print results as a formatted table."""
        print("\n" + "=" * 100)
        print("COMPARISON RESULTS")
        print("=" * 100)
        
        # Header
        print(f"{'Instance':<30} {'C++ Obj':>12} {'C++ Time':>12} {'Rust Obj':>12} {'Rust Time':>12} {'Diff':>8} {'Ratio':>8}")
        print("-" * 100)
        
        for r in results:
            cpp_obj = str(r.cpp_result.objective) if r.cpp_result and r.cpp_result.success else "N/A"
            cpp_time = f"{r.cpp_result.runtime_seconds:.2f}s" if r.cpp_result and r.cpp_result.success else "N/A"
            rust_obj = str(r.rust_result.objective) if r.rust_result and r.rust_result.success else "N/A"
            rust_time = f"{r.rust_result.runtime_seconds:.2f}s" if r.rust_result and r.rust_result.success else "N/A"
            
            diff = r.objective_diff()
            diff_str = str(diff) if diff is not None else "N/A"
            
            ratio = r.runtime_ratio()
            ratio_str = f"{ratio:.2f}x" if ratio is not None else "N/A"
            
            print(f"{r.instance:<30} {cpp_obj:>12} {cpp_time:>12} {rust_obj:>12} {rust_time:>12} {diff_str:>8} {ratio_str:>8}")
        
        print("=" * 100)
        
        # Summary statistics
        successful_comparisons = [r for r in results if r.cpp_result and r.rust_result and r.cpp_result.success and r.rust_result.success]
        
        if successful_comparisons:
            avg_ratio = sum(r.runtime_ratio() for r in successful_comparisons) / len(successful_comparisons)
            avg_diff = sum(r.objective_diff() for r in successful_comparisons) / len(successful_comparisons)
            print(f"\nSummary ({len(successful_comparisons)} successful comparisons):")
            print(f"  Average runtime ratio (Rust/C++): {avg_ratio:.2f}x")
            print(f"  Average objective difference: {avg_diff:.2f}")
    
    def export_results_csv(self, results: List[ComparisonResult], output_path: Path):
        """Export results to CSV file."""
        with open(output_path, "w") as f:
            f.write("instance,cpp_objective,cpp_runtime,rust_objective,rust_runtime,objective_diff,runtime_ratio\n")
            
            for r in results:
                cpp_obj = r.cpp_result.objective if r.cpp_result and r.cpp_result.success else ""
                cpp_time = r.cpp_result.runtime_seconds if r.cpp_result and r.cpp_result.success else ""
                rust_obj = r.rust_result.objective if r.rust_result and r.rust_result.success else ""
                rust_time = r.rust_result.runtime_seconds if r.rust_result and r.rust_result.success else ""
                diff = r.objective_diff() if r.objective_diff() is not None else ""
                ratio = r.runtime_ratio() if r.runtime_ratio() is not None else ""
                
                f.write(f"{r.instance},{cpp_obj},{cpp_time},{rust_obj},{rust_time},{diff},{ratio}\n")
        
        print(f"\nResults exported to: {output_path}")


def parse_config_file(config_path: Path) -> Dict:
    """Parse an INI-style configuration file."""
    config = {}
    
    with open(config_path) as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith(';'):
                continue
            
            if '=' in line:
                key, value = line.split('=', 1)
                config[key.strip()] = value.strip()
    
    return config


def main():
    parser = argparse.ArgumentParser(
        description="Compare Alkaid SDVRP solver implementations (C++ vs Rust)",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog=__doc__
    )
    
    parser.add_argument(
        "--set",
        type=str,
        help="Run on instances from a specific set (e.g., SET-1, SET-2)",
        metavar="NAME"
    )
    
    parser.add_argument(
        "--instance",
        type=str,
        help="Run on a specific instance file",
        metavar="FILE"
    )
    
    parser.add_argument(
        "--time-limit",
        type=float,
        default=10.0,
        help="Time limit in seconds (default: 10)"
    )
    
    parser.add_argument(
        "--random-seed",
        type=int,
        default=42,
        help="Random seed for reproducibility (default: 42)"
    )
    
    parser.add_argument(
        "--cpp-only",
        action="store_true",
        help="Run only the C++ implementation"
    )
    
    parser.add_argument(
        "--rust-only",
        action="store_true",
        help="Run only the Rust implementation"
    )
    
    parser.add_argument(
        "--config",
        type=Path,
        help="Path to custom configuration file",
        metavar="FILE"
    )
    
    parser.add_argument(
        "--output",
        type=Path,
        help="Path to output CSV file for results",
        metavar="FILE"
    )
    
    parser.add_argument(
        "--no-build",
        action="store_true",
        help="Skip building the implementations"
    )
    
    parser.add_argument(
        "--max-instances",
        type=int,
        help="Maximum number of instances to run",
        metavar="N"
    )
    
    args = parser.parse_args()
    
    # Find repository root
    script_dir = Path(__file__).resolve().parent
    repo_root = script_dir.parent
    
    if not (repo_root / "data").exists():
        print(f"Error: Could not find data directory at {repo_root / 'data'}")
        sys.exit(1)
    
    # Create comparison instance
    comparison = AlkaidComparison(repo_root)
    
    # Build implementations
    if not args.no_build:
        run_cpp = not args.rust_only
        run_rust = not args.cpp_only
        
        if run_cpp:
            if not comparison.build_cpp():
                print("Warning: C++ build failed, skipping C++ comparisons")
                run_cpp = False
        
        if run_rust:
            if not comparison.build_rust():
                print("Warning: Rust build failed, skipping Rust comparisons")
                run_rust = False
    else:
        run_cpp = not args.rust_only
        run_rust = not args.cpp_only
    
    # Get instances to run
    if args.instance:
        instances = [Path(args.instance)]
    else:
        instances = comparison.list_instances(args.set)
    
    if not instances:
        print("No instances found")
        sys.exit(1)
    
    # Limit number of instances if requested
    if args.max_instances:
        instances = instances[:args.max_instances]
    
    print(f"\nFound {len(instances)} instance(s) to process")
    
    # Parse configuration overrides
    config_overrides = None
    if args.config:
        config_overrides = parse_config_file(args.config)
    
    # Run comparisons
    results = comparison.run_comparison(
        instances,
        random_seed=args.random_seed,
        time_limit=args.time_limit,
        run_cpp=run_cpp,
        run_rust=run_rust,
        config_overrides=config_overrides
    )
    
    # Print results
    comparison.print_results_table(results)
    
    # Export to CSV if requested
    if args.output:
        comparison.export_results_csv(results, args.output)


if __name__ == "__main__":
    main()
