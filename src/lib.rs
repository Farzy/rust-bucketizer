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
