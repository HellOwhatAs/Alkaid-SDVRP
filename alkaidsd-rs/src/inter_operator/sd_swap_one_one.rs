//! SdSwapOneOne operator implementation.
//!
//! Split Delivery Swap(1,1) - exchanges single nodes with load splitting.

use super::{calc_delta, InterOperator};
use crate::cache::CacheMap;
use crate::delta::Delta;
use crate::instance::{Instance, Node};
use crate::random::Random;
use crate::route_context::RouteContext;
use crate::solution::AlkaidSolution;

/// Move data for the SdSwapOneOne operator.
#[derive(Clone, Default)]
struct SdSwapOneOneMove {
    /// Whether the routes were swapped during evaluation (for route ordering)
    #[allow(dead_code)]
    swapped: bool,
    route_x: Node,
    route_y: Node,
    node_x: Node,
    predecessor_x: Node,
    successor_x: Node,
    node_y: Node,
    predecessor_y: Node,
    successor_y: Node,
    split_load: i32,
}

/// SdSwapOneOne operator.
///
/// Split Delivery version of Swap(1,1). Exchanges single nodes between routes
/// while allowing load splitting to handle capacity differences.
#[derive(Debug, Clone, Default)]
pub struct SdSwapOneOne;

impl SdSwapOneOne {
    /// Applies a SD swap(1,1) move.
    fn do_sd_swap_one_one(mv: &SdSwapOneOneMove, solution: &mut AlkaidSolution, context: &mut RouteContext) {
        let predecessor_y = solution.predecessor(mv.node_y);
        let successor_y = solution.successor(mv.node_y);

        // Update route X
        solution.set_successor(0, context.head(mv.route_x));
        solution.set_load(mv.node_x, mv.split_load);
        solution.link(mv.predecessor_y, mv.node_y);
        solution.link(mv.node_y, mv.successor_y);
        context.set_head(mv.route_x, solution.successor(0));

        // Update route Y
        solution.set_successor(0, context.head(mv.route_y));
        solution.link(predecessor_y, successor_y);
        let customer_x = solution.customer(mv.node_x);
        let load_y = solution.load(mv.node_y);
        solution.insert(customer_x, load_y, mv.predecessor_x, mv.successor_x);
        context.set_head(mv.route_y, solution.successor(0));
    }
}

impl InterOperator for SdSwapOneOne {
    fn apply(
        &self,
        instance: &Instance,
        solution: &mut AlkaidSolution,
        context: &mut RouteContext,
        random: &mut Random,
        _cache_map: &mut CacheMap,
    ) -> Vec<Node> {
        let mut best_move = SdSwapOneOneMove::default();
        let mut best_delta = Delta::default();

        for route_x in 0..context.num_routes() {
            for route_y in (route_x + 1)..context.num_routes() {
                let mut node_x = context.head(route_x);

                while node_x != 0 {
                    let load_x = solution.load(node_x);

                    let mut node_y = context.head(route_y);
                    while node_y != 0 {
                        let load_y = solution.load(node_y);

                        // Only process when loads differ
                        if load_x != load_y {
                            let (swapped, n_x, n_y, r_x, r_y, split_load) = if load_x > load_y {
                                (false, node_x, node_y, route_x, route_y, load_x - load_y)
                            } else {
                                (true, node_y, node_x, route_y, route_x, load_y - load_x)
                            };

                            let predecessor_x_local = solution.predecessor(n_x);
                            let successor_x_local = solution.successor(n_x);
                            let predecessor_y_local = solution.predecessor(n_y);
                            let successor_y_local = solution.successor(n_y);

                            let removal_y = -calc_delta(instance, solution, n_y, predecessor_y_local, successor_y_local);
                            let insert_x = calc_delta(instance, solution, n_x, predecessor_y_local, successor_y_local);

                            // Insert y adjacent to x (before or after)
                            let before = calc_delta(instance, solution, n_y, predecessor_x_local, n_x);
                            let after = calc_delta(instance, solution, n_y, n_x, successor_x_local);

                            let (predecessor, successor, insert_y) = if before <= after {
                                (predecessor_x_local, n_x, before)
                            } else {
                                (n_x, successor_x_local, after)
                            };

                            let delta = removal_y + insert_x + insert_y;

                            if best_delta.update(delta, random) {
                                best_move = SdSwapOneOneMove {
                                    swapped,
                                    route_x: r_x,
                                    route_y: r_y,
                                    node_x: n_x,
                                    predecessor_y: predecessor_y_local,
                                    successor_y: successor_y_local,
                                    node_y: n_y,
                                    predecessor_x: predecessor,
                                    successor_x: successor,
                                    split_load,
                                };
                            }
                        }

                        node_y = solution.successor(node_y);
                    }

                    node_x = solution.successor(node_x);
                }
            }
        }

        if best_delta.value < 0 {
            Self::do_sd_swap_one_one(&best_move, solution, context);
            vec![best_move.route_x, best_move.route_y]
        } else {
            vec![]
        }
    }
}
