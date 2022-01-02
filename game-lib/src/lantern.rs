use gdnative::api::{Light2D, Sprite};
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Sprite)]
pub struct Lantern {
    t: f32,
    #[property(default = 2.0)]
    init_energy: f32,
    #[property(default = 0.5)]
    frequency: f32
}

#[methods]
impl Lantern {
    fn new(_owner: &Sprite) -> Self {
        Lantern {
            t: 0.0,
            init_energy: 2.0,
            frequency: 0.5
        }
    }

    #[export]
    fn _ready(&mut self, owner: &Sprite) {
        
    }

    #[export]
    fn _process(&mut self, owner: &Sprite, delta: f32) {
        self.t += delta;
        let light = unsafe {owner.get_node_as::<Light2D>("light").unwrap()};
        light.set_energy((self.init_energy + (self.init_energy * self.frequency * self.t).sin()) as f64);
    }
}