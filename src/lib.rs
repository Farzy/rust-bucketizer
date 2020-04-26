#[derive(Clone, Debug, PartialEq)]
struct Bucketizer {
    buckets: Vec<Bucket>
}

type Bucket = (Option<f64>, Option<f64>, f64);

impl Bucketizer {
    fn new() -> Self {
        Bucketizer {
            buckets: Vec::new()
        }
    }

    fn add_bucket(&mut self, min: Option<f64>, max: Option<f64>, value: f64) {

    }

    fn bucketize(&self, input: f64) -> Option<f64> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::Bucketizer;

    #[test]
    fn single_bucket_middle_values() {
        let mut bucketizer = Bucketizer::new();
        bucketizer.add_bucket(Some(0.0), Some(1.0), 0.5);
        assert_eq!(bucketizer.bucketize(0.1), Some(0.5));
        assert_eq!(bucketizer.bucketize(999.999), None);
    }

    #[test]
    fn single_bucket_end_values() {
        let mut bucketizer = Bucketizer::new();
        bucketizer.add_bucket(Some(0.0), Some(1.0), 0.5);
        assert_eq!(bucketizer.bucketize(0.0), Some(0.5));
        assert_eq!(bucketizer.bucketize(1.0), None);
    }
}
