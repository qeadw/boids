use crate::vector::Vec2;

#[derive(Clone)]
pub struct Bug {
    pub position: Vec2,
    pub velocity: Vec2,
    pub energy: f32,
    pub size: f32,
    pub hue: f32,
    pub lifetime: u32,
}

impl Bug {
    pub fn new(x: f32, y: f32) -> Self {
        let angle = rand_f32() * std::f32::consts::TAU;
        Self {
            position: Vec2::new(x, y),
            velocity: Vec2::from_angle(angle, 1.0 + rand_f32()),
            energy: 20.0,
            size: 2.0 + rand_f32() * 2.0,
            hue: if rand_f32() > 0.5 { 60.0 } else { 120.0 },
            lifetime: 500 + (rand_f32() * 500.0) as u32,
        }
    }

    pub fn update(&mut self, width: f32, height: f32, obstacle_positions: &[(f32, f32)]) -> bool {
        self.lifetime = self.lifetime.saturating_sub(1);
        if self.lifetime == 0 || self.energy <= 0.0 {
            return false;
        }

        // Random steering
        let angle = rand_f32() * std::f32::consts::TAU;
        let steer = Vec2::from_angle(angle, 0.15);
        self.velocity.add_mut(steer);
        self.velocity.limit_mut(2.0);

        // Avoid obstacles
        let px = self.position.x;
        let py = self.position.y;
        let next_x = px + self.velocity.x;
        let next_y = py + self.velocity.y;

        for &(ox, oy) in obstacle_positions {
            let dx = next_x - ox;
            let dy = next_y - oy;
            if dx * dx + dy * dy < 18.0 * 18.0 {
                let dx = px - ox;
                let dy = py - oy;
                let m = (dx * dx + dy * dy).sqrt();
                if m > 0.0 {
                    self.velocity.set(dx / m * 2.0, dy / m * 2.0);
                }
                break;
            }
        }

        self.position.add_mut(self.velocity);

        // Wrap edges
        if self.position.x < 0.0 { self.position.x = width; }
        if self.position.x > width { self.position.x = 0.0; }
        if self.position.y < 0.0 { self.position.y = height; }
        if self.position.y > height { self.position.y = 0.0; }

        true
    }
}

fn rand_f32() -> f32 {
    static mut SEED: u32 = 11111;
    unsafe {
        SEED ^= SEED << 13;
        SEED ^= SEED >> 17;
        SEED ^= SEED << 5;
        (SEED as f32) / (u32::MAX as f32)
    }
}
