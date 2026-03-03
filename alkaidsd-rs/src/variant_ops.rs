//! Variant-specific operations for different VRP variants.
//!
//! This module defines the `VariantOps` trait that encapsulates operations
//! specific to a VRP variant, and `SdvrpOps` which implements it for SDVRP.

use crate::construction::construct;
use crate::instance::{Instance, Node, ProblemInstance};
use crate::random::Random;
use crate::repair::repair;
use crate::route_context::RouteContext;
use crate::solution::AlkaidSolution;
use crate::split_reinsertion::split_reinsertion;
use crate::utils::calc_fleet_lower_bound;

/// Trait for variant-specific VRP operations.
///
/// This trait encapsulates all operations that differ between VRP variants,
/// allowing the solver and operators to remain generic. Each VRP variant
/// (SDVRP, CVRP, VRPTW, etc.) provides its own implementation.
///
/// The solver calls these methods for variant-specific logic while using
/// generic operators for the common local search.
///
/// # Type Parameters
///
/// * `I` - The problem instance type (must implement [`ProblemInstance`])
///
/// # Example
///
/// See [`SdvrpOps`] for the SDVRP implementation.
pub trait VariantOps<I: ProblemInstance> {
    /// Constructs an initial feasible solution.
    fn construct(&self, instance: &I, random: &mut Random) -> AlkaidSolution;

    /// Repairs a route after intra-route modifications.
    ///
    /// For SDVRP, this merges duplicate customer visits within a route.
    /// For variants without such needs, this can be a no-op.
    fn repair_route(&self, instance: &I, route_index: Node, solution: &mut AlkaidSolution, context: &mut RouteContext);

    /// Reinserts a customer after it has been removed during perturbation.
    ///
    /// For SDVRP, this performs split reinsertion across multiple routes.
    /// For CVRP, this would insert the full demand into a single route.
    fn reinsert_customer(&self, instance: &I, customer: Node, solution: &mut AlkaidSolution, context: &mut RouteContext, random: &mut Random, blink_rate: f64);

    /// Returns a lower bound on the fleet size needed.
    fn fleet_lower_bound(&self, instance: &I) -> Node;

    /// Calculates the objective value of a solution.
    ///
    /// Default implementation sums distances along all routes.
    fn calc_objective(&self, instance: &I, solution: &AlkaidSolution) -> i32 {
        solution.calc_objective(instance)
    }
}

/// SDVRP-specific operations.
pub struct SdvrpOps;

impl VariantOps<Instance> for SdvrpOps {
    fn construct(&self, instance: &Instance, random: &mut Random) -> AlkaidSolution {
        construct(instance, random)
    }

    fn repair_route(&self, instance: &Instance, route_index: Node, solution: &mut AlkaidSolution, context: &mut RouteContext) {
        repair(instance, route_index, solution, context);
    }

    fn reinsert_customer(&self, instance: &Instance, customer: Node, solution: &mut AlkaidSolution, context: &mut RouteContext, random: &mut Random, blink_rate: f64) {
        split_reinsertion(
            instance,
            customer,
            instance.demands[customer as usize],
            blink_rate,
            solution,
            context,
            random,
        );
    }

    fn fleet_lower_bound(&self, instance: &Instance) -> Node {
        calc_fleet_lower_bound(instance)
    }
}
