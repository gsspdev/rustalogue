pub mod envelope {
    struct Envelope {
       attack: f32,
       decay: f32,
       sustain: f32,
       release: f32
        }
}

trait TraitName {
    fn new(attack: f32, decay: f32, sustain: f32, release: f32) -> Self;
}

impl TraitName for envelope::Envelope {
    fn new(attack: f32, decay: f32, sustain: f32, release: f32) -> Self {
        Self {
            attack,
            decay,
            sustain,
            release
        }
    }
}