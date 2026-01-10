use crate::vector::Vec2;

const MAX_FORCE: f32 = 0.15;

#[derive(Clone)]
pub struct Predator {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub energy: f32,
    pub base_speed: f32,
    pub max_speed: f32,
    pub max_force: f32,
    pub pack_id: u32,
    pub is_leader: bool,
    pub generation: u32,
    pub kills: u32,
    pub target_index: Option<usize>,
}

impl Predator {
    pub fn new(x: f32, y: f32, pack_id: u32, generation: u32) -> Self {
        let angle = rand_f32() * std::f32::consts::TAU;
        Self {
            position: Vec2::new(x, y),
            velocity: Vec2::from_angle(angle, 2.0),
            acceleration: Vec2::zero(),
            energy: 100.0,
            base_speed: 3.5 + generation as f32 * 0.3,
            max_speed: 3.5 + generation as f32 * 0.3,
            max_force: 0.1 + generation as f32 * 0.02,
            pack_id,
            is_leader: false,
            generation,
            kills: 0,
            target_index: None,
        }
    }

    pub fn hunt(
        &mut self,
        boid_positions: &[(f32, f32, f32, f32, f32, bool, bool)], // x, y, vx, vy, energy, is_collapsed, is_tasty
        shelter_positions: &[(f32, f32, f32)], // x, y, radius
        day_phase: f32,
    ) -> Option<usize> {
        let agg = 1.0 + (1.0 - day_phase) * 0.5;
        self.max_speed = self.base_speed + (1.0 - day_phase) * 2.0;

        let px = self.position.x;
        let py = self.position.y;

        let mut target_idx: Option<usize> = None;
        let mut target_dist = f32::INFINITY;
        let max_dist_sq = 22500.0 * agg * agg;

        for (i, &(bx, by, _vx, _vy, energy, is_collapsed, is_tasty)) in boid_positions.iter().enumerate() {
            // Check if in shelter
            let mut in_shelter = false;
            for &(sx, sy, sr) in shelter_positions {
                let dx = sx - bx;
                let dy = sy - by;
                if dx * dx + dy * dy < sr * sr {
                    in_shelter = true;
                    break;
                }
            }
            if in_shelter { continue; }

            let dx = px - bx;
            let dy = py - by;
            let d = dx * dx + dy * dy;

            let mut priority = d;
            if is_tasty { priority *= 0.25; }
            if is_collapsed { priority *= 0.09; }
            priority *= (1.0 - energy / 100.0) * 0.5 + 0.5;

            if d < max_dist_sq && priority < target_dist {
                target_dist = priority;
                target_idx = Some(i);
            }
        }

        self.target_index = target_idx;

        if let Some(idx) = target_idx {
            let (tx, ty, tvx, tvy, _, _, _) = boid_positions[idx];
            // Predict position
            let pred_x = tx + tvx * 8.0;
            let pred_y = ty + tvy * 8.0;

            let mut sx = pred_x - px;
            let mut sy = pred_y - py;
            let sm = (sx * sx + sy * sy).sqrt();
            if sm > 0.0 { sx /= sm; sy /= sm; }

            sx = sx * self.max_speed - self.velocity.x;
            sy = sy * self.max_speed - self.velocity.y;
            let sm = (sx * sx + sy * sy).sqrt();
            let sl = self.max_force * agg;
            if sm > sl { sx = sx / sm * sl; sy = sy / sm * sl; }

            self.acceleration.x += sx;
            self.acceleration.y += sy;

            // Check catch distance
            let actual_dist = ((px - tx).powi(2) + (py - ty).powi(2)).sqrt();
            if actual_dist < 12.0 {
                return Some(idx);
            }
        }

        None
    }

    pub fn update(&mut self, width: f32, height: f32, obstacle_positions: &[(f32, f32)]) -> bool {
        self.energy -= 0.04;

        let px = self.position.x;
        let py = self.position.y;

        // Avoid obstacles
        for &(ox, oy) in obstacle_positions {
            let dx = px - ox;
            let dy = py - oy;
            let dsq = dx * dx + dy * dy;
            if dsq < 900.0 {
                let m = dsq.sqrt();
                if m > 0.0 {
                    self.acceleration.x += dx / m * MAX_FORCE * 3.0;
                    self.acceleration.y += dy / m * MAX_FORCE * 3.0;
                }
            }
        }

        self.velocity.add_mut(self.acceleration);
        self.velocity.limit_mut(self.max_speed);
        self.position.add_mut(self.velocity);
        self.acceleration.reset();

        // Wrap edges
        if self.position.x > width { self.position.x = 0.0; }
        else if self.position.x < 0.0 { self.position.x = width; }
        if self.position.y > height { self.position.y = 0.0; }
        else if self.position.y < 0.0 { self.position.y = height; }

        self.energy > 0.0
    }
}

fn rand_f32() -> f32 {
    static mut SEED: u32 = 67890;
    unsafe {
        SEED ^= SEED << 13;
        SEED ^= SEED >> 17;
        SEED ^= SEED << 5;
        (SEED as f32) / (u32::MAX as f32)
    }
}
