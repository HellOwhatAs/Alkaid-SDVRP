//! Alkaid SDVRP Solver - Command Line Interface
//!
//! This binary provides a command-line interface for the Alkaid SDVRP solver.
//! It reads problem instances from files and writes solutions.

fn main() {
    println!("AlkaidSD-RS: Split Delivery Vehicle Routing Problem Solver");
    println!();
    println!("This is a Rust port of the C++ AlkaidSD library.");
    println!("Use the library directly in your Rust projects for full functionality.");
    println!();
    println!("Example usage:");
    println!("  use alkaidsd::{{Instance, AlkaidSolver, AlkaidConfig}};");
    println!("  use alkaidsd::acceptance_rule::HillClimbing;");
    println!("  use alkaidsd::inter_operator::SwapStar;");
    println!("  use alkaidsd::intra_operator::Exchange;");
    println!("  use alkaidsd::ruin_method::RandomRuin;");
    println!("  use alkaidsd::sorter::{{Sorter, SortByRandom}};");
    println!();
    println!("For CLI support, enable the 'cli' feature and add dependencies.");
}
