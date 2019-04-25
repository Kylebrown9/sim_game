use std::time::Duration;

#[derive(Default)]
pub struct DeltaTime(pub f32);

impl DeltaTime {
    pub fn new(duration: f32) -> Self {
        DeltaTime(duration)
    }
}
