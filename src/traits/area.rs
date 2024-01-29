pub trait Area {
    fn calculate_area(&self) -> usize;
    fn calculate_perimeter(&self) -> usize;
}

pub trait RoundArea {
    fn calculate_circumference(&self) -> usize;
    fn calculate_area(&self) -> usize;
}
#[derive(Debug)]
pub struct Circle {
    pub radius: f32,
}

impl RoundArea for Circle {
    fn calculate_circumference(&self) -> usize {
        return (((self.radius as usize) * 22) / 7) * (2 as usize);
    }
    fn calculate_area(&self) -> usize {
        return ((self.radius.powf(2.0) as usize) * 22) / 7;
    }
}
#[derive(Debug)]
pub struct Triangle {
    pub base: i32,
    pub height: i32,
}

impl Area for Triangle {
    fn calculate_area(&self) -> usize {
        return ((self.base * self.height) / 2) as usize;
    }
    fn calculate_perimeter(&self) -> usize {
        let half_base = self.base / 2;
        let third_side_length: f64 = (half_base * half_base + self.height * self.height) as f64;
        (third_side_length.sqrt() as usize) * 2 + (self.base as usize)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Calculate;
impl Calculate {
    pub fn area<T>(shape: T) -> usize where T: Area {
        shape.calculate_area()
    }

    pub fn perimeter<T: Area>(shape: T) -> usize {
        shape.calculate_perimeter()
    }

    pub fn round_circumference<T: RoundArea>(shape: T) -> usize {
        shape.calculate_circumference()
    }
    pub fn round_area<T: RoundArea>(shape: T) -> usize {
        shape.calculate_area()
    }
}
