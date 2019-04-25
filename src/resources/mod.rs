use std::time::Duration;

#[derive(Default)]
pub struct DeltaTime(pub Duration);

impl DeltaTime {
    pub fn new(secs: u64, nanos: u32) -> Self {
        DeltaTime(Duration::new(secs, nanos))
    }
}
