pub struct FreqFilter {
    pub filter_type: Vec<String>,
}

impl FreqFilter {
    pub fn new() -> FreqFilter {
        FreqFilter {
            filter_type: Vec::new(),
        }
    }

    pub fn give_type(&self) {
        for filter_type in &self.filter_type {
            println!("{}", filter_type);
        }
    }
}

#[test]
pub fn test_freq_filter() {
    let freq_filter = FreqFilter::new();
    freq_filter.give_type();
}
