// Copyright © 2020 Yokesh Thirumoorthi
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.
use kmeans::dataset::DataSet;
use kmeans::point::*;
fn main() {
    let mut dataset: DataSet = Default::default();

    dataset.push(Vector::new(vec![1.0, 1.0]));
    dataset.push(Vector::new(vec![2.0, 2.0]));
    dataset.push(Vector::new(vec![3.0, 3.0]));
    dataset.push(Vector::new(vec![4.0, 4.0]));
    dataset.push(Vector::new(vec![5.0, 5.0]));
    dataset.push(Vector::new(vec![6.0, 6.0]));
    dataset.push(Vector::new(vec![7.0, 7.0]));
    dataset.push(Vector::new(vec![8.0, 8.0]));
    dataset.push(Vector::new(vec![9.0, 9.0]));
    // dataset.push(Vector2D::new((1.0, 1.0)));
    // dataset.push(Vector2D::new((2.0, 2.0)));
    // dataset.push(Vector2D::new((3.0, 3.0)));
    // dataset.push(Vector2D::new((4.0, 4.0)));
    // dataset.push(Vector2D::new((5.0, 5.0)));
    // dataset.push(Vector2D::new((6.0, 6.0)));
    // dataset.push(Vector2D::new((7.0, 7.0)));
    // dataset.push(Vector2D::new((8.0, 8.0)));
    // dataset.push(Vector2D::new((9.0, 9.0)));

    let initial_clusters = dataset.generate_initial_clusters(2);
    println!(
        "final centroids 2 : {:?}",
        kmeans::kmeans(&dataset, initial_clusters, 0.02)
    );
}

// fn main() {
//     println!("Hello world");
// }