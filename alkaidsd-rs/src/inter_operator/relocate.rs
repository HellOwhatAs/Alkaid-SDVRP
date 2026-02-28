//! Relocate operator implementation.
//!
//! Moves a single node from one route to another.

use super::{calc_delta, InterOperator};
use crate::cache::CacheMap;
use crate::delta::Delta;
use crate::instance::{Instance, Node};
use crate::random::Random;
use crate::route_context::RouteContext;
use crate::solution::AlkaidSolution;

/// Move data for the Relocate operator.
#[derive(Clone, Default)]
struct RelocateMove {
    route_x: Node,
    route_y: Node,
    node_x: Node,
    predecessor_x: Node,
    successor_x: Node,
}

/// Relocate operator.
///
/// Moves a single node from route X to route Y, finding the best
/// insertion position in route Y.
#[derive(Debug, Clone, Default)]
pub struct Relocate;

impl Relocate {
    /// Applies a relocate move.
    fn do_relocate(mv: &RelocateMove, solution: &mut AlkaidSolution, context: &mut RouteContext) {
        let predecessor_x = solution.predecessor(mv.node_x);
        let successor_x = solution.successor(mv.node_x);

        // Remove from route X
        solution.set_successor(0, context.head(mv.route_x));
        solution.link(predecessor_x, successor_x);
        context.set_head(mv.route_x, solution.successor(0));

        // Insert into route Y
        solution.set_successor(0, context.head(mv.route_y));
        solution.link(mv.predecessor_x, mv.node_x);
        solution.link(mv.node_x, mv.successor_x);
        context.set_head(mv.route_y, solution.successor(0));
    }

    /// Finds best insertion position for a node in a route.
    fn find_best_insertion(
        instance: &Instance,
        solution: &AlkaidSolution,
        context: &RouteContext,
        route_y: Node,
        customer: Node,
        random: &mut Random,
    ) -> (Node, Node, i32) {
        let mut best_pred = 0;
        let mut best_succ = context.head(route_y);
        let mut best_delta = calc_delta(instance, solution, customer, 0, context.head(route_y));
        let mut count = 1;

        let mut node = context.head(route_y);
        while node != 0 {
            let succ = solution.successor(node);
            let delta = calc_delta(instance, solution, customer, node, succ);
            
            if delta < best_delta {
                best_delta = delta;
                best_pred = node;
                best_succ = succ;
                count = 1;
            } else if delta == best_delta {
                count += 1;
                if random.next_int(1, count) == 1 {
                    best_pred = node;
                    best_succ = succ;
                }
            }
            
            node = succ;
        }

        (best_pred, best_succ, best_delta)
    }
}

impl InterOperator for Relocate {
    fn apply(
        &self,
        instance: &Instance,
        solution: &mut AlkaidSolution,
        context: &mut RouteContext,
        random: &mut Random,
        _cache_map: &mut CacheMap,
    ) -> Vec<Node> {
        let mut best_move = RelocateMove::default();
        let mut best_delta = Delta::default();

        for route_x in 0..context.num_routes() {
            for route_y in 0..context.num_routes() {
                if route_x == route_y {
                    continue;
                }

                let mut node_x = context.head(route_x);
                while node_x != 0 {
                    // Check capacity constraint
                    if context.load(route_y) + solution.load(node_x) <= instance.capacity {
                        let customer_x = solution.customer(node_x);
                        let (pred, succ, insertion_delta) = Self::find_best_insertion(
                            instance, solution, context, route_y, customer_x, random,
                        );
                        
                        let predecessor_x = solution.predecessor(node_x);
                        let successor_x = solution.successor(node_x);
                        let removal_delta = -calc_delta(instance, solution, node_x, predecessor_x, successor_x);
                        
                        let delta = insertion_delta + removal_delta;
                        
                        if best_delta.update(delta, random) {
                            best_move = RelocateMove {
                                route_x,
                                route_y,
                                node_x,
                                predecessor_x: pred,
                                successor_x: succ,
                            };
                        }
                    }
                    node_x = solution.successor(node_x);
                }
            }
        }

        if best_delta.value < 0 {
            Self::do_relocate(&best_move, solution, context);
            vec![best_move.route_x, best_move.route_y]
        } else {
            vec![]
        }
    }
}
