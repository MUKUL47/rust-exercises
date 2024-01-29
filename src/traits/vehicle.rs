pub trait Vehicle {
    fn start(&self);
    fn stop(&self);
}
pub trait Bike {
    fn is_sport_bike(&self) -> bool;
}
pub trait Car {
    fn is_manual(&self) -> bool;
    fn is_hatch_back(&self) -> bool;
}

struct Ford {
    model: i32,
    speed: i64,
    is_manual: bool,
}

impl Vehicle for Ford {
    fn start(&self) {}
    fn stop(&self) {}
}
impl Car for Ford {
    fn is_hatch_back(&self) -> bool {
        return false;
    }
    fn is_manual(&self) -> bool {
        true
    }
}

#[derive(Default)]
pub struct FourWheel {
    is_running: bool,
}
impl FourWheel {
    pub fn new() -> Self {
        FourWheel {
            ..Default::default()
        }
    }
    pub fn start_driving(&mut self) {
        self.is_running = true;
    }
    pub fn stop_driving(&mut self) {
        self.is_running = false;
    }
}
