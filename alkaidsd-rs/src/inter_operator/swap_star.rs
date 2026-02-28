//! SwapStar operator implementation.
//!
//! Exchanges single nodes between routes.

use super::{calc_delta, InterOperator};
use crate::cache::CacheMap;
use crate::delta::Delta;
use crate::instance::{Instance, Node};
use crate::random::Random;
use crate::route_context::RouteContext;
use crate::solution::AlkaidSolution;

/// Move data for the SwapStar operator.
#[derive(Clone, Default)]
struct SwapStarMove {
    route_x: Node,
    route_y: Node,
    node_x: Node,
    predecessor_x: Node,
    successor_x: Node,
    node_y: Node,
    predecessor_y: Node,
    successor_y: Node,
}

/// SwapStar operator.
///
/// Exchanges single nodes between two routes, finding optimal insertion
/// positions for each.
#[derive(Debug, Clone, Default)]
pub struct SwapStar;

impl SwapStar {
    /// Applies a swap star move.
    fn do_swap_star(mv: &SwapStarMove, solution: &mut AlkaidSolution, context: &mut RouteContext) {
        let predecessor_x = solution.predecessor(mv.node_x);
        let successor_x = solution.successor(mv.node_x);
        let predecessor_y = solution.predecessor(mv.node_y);
        let successor_y = solution.successor(mv.node_y);

        // Update route X
        solution.set_successor(0, context.head(mv.route_x));
        solution.link(predecessor_x, successor_x);
        solution.link(mv.predecessor_y, mv.node_y);
        solution.link(mv.node_y, mv.successor_y);
        context.set_head(mv.route_x, solution.successor(0));

        // Update route Y
        solution.set_successor(0, context.head(mv.route_y));
        solution.link(predecessor_y, successor_y);
        solution.link(mv.predecessor_x, mv.node_x);
        solution.link(mv.node_x, mv.successor_x);
        context.set_head(mv.route_y, solution.successor(0));
    }

    /// Finds best insertion position for a node in a route, excluding a specific node.
    fn find_best_insertion_excluding(
        instance: &Instance,
        solution: &AlkaidSolution,
        context: &RouteContext,
        route: Node,
        customer: Node,
        exclude_node: Node,
        random: &mut Random,
    ) -> Option<(Node, Node, i32)> {
        let mut best: Option<(Node, Node, i32)> = None;
        let mut count = 0;

        let mut pred = 0;
        let mut node = context.head(route);
        
        loop {
            if pred != exclude_node && node != exclude_node {
                let delta = calc_delta(instance, solution, customer, pred, node);
                
                if best.is_none() || delta < best.unwrap().2 {
                    best = Some((pred, node, delta));
                    count = 1;
                } else if delta == best.unwrap().2 {
                    count += 1;
                    if random.next_int(1, count) == 1 {
                        best = Some((pred, node, delta));
                    }
                }
            }
            
            if node == 0 {
                break;
            }
            pred = node;
            node = solution.successor(node);
        }

        best
    }
}

impl InterOperator for SwapStar {
    fn apply(
        &self,
        instance: &Instance,
        solution: &mut AlkaidSolution,
        context: &mut RouteContext,
        random: &mut Random,
        _cache_map: &mut CacheMap,
    ) -> Vec<Node> {
        let mut best_move = SwapStarMove::default();
        let mut best_delta = Delta::default();

        for route_x in 0..context.num_routes() {
            for route_y in (route_x + 1)..context.num_routes() {
                let mut node_x = context.head(route_x);

                while node_x != 0 {
                    let load_x = solution.load(node_x);
                    let load_y_lower = -instance.capacity + context.load(route_y) + load_x;
                    let load_y_upper = instance.capacity - context.load(route_x) + load_x;

                    let mut node_y = context.head(route_y);
                    while node_y != 0 {
                        let load_y = solution.load(node_y);

                        if load_y >= load_y_lower && load_y <= load_y_upper {
                            let predecessor_x = solution.predecessor(node_x);
                            let successor_x = solution.successor(node_x);
                            let predecessor_y = solution.predecessor(node_y);
                            let successor_y = solution.successor(node_y);

                            let removal_x = -calc_delta(instance, solution, node_x, predecessor_x, successor_x);
                            let removal_y = -calc_delta(instance, solution, node_y, predecessor_y, successor_y);

                            // Try inserting x into y's position
                            let insert_x = calc_delta(instance, solution, node_x, predecessor_y, successor_y);
                            let insert_y = calc_delta(instance, solution, node_y, predecessor_x, successor_x);

                            let delta = removal_x + removal_y + insert_x + insert_y;

                            if best_delta.update(delta, random) {
                                best_move = SwapStarMove {
                                    route_x,
                                    route_y,
                                    node_x,
                                    predecessor_x: predecessor_y,
                                    successor_x: successor_y,
                                    node_y,
                                    predecessor_y: predecessor_x,
                                    successor_y: successor_x,
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
            Self::do_swap_star(&best_move, solution, context);
            vec![best_move.route_x, best_move.route_y]
        } else {
            vec![]
        }
    }
}
