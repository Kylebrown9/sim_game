use specs::prelude::*;

use std::time::Duration;

use nalgebra::{ Vector2 };

type Num = f32;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position( Vector2<Num> );

impl Position {
    pub fn new(x: Num, y: Num) -> Self {
        Position( Vector2::new(x, y) )
    }

    pub fn accumulate_v(&mut self, v: Velocity, t: Duration) {
        *self += ??? 
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
// Represented by a Vector representing displacement in meters
// and a delta_t
pub struct Velocity {
    distance: Vector2<Num>,
    delta_t: Duration
}

impl Velocity {
    pub fn meters_per_second(x: Num, y: Num) -> Self {
        Velocity {
            distance: Vector2::new(x, y),
            delta_t: Duration::new(1, 0)
        }
    }
}