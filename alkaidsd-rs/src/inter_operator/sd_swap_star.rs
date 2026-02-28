//! SdSwapStar operator implementation.
//!
//! Split Delivery SwapStar - allows splitting loads during exchange.

use super::{calc_delta, InterOperator};
use crate::cache::CacheMap;
use crate::delta::Delta;
use crate::instance::{Instance, Node};
use crate::random::Random;
use crate::route_context::RouteContext;
use crate::solution::AlkaidSolution;

/// Move data for the SdSwapStar operator.
#[derive(Clone, Default)]
struct SdSwapStarMove {
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

/// SdSwapStar operator.
///
/// Split Delivery version of SwapStar. When exchanging nodes between routes,
/// allows splitting the load to maintain capacity constraints.
#[derive(Debug, Clone, Default)]
pub struct SdSwapStar;

impl SdSwapStar {
    /// Applies a SD swap star move.
    fn do_sd_swap_star(mv: &SdSwapStarMove, solution: &mut AlkaidSolution, context: &mut RouteContext) {
        let predecessor_y = solution.predecessor(mv.node_y);
        let successor_y = solution.successor(mv.node_y);

        // Update route X: adjust load and reposition node_y
        solution.set_successor(0, context.head(mv.route_x));
        solution.set_load(mv.node_x, mv.split_load);
        solution.link(mv.predecessor_y, mv.node_y);
        solution.link(mv.node_y, mv.successor_y);
        context.set_head(mv.route_x, solution.successor(0));

        // Update route Y: remove old node_y, insert new node with remaining load
        solution.set_successor(0, context.head(mv.route_y));
        solution.link(predecessor_y, successor_y);
        let customer_x = solution.customer(mv.node_x);
        let load_y = solution.load(mv.node_y);
        solution.insert(customer_x, load_y, mv.predecessor_x, mv.successor_x);
        context.set_head(mv.route_y, solution.successor(0));
    }
}

impl InterOperator for SdSwapStar {
    fn apply(
        &self,
        instance: &Instance,
        solution: &mut AlkaidSolution,
        context: &mut RouteContext,
        random: &mut Random,
        _cache_map: &mut CacheMap,
    ) -> Vec<Node> {
        let mut best_move = SdSwapStarMove::default();
        let mut best_delta = Delta::default();

        for route_x in 0..context.num_routes() {
            for route_y in (route_x + 1)..context.num_routes() {
                let mut node_x = context.head(route_x);

                while node_x != 0 {
                    let load_x = solution.load(node_x);

                    let mut node_y = context.head(route_y);
                    while node_y != 0 {
                        let load_y = solution.load(node_y);

                        // Only process when loads differ (split scenario)
                        if load_x != load_y {
                            let (swapped, n_x, n_y, r_x, r_y, split_load) = if load_x > load_y {
                                (false, node_x, node_y, route_x, route_y, load_x - load_y)
                            } else {
                                (true, node_y, node_x, route_y, route_x, load_y - load_x)
                            };

                            let predecessor_y_local = solution.predecessor(n_y);
                            let successor_y_local = solution.successor(n_y);

                            let removal_y = -calc_delta(instance, solution, n_y, predecessor_y_local, successor_y_local);
                            let insert_x = calc_delta(instance, solution, n_x, predecessor_y_local, successor_y_local);

                            // Find best insertion for n_y in route containing n_x
                            let predecessor_x_local = solution.predecessor(n_x);
                            let successor_x_local = solution.successor(n_x);
                            let insert_y = calc_delta(instance, solution, n_y, predecessor_x_local, successor_x_local);

                            let delta = removal_y + insert_x + insert_y;

                            if best_delta.update(delta, random) {
                                best_move = SdSwapStarMove {
                                    swapped,
                                    route_x: r_x,
                                    route_y: r_y,
                                    node_x: n_x,
                                    predecessor_y: predecessor_y_local,
                                    successor_y: successor_y_local,
                                    node_y: n_y,
                                    predecessor_x: predecessor_x_local,
                                    successor_x: successor_x_local,
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
            Self::do_sd_swap_star(&best_move, solution, context);
            vec![best_move.route_x, best_move.route_y]
        } else {
            vec![]
        }
    }
}
