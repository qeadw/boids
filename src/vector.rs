/// High-performance 2D vector for simulation calculations
#[derive(Clone, Copy, Debug, Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    #[inline]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    #[inline]
    pub fn from_angle(angle: f32, magnitude: f32) -> Self {
        Self {
            x: angle.cos() * magnitude,
            y: angle.sin() * magnitude,
        }
    }

    #[inline]
    pub fn add(&self, other: Vec2) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    #[inline]
    pub fn sub(&self, other: Vec2) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    #[inline]
    pub fn mult(&self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    #[inline]
    pub fn div(&self, scalar: f32) -> Self {
        if scalar == 0.0 {
            Self::zero()
        } else {
            Self {
                x: self.x / scalar,
                y: self.y / scalar,
            }
        }
    }

    #[inline]
    pub fn mag_sq(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    #[inline]
    pub fn mag(&self) -> f32 {
        self.mag_sq().sqrt()
    }

    #[inline]
    pub fn dist_sq(&self, other: Vec2) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }

    #[inline]
    pub fn dist(&self, other: Vec2) -> f32 {
        self.dist_sq(other).sqrt()
    }

    #[inline]
    pub fn normalize(&self) -> Self {
        let m = self.mag();
        if m == 0.0 {
            Self::zero()
        } else {
            self.div(m)
        }
    }

    #[inline]
    pub fn limit(&self, max: f32) -> Self {
        let msq = self.mag_sq();
        if msq > max * max {
            self.normalize().mult(max)
        } else {
            *self
        }
    }

    // Mutable operations for hot paths
    #[inline]
    pub fn add_mut(&mut self, other: Vec2) {
        self.x += other.x;
        self.y += other.y;
    }

    #[inline]
    pub fn mult_mut(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
    }

    #[inline]
    pub fn limit_mut(&mut self, max: f32) {
        let msq = self.mag_sq();
        if msq > max * max {
            let m = msq.sqrt();
            self.x = self.x / m * max;
            self.y = self.y / m * max;
        }
    }

    #[inline]
    pub fn set(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    #[inline]
    pub fn reset(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
    }
}
