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
pub trait VariantOps<I: ProblemInstance> {
    fn construct(&self, instance: &I, random: &mut Random) -> AlkaidSolution;
    fn repair_route(&self, instance: &I, route_index: Node, solution: &mut AlkaidSolution, context: &mut RouteContext);
    fn reinsert_customer(&self, instance: &I, customer: Node, solution: &mut AlkaidSolution, context: &mut RouteContext, random: &mut Random, blink_rate: f64);
    fn fleet_lower_bound(&self, instance: &I) -> Node;
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
