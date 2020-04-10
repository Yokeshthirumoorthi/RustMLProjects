use kmeans::cluster::*;
use kmeans::dataset::*;
use kmeans::point::*;

fn p0() -> Point {
    Point::new(vec![0.0, 0.0])
}
fn p1() -> Point {
    Point::new(vec![1.0, 1.0])
}
fn p2() -> Point {
    Point::new(vec![2.0, 2.0])
}
fn p3() -> Point {
    Point::new(vec![3.0, 3.0])
}
fn p4() -> Point {
    Point::new(vec![4.0, 4.0])
}

#[test]
fn point_operator_overloadings() {
    assert_eq!(p0() * p0(), p0());
    assert_eq!(p0() * p1(), p0());
    assert_eq!(p1() * p2(), p2());
    assert_eq!(p2() - p1(), p1());
    assert_eq!(p1() - p1(), p0());
    assert_eq!(p1() + p1(), p2());
    assert_eq!(p1() / 1.0, p1());
    assert_eq!(p0() / 1.0, p0());
}

#[test]
#[should_panic(expected = "Cannot divide by zero-valued `Rational`!")]
fn point_check_div_by_0() {
    p1() / 0.0;
}

#[test]
fn point_kmeans_math() {
    assert_eq!(p0().distance(p1()), 1.4142135);
    assert_eq!(p1().distance(p2()), 1.4142135);
    assert_eq!(p2().distance(p2()), 0.0);
}

#[test]
fn cluster_init_works() {
    let c0 = Cluster::from(p0());
    assert_eq!(
        c0,
        Cluster {
            centroid: p0(),
            points_count: 0,
            points_sum: p0(),
        }
    );
    assert_eq!(
        c0.push(p1()),
        Cluster {
            centroid: p0(),
            points_count: 1,
            points_sum: p0() + p1(),
        }
    );
}

#[test]
fn clusterset_init_works() {
    let mut c0 = Cluster::from(p0());
    let mut c1 = Cluster::from(p1());
    let mut clusterset = ClusterSet::new(vec![c0.clone(), c1.clone()]);

    assert_eq!(
        clusterset,
        ClusterSet {
            clusters: vec![c0.clone(), c1.clone()],
        }
    );

    assert_eq!(clusterset.find_nearest(p2()), c1.clone());

    c0 = c0.push(p0());
    clusterset = clusterset.update(c0);
    c1 = c1.push(p1());
    clusterset = clusterset.update(c1.clone());
    c1 = c1.push(p2());
    clusterset = clusterset.update(c1);
    
    let new_c0 = Cluster {
        centroid: p0(),
        points_count: 1,
        points_sum: p0(),
    };
    let new_c1 = Cluster {
        centroid: p1(),
        points_count: 2,
        points_sum: p1()+p2(),
    };
    let expected_clusterset = ClusterSet::new(vec![new_c0, new_c1]);

    assert_eq!(clusterset, expected_clusterset);
    
    assert_eq!(clusterset.delta(), 0.70710677);
}

#[test]
fn dataset_init_works() {
    let mut dataset: DataSet = Default::default();
    assert_eq!(dataset, DataSet { items: Vec::new() });
    dataset.push(p0());
    assert_eq!(dataset, DataSet { items: vec![p0()] });
    dataset.push(p1());
    assert_eq!(
        dataset,
        DataSet {
            items: vec![p0(), p1()]
        }
    );
}

#[test]
fn dataset_generates_initial_clusters() {
    let mut dataset: DataSet = Default::default();
    dataset.push(p0());
    dataset.push(p1());
    dataset.push(p2());

    let c0 = Cluster::from(p0());
    let c1 = Cluster::from(p1());
    let expected_clusterset = ClusterSet::new(vec![c0, c1]);
    assert_eq!(dataset.generate_initial_clusters(2), expected_clusterset);
}

#[test]
fn dataset_classifies_into_clusters() {
    let mut dataset: DataSet = Default::default();
    dataset.push(p0());
    dataset.push(p1());
    dataset.push(p2());
    dataset.push(p3());
    dataset.push(p4());

    let c1 = Cluster::from(p1());
    let c2 = Cluster::from(p2());
    let clusterset = ClusterSet::new(vec![c1, c2]);
    let c1 = Cluster::from(p1());
    let c2 = Cluster::from(p2());
    let expected_clusterset = ClusterSet::new(vec![c1.push(p0()).push(p1()), c2.push(p2()).push(p3()).push(p4())]);
    assert_eq!(dataset.classify_into(clusterset), expected_clusterset);
}

#[test]
#[should_panic(expected = "Dont ask for more clusters than total points in dataset!")]
fn clusters_to_dataset_size_check() {
    let mut dataset: DataSet = Default::default();
    dataset.push(p1());
    dataset.generate_initial_clusters(3);
}
