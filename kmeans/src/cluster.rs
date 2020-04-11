// Copyright © 2020 Yokesh Thirumoorthi
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.
use crate::point::*;
use std::convert::From;

pub trait ClusterObject {
    /// Increment the number of points and recompute the
    /// points sum when a new point is pushed into cluster
    fn push(self, point: Point) -> Cluster;
    /// divide points_sum with number of points in the cluster
    /// to find clusters new centroid
    fn next_cluster(&self) -> Cluster;
    /// Compute the distance between centroid
    /// and nxt_centroid
    fn oscillation(&self) -> f32;
}

pub trait ClusterSetObject {
    /// Create a new clusterset using list of cluster objects
    fn new(clusters: Vec<Cluster>) -> ClusterSet;
    /// Get the cluster closest to a given point
    fn find_nearest(&self, point: Point) -> Cluster;
    /// Replace a old cluster with a new cluster in a clusterset
    fn update(&self, updated_cluster: Cluster) -> ClusterSet;
    /// Compute new centroid for each cluster in clusterset
    fn next_clusterset(&self) -> ClusterSet;
    /// Compute the aggregated oscillation of all the clusters in clusterset
    fn delta(&self) -> f32;
}

/// Centroid is a point again
pub type Centroid = Point;

/// Cluster type has a centroid, number of points
/// in the cluster and sum of those points.
#[derive(Debug, Clone, PartialEq)]
pub struct Cluster {
    pub centroid: Centroid,
    pub points_count: i32,
    pub points_sum: Point,
}

impl ClusterObject for Cluster {
    fn push(self, point: Point) -> Cluster {
        Cluster {
            centroid: self.centroid,
            points_count: self.points_count + 1,
            points_sum: self.points_sum + point,
        }
    }
    fn next_cluster(&self) -> Cluster {
        Cluster::from(self.clone().points_sum / self.points_count as f32)
    }
    fn oscillation(&self) -> f32 {
        self.clone().centroid.distance(self.clone().next_cluster().centroid)
    }
}

/// create a new cluster object with a centroid point.
/// centroid is the first point for any cluster,
impl From<Centroid> for Cluster {
    fn from(centroid: Centroid) -> Self {
        let n = centroid.point.len();
        Cluster {
            centroid,
            points_count: 0,
            points_sum: Point::new(vec![0.0; n]),
        }
    }
}


/// ClusterSet is collection of clusters.
/// Capacity of ClusterSet is always same as number of clusters.
#[derive(Debug, Clone, PartialEq)]
pub struct ClusterSet {
    pub clusters: Vec<Cluster>,
}

impl ClusterSetObject for ClusterSet {
    fn new(clusters: Vec<Cluster>) -> ClusterSet {
        ClusterSet { clusters }
    }
    fn find_nearest(&self, point: Point) -> Cluster {
        self
            .clusters
            .iter()
            .map(|c| (point.clone().distance(c.centroid.clone()), c))
            .min_by(|(d1, _), (d2, _)| d1.partial_cmp(d2).expect("tried a NaN comparison"))
            .unwrap()
            .1.clone()
    }
    fn update(&self, updated_cluster: Cluster) -> ClusterSet {
        let mut updated_clusters = Vec::new();
        for cluster in self.clusters.iter() {
            if cluster.centroid == updated_cluster.centroid {
                updated_clusters.push(updated_cluster.clone())
            } else {
                updated_clusters.push(cluster.clone())
            }
        }
        ClusterSet::new(updated_clusters)
    }
    fn next_clusterset(&self) -> ClusterSet {
        let next_centroids = self
            .clusters
            .iter()
            .map(|c| c.next_cluster())
            .collect::<Vec<Cluster>>();
        ClusterSet::new(next_centroids)
    }
    fn delta(&self) -> f32 {
        self.clusters
            .iter()
            .fold(0.0, |acc, c| acc + c.oscillation())
    }
}
