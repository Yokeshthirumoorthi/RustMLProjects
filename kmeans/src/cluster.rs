// Copyright © 2020 Yokesh Thirumoorthi
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.
use crate::point::Vector2D;

/// Centroid is a point again
pub type Centroid = Vector2D;

/// Cluster type has a centroid, number of points
/// in the cluster and sum of those points.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Cluster {
    pub curr_centroid: Centroid,
    pub points_count: i32,
    pub points_sum: Vector2D,
}

impl Cluster {
    /// create a new cluster object with a centroid point.
    /// centroid is the first point for any cluster,
    pub fn new(centroid: Centroid) -> Cluster {
        Cluster {
            curr_centroid: centroid,
            points_count: 1,
            points_sum: centroid,
        }
    }
    /// Increment the number of points and recompute the 
    /// points sum when a new point is pushed into cluster
    pub fn push(&self, point: Vector2D) -> Cluster {
        if self.curr_centroid == point {
            return *self;
        }
        Cluster {
            curr_centroid: self.curr_centroid,
            points_count: self.points_count + 1,
            points_sum: self.points_sum + point,
        }
    }
    /// divide points_sum with number of points in the cluster
    /// to find clusters new centroid
    fn next_cluster(&self) -> Cluster {
        Cluster::new(self.points_sum / self.points_count as f32)
    }
    /// Compute the distance between curr_centroid
    /// and nxt_centroid
    fn oscillation(&self) -> f32 {
        self.curr_centroid.distance(self.next_cluster().curr_centroid)
    }
}

#[test]
fn cluster_init_works() {
    let p0 = Vector2D::new((0.0, 0.0));
    let p1 = Vector2D::new((1.0, 1.0));
    let c0 = Cluster::new(p0);
    assert_eq!(
        c0,
        Cluster {
            curr_centroid: p0,
            points_count: 1,
            points_sum: p0,
        }
    );
    assert_eq!(
        c0.push(p1),
        Cluster {
            curr_centroid: p0,
            points_count: 2,
            points_sum: p0 + p1,
        }
    );
    assert_eq!(c0.oscillation(), 0.0);
    assert_eq!(c0.push(p1).oscillation(), 0.70710677);
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
            .map(|c| (point.distance(c.curr_centroid), c))
            .min_by(|(d1, _), (d2, _)| d1.partial_cmp(d2).expect("tried a NaN comparison"))
            .unwrap()
            .1
    }
    /// Replace a old cluster with a new cluster in a clusterset
    pub fn update(&self, updated_cluster: &Cluster) -> ClusterSet {
        let mut updated_clusters = Vec::new();
        for cluster in self.clusters.iter() {
            if cluster.curr_centroid == updated_cluster.curr_centroid {
                updated_clusters.push(*updated_cluster)
            } else {
                updated_clusters.push(*cluster)
            }
        }
        ClusterSet::new(updated_clusters)
    }
    /// Compute new centroid for each cluster in clusterset
    pub fn next_clusterset(&self) -> ClusterSet {
        let next_centroids = self.clusters.iter().map(|c| c.next_cluster()).collect::<Vec<Cluster>>();
        ClusterSet::new(next_centroids)
    }
    /// Compute the aggregated oscillation of all the clusters in clusterset
    pub fn delta(&self) -> f32 {
        self.clusters
            .iter()
            .fold(0.0, |acc, c| acc + c.oscillation())
    }
}

#[test]
fn clusterset_init_works() {
    let p0 = Vector2D::new((0.0, 0.0));
    let p1 = Vector2D::new((1.0, 1.0));
    let p2 = Vector2D::new((2.0, 2.0));
    let c0 = Cluster::new(p0);
    let mut c1 = Cluster::new(p1);
    let clusterset = ClusterSet::new(vec![c0, c1]);
    assert_eq!(
        clusterset,
        ClusterSet {
            clusters: vec![c0, c1],
        }
    );
    assert_eq!(clusterset.find_nearest(p2), c1);
    c1 = c1.push(p2);
    let new_c1 = Cluster {
        curr_centroid: p1,
        points_count: 2,
        points_sum: p1 + p2,
    };
    let new_clusterset = ClusterSet::new(vec![c0, new_c1]);
    assert_eq!(clusterset.update(&c1), new_clusterset);
    assert_eq!(clusterset.delta(), 0.0);
}
