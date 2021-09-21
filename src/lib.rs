use autocxx::include_cpp;

include_cpp! {
    #include "actor-wrapper.h"

    safety!(unsafe)

    generate!("simplified_actor_t")
}

pub struct Actor {
    inner: ffi::simplified_actor_t,
}

// impl Actor {
//     pub fn new() -> Self {}
//     /// Calculates a route.
//     pub fn route() -> Result<()> {}
//     /// Provides information about nodes and edges.
//     pub fn locate() -> Result<()> {}
//     /// Optimizes the order of a set of waypoints by time.
//     pub fn optimized_route() -> Result<()> {}
//     /// Computes the time and distance between a set of locations and returns them as a matrix table.
//     pub fn matrix() -> Result<()> {}
//     /// Calculates isochrones and isodistances.
//     pub fn isochrone() -> Result<()> {}
//     /// Map-matching for a set of input locations, e.g. from a GPS.
//     pub fn trace_route() -> Result<()> {}
//     /// Returns detailed attribution along each portion of a route calculated from a set of input locations, e.g.
// from a GPS trace.     pub fn trace_attributes() -> Result<()> {}
//     /// Provides elevation data for a set of input geometries.
//     pub fn height() -> Result<()> {}
//     /// Lookup if transit stops are available in a defined radius around a set of input locations.
//     pub fn transit_available() -> Result<()> {}
//     /// Returns all road segments which were touched by the routing algorithm during the graph traversal.
//     pub fn expansion() -> Result<()> {}
//     /// Returns routes from all the input locations to the minimum cost meeting point of those paths.
//     pub fn centroid() -> Result<()> {}
// }
