pub enum FilterType {
    LowPass,
    HighPass,
    BandPass,
}

pub struct FreqFilter {
    pub filter_type: Option<FilterType>,
    pub frequency: f64,
    pub qfactor: f64,
}

impl FreqFilter {
    pub fn new() -> FreqFilter {
        FreqFilter {
            filter_type: None,
            frequency: 440.0,
            qfactor: 0.71,
        }
    }

    pub fn print_values(&self) {
        // println!("Filter Type: {}", self.filter_type.as_ref().map(|ft| ft.to_string()).unwrap_or_else(|| "None".to_string()));
        println!("Frequency: {}", self.frequency);
        println!("Q Factor: {}", self.qfactor);
    }

    pub fn set_filter_type(&mut self, filter: &str) {
        self.filter_type = match filter {
            "lowpass" => Some(FilterType::LowPass),
            "highpass" => Some(FilterType::HighPass),
            "bandpass" => Some(FilterType::BandPass),
            _ => None,
        }
    }
}

impl Default for FreqFilter {
    fn default() -> Self {
        FreqFilter { filter_type: std::option::Option::Some(<FilterType>::BandPass), frequency: (240.24), qfactor: (0.64) }
    }
}