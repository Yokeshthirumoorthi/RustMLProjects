// Copyright © 2020 Yokesh Thirumoorthi
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.
pub mod point;
pub mod cluster;
pub mod dataset;

use crate::cluster::ClusterSet;
use crate::dataset::DataSet;

pub fn kmeans(dataset: &DataSet, clusters: ClusterSet, threshold: f32) -> ClusterSet {
    let updated_clusters = dataset.classify_into(clusters);

    if updated_clusters.delta() <= threshold {
        updated_clusters
    } else {
        kmeans(dataset, updated_clusters.next_clusterset(), threshold)
    }
}
