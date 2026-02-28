//! Cache system for efficient inter-route optimization.
//!
//! Caches store computed move deltas to avoid redundant calculations
//! when routes haven't changed since the last evaluation.

use crate::instance::Node;
use crate::route_context::RouteContext;
use crate::solution::AlkaidSolution;
use std::any::{Any, TypeId};
use std::collections::HashMap;

/// Trait for cache implementations.
///
/// Caches must support resetting, adding/removing routes, and saving state.
pub trait Cache: Any {
    /// Resets the cache based on the current solution state.
    fn reset(&mut self, solution: &AlkaidSolution, context: &RouteContext);

    /// Called when a new route is added.
    fn add_route(&mut self, route_index: Node);

    /// Called when a route is removed.
    fn remove_route(&mut self, route_index: Node);

    /// Called when a route is moved (during compaction).
    fn move_route(&mut self, dest_route_index: Node, src_route_index: Node);

    /// Saves the current state for later comparison.
    fn save(&mut self, solution: &AlkaidSolution, context: &RouteContext);

    /// Required for downcasting.
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// A map of caches keyed by type.
///
/// Allows operators to store their specific cache types without
/// knowing about each other's implementations.
#[derive(Default)]
pub struct CacheMap {
    caches: HashMap<TypeId, Box<dyn Cache>>,
}

impl CacheMap {
    /// Creates a new empty cache map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets or creates a cache of the specified type.
    ///
    /// If the cache doesn't exist, it's created and reset.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The cache type (must implement Cache + Default)
    pub fn get<T: Cache + Default + 'static>(
        &mut self,
        solution: &AlkaidSolution,
        context: &RouteContext,
    ) -> &mut T {
        let type_id = TypeId::of::<T>();

        if !self.caches.contains_key(&type_id) {
            let mut cache = Box::new(T::default());
            cache.reset(solution, context);
            self.caches.insert(type_id, cache);
        }

        self.caches
            .get_mut(&type_id)
            .unwrap()
            .as_any_mut()
            .downcast_mut::<T>()
            .unwrap()
    }

    /// Resets all caches.
    pub fn reset(&mut self, solution: &AlkaidSolution, context: &RouteContext) {
        for cache in self.caches.values_mut() {
            cache.reset(solution, context);
        }
    }

    /// Notifies all caches that a route was added.
    pub fn add_route(&mut self, route_index: Node) {
        for cache in self.caches.values_mut() {
            cache.add_route(route_index);
        }
    }

    /// Notifies all caches that a route was removed.
    pub fn remove_route(&mut self, route_index: Node) {
        for cache in self.caches.values_mut() {
            cache.remove_route(route_index);
        }
    }

    /// Notifies all caches that a route was moved.
    pub fn move_route(&mut self, dest_route_index: Node, src_route_index: Node) {
        for cache in self.caches.values_mut() {
            cache.move_route(dest_route_index, src_route_index);
        }
    }

    /// Saves state in all caches.
    pub fn save(&mut self, solution: &AlkaidSolution, context: &RouteContext) {
        for cache in self.caches.values_mut() {
            cache.save(solution, context);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct TestCache {
        reset_count: usize,
    }

    impl Cache for TestCache {
        fn reset(&mut self, _: &AlkaidSolution, _: &RouteContext) {
            self.reset_count += 1;
        }
        fn add_route(&mut self, _: Node) {}
        fn remove_route(&mut self, _: Node) {}
        fn move_route(&mut self, _: Node, _: Node) {}
        fn save(&mut self, _: &AlkaidSolution, _: &RouteContext) {}
        fn as_any(&self) -> &dyn Any { self }
        fn as_any_mut(&mut self) -> &mut dyn Any { self }
    }

    #[test]
    fn test_cache_map() {
        let solution = AlkaidSolution::new();
        let context = RouteContext::new();
        let mut cache_map = CacheMap::new();

        // First access creates and resets
        let cache: &mut TestCache = cache_map.get(&solution, &context);
        assert_eq!(cache.reset_count, 1);

        // Second access returns same cache
        let cache: &mut TestCache = cache_map.get(&solution, &context);
        assert_eq!(cache.reset_count, 1);
    }
}
