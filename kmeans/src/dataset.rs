// Copyright © 2020 Yokesh Thirumoorthi
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use crate::cluster::*;
use crate::point::*;

/// Dataset holds the complete collection of
/// points that need to be classified.
/// The size of the dataset is not known
/// until the program starts.
/// A new dataset is initialized
/// using Default::default() and the points
/// are inserted using dataset.push method.
///
/// The PartialEq is derived for testing purpose
#[derive(Debug, Default, PartialEq)]
pub struct DataSet {
    pub items: Vec<Point>,
}

impl DataSet {

    fn collect_as_cluster(&self, n: usize) -> Vec<Cluster> {
        self.items
            .iter()
            .take(n)
            .map(|item| Cluster::from(item.clone()))
            .collect()
    }

    /// Add new points to dataset
    pub fn push(&mut self, point: Point) {
        self.items.push(point);
    }

    /// Generate/Pick the first n elements
    /// of dataset as centroids.
    /// The program panics when the number
    /// of centroids is more than the length of dataset.
    pub fn generate_initial_clusters(&self, n: usize) -> ClusterSet {
        if n > self.items.len() {
            panic!("Dont ask for more clusters than total points in dataset!");
        }

        ClusterSet::new(self.collect_as_cluster(n))
    }

    /// Find the nearest cluster for each point in dataset.
    /// And update the cluster's nxt_centroid with the point.
    pub fn classify_into(&self, clusters: ClusterSet) -> ClusterSet {
        let mut updated_clusters = clusters;
        for item in self.items.iter() {
            let nearest_cluster = updated_clusters.find_nearest(item).push(item);
            updated_clusters = updated_clusters.update(nearest_cluster);
        }
        updated_clusters
    }
}
