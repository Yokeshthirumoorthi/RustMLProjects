#![feature(test)]
extern crate test;
use kmeans::cluster::*;
use kmeans::dataset::*;
use kmeans::point::*;
use test::Bencher;

fn get_dataset() -> DataSet {
    let mut dataset: DataSet = Default::default();
    dataset.push(Point::new(vec![-114.635458, 34.876902]));
    dataset.push(Point::new(vec![-114.636768000000103, 34.885705]));
    dataset.push(Point::new(vec![-114.636725, 34.889107]));
    dataset.push(Point::new(vec![-114.635425, 34.895192]));
    dataset
}

#[bench]
fn bench_add_two(b: &mut Bencher) {
    let dataset = get_dataset();
    let p0 = dataset.items.get(0).unwrap();
    let p1 = dataset.items.get(1).unwrap();
    b.iter(|| p0.clone() + p1.clone());
}

#[bench]
fn bench_distance_two(b: &mut Bencher) {
    let dataset = get_dataset();
    let p0 = dataset.items.get(0).unwrap();
    let p1 = dataset.items.get(1).unwrap();
    b.iter(|| p0.clone().distance(p1.clone()));
}

#[bench]
fn bench_kmeans(b: &mut Bencher) {
    let dataset = get_dataset();
    b.iter(|| {
        let initial_clusters = dataset.generate_initial_clusters(2);
        kmeans::kmeans(&dataset, initial_clusters, 0.02)
    });
}
