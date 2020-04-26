extern crate bucketizer;
use bucketizer::Bucketizer;

#[test]
fn multiple_buckets_open_end() {
    let bucketizer = Bucketizer::new()
        .bucket(Some(0.0), Some(1.0), 0.5)
        .bucket(Some(1.0), None, 1.5);
    assert_eq!(bucketizer.bucketize(0.0), Some(0.5));
    assert_eq!(bucketizer.bucketize(-0.7), None);
    assert_eq!(bucketizer.bucketize(999.999), Some(1.5));
}
