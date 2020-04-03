// Copyright © 2020 Yokesh Thirumoorthi
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use crate::point::Vector2D;
use crate::cluster::*;


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
    pub items: Vec<Vector2D>,
}

impl DataSet {
    /// Add new points to dataset
    pub fn push(&mut self, point: Vector2D) {
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
        let clusters = self.items[..n].iter().map(|p| Cluster::new(*p)).collect();

        ClusterSet::new(clusters)
    }

    /// Find the nearest cluster for each point in dataset.
    /// And update the cluster's nxt_centroid with the point.
    pub fn classify_into(&self, clusters: ClusterSet) -> ClusterSet {
        let mut updated_clusters = clusters.clone();
        for item in self.items.iter() {
            let nearest_cluster = updated_clusters.find_nearest(*item).push(*item);
            updated_clusters = updated_clusters.update(&nearest_cluster);
        }
        updated_clusters
    }
}

#[test]
fn dataset_init_works() {
    let p0 = Vector2D::new((0.0, 0.0));
    let p1 = Vector2D::new((1.0, 1.0));
    let mut dataset: DataSet = Default::default();
    assert_eq!(dataset, DataSet { items: Vec::new() });
    dataset.push(p0);
    assert_eq!(dataset, DataSet { items: vec![p0] });
    dataset.push(p1);
    assert_eq!(
        dataset,
        DataSet {
            items: vec![p0, p1]
        }
    );
}

#[test]
fn dataset_generates_initial_clusters() {
    let mut dataset: DataSet = Default::default();
    let p0 = Vector2D::new((0.0, 0.0));
    let p1 = Vector2D::new((1.0, 1.0));
    let p2 = Vector2D::new((2.0, 2.0));
    dataset.push(p0);
    dataset.push(p1);
    dataset.push(p2);

    let c0 = Cluster::new(p0);
    let c1 = Cluster::new(p1);
    let expected_clusterset = ClusterSet::new(vec![c0, c1]);
    assert_eq!(dataset.generate_initial_clusters(2), expected_clusterset);
}

#[test]
fn dataset_classifies_into_clusters() {
    let mut dataset: DataSet = Default::default();
    let p0 = Vector2D::new((0.0, 0.0));
    let p1 = Vector2D::new((1.0, 1.0));
    let p2 = Vector2D::new((2.0, 2.0));
    let p3 = Vector2D::new((3.0, 3.0));
    let p4 = Vector2D::new((4.0, 4.0));
    dataset.push(p0);
    dataset.push(p1);
    dataset.push(p2);
    dataset.push(p3);
    dataset.push(p4);

    let c1 = Cluster::new(p1);
    let c2 = Cluster::new(p2);
    let clusterset = ClusterSet::new(vec![c1, c2]);
    let expected_clusterset = ClusterSet::new(vec![c1.push(p0), c2.push(p3).push(p4)]);
    assert_eq!(dataset.classify_into(clusterset), expected_clusterset);
}

#[test]
#[should_panic(expected = "Dont ask for more clusters than total points in dataset!")]
fn clusters_to_dataset_size_check() {
    let mut dataset: DataSet = Default::default();
    let p1 = Vector2D::new((1.0, 1.0));
    dataset.push(p1);
    dataset.generate_initial_clusters(3);
}
