pub trait Filter {
    fn valid(&self, path: &str) -> bool;
}

pub struct FilterOpts {
    pub filters: Vec<Box<dyn Filter>>,
}