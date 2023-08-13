// rustalogue::main

extern crate meval;

use rustalogue::oscillator as osc;
use rustalogue::FreqFilter;
// use rustalogue::envelope as enve;
// use rustalogue::lfo;

use osc::shape;
// use enve::settings as envSet;
// use lfo::rate as lfoRate;

pub fn main() {
    shape::sin();
    shape::saw();
    FreqFilter::giveType();

}
