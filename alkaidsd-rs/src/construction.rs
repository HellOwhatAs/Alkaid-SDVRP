//! Construction heuristic for initial solution generation.
//!
//! Builds an initial feasible solution using insertion heuristics.

use crate::delta::Delta;
use crate::instance::{Instance, Node};
use crate::random::Random;
use crate::route_context::RouteContext;
use crate::solution::AlkaidSolution;
use crate::utils::calc_fleet_lower_bound;

/// Candidate for insertion: (customer, demand)
type CandidateList = Vec<(Node, i32)>;

/// Insertion criteria.
#[derive(Clone, Copy)]
enum Criterion {
    Mcfic { gamma: f32 },
    Nfic,
}

/// Calculates insertion cost based on criterion.
fn calc_insertion_cost(
    instance: &Instance,
    solution: &AlkaidSolution,
    predecessor: Node,
    successor: Node,
    customer: Node,
    criterion: Criterion,
) -> f32 {
    let pre_customer = solution.customer(predecessor);
    let suc_customer = solution.customer(successor);

    match criterion {
        Criterion::Mcfic { gamma } => {
            (instance.distance(pre_customer, customer)
                + instance.distance(customer, suc_customer)
                - instance.distance(pre_customer, suc_customer)) as f32
                - 2.0 * gamma * instance.distance(0, customer) as f32
        }
        Criterion::Nfic => {
            if pre_customer == 0 {
                f32::MAX
            } else {
                instance.distance(pre_customer, customer) as f32
            }
        }
    }
}

/// Finds best insertion for a customer in a route.
fn find_best_insertion(
    instance: &Instance,
    solution: &AlkaidSolution,
    context: &RouteContext,
    route_index: Node,
    customer: Node,
    criterion: Criterion,
    random: &mut Random,
) -> (Node, Node, Delta<f32>) {
    let head = context.head(route_index);
    let head_cost = calc_insertion_cost(instance, solution, 0, head, customer, criterion);
    
    let mut best_predecessor = 0;
    let mut best_successor = head;
    let mut best_delta = Delta::new(head_cost, 1);

    let mut node_index = head;
    while node_index != 0 {
        let successor = solution.successor(node_index);
        let cost = calc_insertion_cost(instance, solution, node_index, successor, customer, criterion);
        
        if best_delta.update(cost, random) {
            best_predecessor = node_index;
            best_successor = successor;
        }
        node_index = successor;
    }

    (best_predecessor, best_successor, best_delta)
}

/// Adds a new route with a random candidate.
fn add_route(
    candidate_list: &mut CandidateList,
    random: &mut Random,
    solution: &mut AlkaidSolution,
    context: &mut RouteContext,
) -> usize {
    let position = random.next_int(0, candidate_list.len() as i32 - 1) as usize;
    let (customer, demand) = candidate_list[position];
    let node_index = solution.insert(customer, demand, 0, 0);

    candidate_list[position] = *candidate_list.last().unwrap();
    candidate_list.pop();

    context.add_route(node_index, node_index, demand);
    position
}

/// Insertion with metadata.
struct InsertionInfo {
    predecessor: Node,
    successor: Node,
    route_index: Node,
    cost: Delta<f32>,
    candidate_position: Option<usize>,
}

impl Default for InsertionInfo {
    fn default() -> Self {
        Self {
            predecessor: 0,
            successor: 0,
            route_index: 0,
            cost: Delta::new(f32::MAX, -1),
            candidate_position: None,
        }
    }
}

/// Sequential insertion strategy.
fn sequential_insertion(
    instance: &Instance,
    criterion: Criterion,
    candidate_list: &mut CandidateList,
    random: &mut Random,
    solution: &mut AlkaidSolution,
    context: &mut RouteContext,
) {
    let mut is_full = vec![false; context.num_routes() as usize];

    while !candidate_list.is_empty() {
        let mut inserted = false;

        for route_index in 0..context.num_routes() {
            if is_full[route_index as usize] {
                continue;
            }

            let mut best = InsertionInfo::default();

            for (i, &(customer, demand)) in candidate_list.iter().enumerate() {
                if context.load(route_index) + demand > instance.capacity {
                    continue;
                }

                let (pred, succ, delta) = find_best_insertion(
                    instance, solution, context, route_index, customer, criterion, random,
                );

                if best.cost.update_from(&delta, random) {
                    best.predecessor = pred;
                    best.successor = succ;
                    best.route_index = route_index;
                    best.candidate_position = Some(i);
                }
            }

            if let Some(pos) = best.candidate_position {
                let (customer, demand) = candidate_list[pos];
                candidate_list[pos] = *candidate_list.last().unwrap();
                candidate_list.pop();

                let node_index = solution.insert(customer, demand, best.predecessor, best.successor);

                if best.predecessor == 0 {
                    context.set_head(route_index, node_index);
                }
                context.add_load(route_index, demand);
                inserted = true;
            } else {
                is_full[route_index as usize] = true;
            }
        }

        if !inserted {
            add_route(candidate_list, random, solution, context);
            is_full.push(false);
        }
    }
}

/// Constructs an initial solution using randomized insertion heuristics.
///
/// # Arguments
///
/// * `instance` - The problem instance
/// * `random` - Random number generator
///
/// # Returns
///
/// An initial feasible solution
pub fn construct(instance: &Instance, random: &mut Random) -> AlkaidSolution {
    let mut candidate_list: CandidateList = Vec::new();
    let num_fleets = calc_fleet_lower_bound(instance);

    for i in 1..instance.num_customers {
        let mut demand = instance.demands[i as usize];
        while demand > 0 {
            let split_demand = demand.min(instance.capacity);
            candidate_list.push((i, split_demand));
            demand -= split_demand;
        }
    }

    let mut solution = AlkaidSolution::new();
    let mut context = RouteContext::new();

    for _ in 0..num_fleets {
        if candidate_list.is_empty() {
            break;
        }
        add_route(&mut candidate_list, random, &mut solution, &mut context);
    }

    // Select criterion
    let criterion = if random.next_int(0, 1) == 0 {
        let gamma = random.next_int(0, 34) as f32 * 0.05;
        Criterion::Mcfic { gamma }
    } else {
        Criterion::Nfic
    };

    sequential_insertion(instance, criterion, &mut candidate_list, random, &mut solution, &mut context);

    solution
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct() {
        let instance = Instance {
            num_customers: 5,
            capacity: 100,
            demands: vec![0, 50, 60, 40, 30],
            distance_matrix: vec![
                vec![0, 10, 20, 30, 40],
                vec![10, 0, 15, 25, 35],
                vec![20, 15, 0, 10, 20],
                vec![30, 25, 10, 0, 10],
                vec![40, 35, 20, 10, 0],
            ],
        };

        let mut random = Random::new(42);
        let solution = construct(&instance, &mut random);

        assert!(!solution.node_indices().is_empty());
        let obj = solution.calc_objective(&instance);
        assert!(obj > 0);
    }
}
