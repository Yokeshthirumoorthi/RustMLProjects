// Copyright © 2020 Yokesh Thirumoorthi
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.
use crate::point::*;
use std::convert::From;

/// Centroid is a point again
pub type Centroid = Vector2D;

/// Cluster type has a centroid, number of points
/// in the cluster and sum of those points.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Cluster {
    pub centroid: Centroid,
    pub points_count: i32,
    pub points_sum: Vector2D,
}

impl Cluster {
    /// Increment the number of points and recompute the
    /// points sum when a new point is pushed into cluster
    pub fn push(&self, point: Vector2D) -> Cluster {
        Cluster {
            centroid: self.centroid,
            points_count: self.points_count + 1,
            points_sum: self.points_sum + point,
        }
    }
    /// divide points_sum with number of points in the cluster
    /// to find clusters new centroid
    fn next_cluster(&self) -> Cluster {
        Cluster::from(self.points_sum / self.points_count as f32)
    }
    /// Compute the distance between centroid
    /// and nxt_centroid
    fn oscillation(&self) -> f32 {
        self.centroid.distance(self.next_cluster().centroid)
    }
}

/// create a new cluster object with a centroid point.
/// centroid is the first point for any cluster,
impl From<Centroid> for Cluster {
    fn from(centroid: Centroid) -> Self {
        Cluster {
            centroid,
            points_count: 0,
            points_sum: Vector2D::new((0.0,0.0)),
        }
    }
}

#[test]
fn cluster_init_works() {
    let p0 = Vector2D::new((0.0, 0.0));
    let p1 = Vector2D::new((1.0, 1.0));
    let c0 = Cluster::from(p0);
    assert_eq!(c0.push(p1).oscillation(), 1.4142135);
}

/// ClusterSet is collection of clusters.
/// Capacity of ClusterSet is always same as number of clusters.
#[derive(Debug, Clone, PartialEq)]
pub struct ClusterSet {
    pub clusters: Vec<Cluster>,
}

impl ClusterSet {
    /// Create a new clusterset using list of cluster objects
    pub fn new(clusters: Vec<Cluster>) -> ClusterSet {
        ClusterSet { clusters }
    }
    /// Get the cluster closest to a given point
    pub fn find_nearest(&self, point: Vector2D) -> Cluster {
        *self
            .clusters
            .iter()
            .map(|c| (point.distance(c.centroid), c))
            .min_by(|(d1, _), (d2, _)| d1.partial_cmp(d2).expect("tried a NaN comparison"))
            .unwrap()
            .1
    }
    /// Replace a old cluster with a new cluster in a clusterset
    pub fn update(&self, updated_cluster: &Cluster) -> ClusterSet {
        let mut updated_clusters = Vec::new();
        for cluster in self.clusters.iter() {
            if cluster.centroid == updated_cluster.centroid {
                updated_clusters.push(*updated_cluster)
            } else {
                updated_clusters.push(*cluster)
            }
        }
        ClusterSet::new(updated_clusters)
    }
    /// Compute new centroid for each cluster in clusterset
    pub fn next_clusterset(&self) -> ClusterSet {
        let next_centroids = self
            .clusters
            .iter()
            .map(|c| c.next_cluster())
            .collect::<Vec<Cluster>>();
        ClusterSet::new(next_centroids)
    }
    /// Compute the aggregated oscillation of all the clusters in clusterset
    pub fn delta(&self) -> f32 {
        self.clusters
            .iter()
            .fold(0.0, |acc, c| acc + c.oscillation())
    }
}
