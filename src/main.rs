extern crate rustalogue;
extern crate meval;
use rustalogue::freq_filter::FreqFilter;

pub fn main() {
    let mut filter: FreqFilter = FreqFilter {
        filter_type: None,
        frequency: 520.0,
        qfactor: 0.72 };
    
    filter.set_filter_type("lowpass");
}
