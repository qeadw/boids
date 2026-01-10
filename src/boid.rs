use crate::vector::Vec2;

const MAX_SPEED: f32 = 4.0;
const MAX_FORCE: f32 = 0.15;

/// Mutation flags stored as bitfields for efficiency
#[derive(Clone, Copy, Default)]
pub struct Mutations(u32);

impl Mutations {
    pub const GIANT: u32 = 1 << 0;
    pub const TINY: u32 = 1 << 1;
    pub const SPEEDY: u32 = 1 << 2;
    pub const GLOWING: u32 = 1 << 3;
    pub const TOUGH: u32 = 1 << 4;
    pub const LONGLIVED: u32 = 1 << 5;
    pub const FERTILE: u32 = 1 << 6;
    pub const CAMOUFLAGE: u32 = 1 << 7;
    pub const PLATED: u32 = 1 << 8;
    pub const NOCTURNAL: u32 = 1 << 9;
    pub const RAINBOW: u32 = 1 << 10;
    pub const RAVENOUS: u32 = 1 << 11;
    pub const ZEN: u32 = 1 << 12;
    pub const MAGNETIC: u32 = 1 << 13;
    pub const CANNIBAL: u32 = 1 << 14;
    pub const IMMORTAL: u32 = 1 << 15;
    pub const BIG_STOMACH: u32 = 1 << 16;
    pub const SMALL_STOMACH: u32 = 1 << 17;
    pub const TRAITOR: u32 = 1 << 18;
    pub const FAT: u32 = 1 << 19;
    pub const PAPER: u32 = 1 << 20;
    pub const FLIGHTLESS: u32 = 1 << 21;
    pub const MECHANICAL: u32 = 1 << 22;
    pub const TASTY: u32 = 1 << 23;
    pub const BULLIED: u32 = 1 << 24;
    pub const AGGRESSIVE: u32 = 1 << 25;

    #[inline]
    pub fn has(&self, flag: u32) -> bool {
        self.0 & flag != 0
    }

    #[inline]
    pub fn set(&mut self, flag: u32) {
        self.0 |= flag;
    }

    #[inline]
    pub fn clear(&mut self, flag: u32) {
        self.0 &= !flag;
    }

    #[inline]
    pub fn count(&self) -> u32 {
        self.0.count_ones()
    }

    #[inline]
    pub fn raw(&self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BoidState {
    Normal,
    Perching,
    Collapsed,
    Fishing,
}

#[derive(Clone)]
pub struct Boid {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub species: u8,
    pub is_hybrid: bool,
    pub hue: f32,
    pub energy: f32,
    pub max_energy: f32,
    pub fatigue: f32,
    pub fear: f32,
    pub age: u32,
    pub max_age: u32,
    pub state: BoidState,
    pub collapse_timer: u32,
    pub fish_timer: u32,
    pub mutations: Mutations,
    pub has_armor: bool,
    // Traits
    pub bravery: f32,
    pub hunger: f32,
    pub laziness: f32,
    pub sociability: f32,
    // Computed from mutations
    pub size_mult: f32,
    pub speed_mult: f32,
    pub fatigue_resistance: f32,
    pub energy_drain: f32,
}

impl Boid {
    pub fn new(x: f32, y: f32, species: u8, is_hybrid: bool) -> Self {
        let angle = rand_f32() * std::f32::consts::TAU;
        let hue = if is_hybrid {
            120.0 + rand_f32() * 30.0
        } else if species == 0 {
            190.0 + rand_f32() * 30.0
        } else {
            40.0 + rand_f32() * 30.0
        };

        let mut boid = Self {
            position: Vec2::new(x, y),
            velocity: Vec2::from_angle(angle, 2.0 + rand_f32() * 2.0),
            acceleration: Vec2::zero(),
            species,
            is_hybrid,
            hue,
            energy: 50.0 + rand_f32() * 50.0,
            max_energy: 100.0,
            fatigue: 0.0,
            fear: 0.0,
            age: 0,
            max_age: 8000 + (rand_f32() * 4000.0) as u32,
            state: BoidState::Normal,
            collapse_timer: 0,
            fish_timer: 0,
            mutations: Mutations::default(),
            has_armor: false,
            bravery: rand_f32(),
            hunger: rand_f32(),
            laziness: rand_f32(),
            sociability: rand_f32(),
            size_mult: 1.0,
            speed_mult: 1.0,
            fatigue_resistance: 1.0,
            energy_drain: 1.0,
        };
        boid.apply_mutation_effects();
        boid
    }

    pub fn apply_mutation_effects(&mut self) {
        self.size_mult = 1.0;
        if self.mutations.has(Mutations::GIANT) { self.size_mult = 1.5; }
        if self.mutations.has(Mutations::TINY) { self.size_mult = 0.6; }
        if self.mutations.has(Mutations::FAT) { self.size_mult = 1.3; }

        self.speed_mult = 1.0;
        if self.mutations.has(Mutations::SPEEDY) { self.speed_mult = 1.4; }
        if self.mutations.has(Mutations::TINY) { self.speed_mult *= 1.2; }
        if self.mutations.has(Mutations::GIANT) { self.speed_mult *= 0.85; }
        if self.mutations.has(Mutations::FAT) { self.speed_mult *= 0.7; }
        if self.mutations.has(Mutations::FLIGHTLESS) { self.speed_mult *= 0.5; }
        if self.mutations.has(Mutations::MECHANICAL) { self.speed_mult *= 1.1; }

        self.fatigue_resistance = if self.mutations.has(Mutations::TOUGH) {
            0.5
        } else if self.mutations.has(Mutations::MECHANICAL) {
            0.3
        } else if self.mutations.has(Mutations::PAPER) {
            2.0
        } else {
            1.0
        };

        self.max_energy = if self.mutations.has(Mutations::BIG_STOMACH) {
            200.0
        } else if self.mutations.has(Mutations::SMALL_STOMACH) {
            50.0
        } else {
            100.0
        };

        self.energy_drain = if self.mutations.has(Mutations::IMMORTAL) {
            3.0
        } else if self.mutations.has(Mutations::SMALL_STOMACH) {
            0.66
        } else if self.mutations.has(Mutations::MECHANICAL) {
            0.5
        } else {
            1.0
        };

        self.has_armor = self.mutations.has(Mutations::PLATED);

        if self.mutations.has(Mutations::ZEN) {
            self.bravery = (self.bravery + 0.4).min(1.0);
        }
        if self.mutations.has(Mutations::MAGNETIC) {
            self.sociability = (self.sociability + 0.5).min(1.0);
        }
        if self.mutations.has(Mutations::BULLIED) {
            self.sociability = (self.sociability - 0.4).max(0.0);
            self.fear = (self.fear + 0.3).min(1.0);
        }
    }

    pub fn flock(
        &mut self,
        nearby_boids: &[&Boid],
        predator_positions: &[(f32, f32)],
        obstacle_positions: &[(f32, f32)],
        shelter_positions: &[(f32, f32, f32)], // x, y, radius
        day_phase: f32,
    ) {
        if self.state != BoidState::Normal { return; }
        if self.fatigue > 150.0 {
            self.state = BoidState::Collapsed;
            self.collapse_timer = 100;
            self.velocity.reset();
            return;
        }

        let px = self.position.x;
        let py = self.position.y;
        let vx = self.velocity.x;
        let vy = self.velocity.y;

        // Separation, alignment, cohesion
        let mut sep_x = 0.0f32;
        let mut sep_y = 0.0f32;
        let mut ali_x = 0.0f32;
        let mut ali_y = 0.0f32;
        let mut coh_x = 0.0f32;
        let mut coh_y = 0.0f32;
        let mut sep_ct = 0u32;
        let mut ali_ct = 0u32;
        let mut coh_ct = 0u32;

        let coh_radius = 50.0 * (1.0 + (1.0 - day_phase) * 0.5 + self.sociability * 0.3);
        let coh_radius_sq = coh_radius * coh_radius;

        for other in nearby_boids {
            if std::ptr::eq(*other, self) || other.state != BoidState::Normal { continue; }

            let ox = other.position.x;
            let oy = other.position.y;
            let dx = px - ox;
            let dy = py - oy;
            let dsq = dx * dx + dy * dy;

            if dsq < 625.0 {
                let inv = 1.0 / (dsq + 0.001);
                sep_x += dx * inv;
                sep_y += dy * inv;
                sep_ct += 1;
            }

            let can_flock = self.is_hybrid || other.is_hybrid || other.species == self.species;
            if can_flock {
                if dsq < 2500.0 {
                    ali_x += other.velocity.x;
                    ali_y += other.velocity.y;
                    ali_ct += 1;
                }
                if dsq < coh_radius_sq {
                    coh_x += ox;
                    coh_y += oy;
                    coh_ct += 1;
                }
            }
        }

        let mut ax = 0.0f32;
        let mut ay = 0.0f32;

        // Apply separation
        if sep_ct > 0 {
            let mut sx = sep_x / sep_ct as f32;
            let mut sy = sep_y / sep_ct as f32;
            let sm = (sx * sx + sy * sy).sqrt();
            if sm > 0.0 { sx /= sm; sy /= sm; }
            sx = sx * MAX_SPEED - vx;
            sy = sy * MAX_SPEED - vy;
            let sm = (sx * sx + sy * sy).sqrt();
            if sm > MAX_FORCE { sx = sx / sm * MAX_FORCE; sy = sy / sm * MAX_FORCE; }
            ax += sx * 1.8;
            ay += sy * 1.8;
        }

        // Apply alignment
        if ali_ct > 0 {
            let mut lx = ali_x / ali_ct as f32;
            let mut ly = ali_y / ali_ct as f32;
            let lm = (lx * lx + ly * ly).sqrt();
            if lm > 0.0 { lx /= lm; ly /= lm; }
            lx = lx * MAX_SPEED - vx;
            ly = ly * MAX_SPEED - vy;
            let lm = (lx * lx + ly * ly).sqrt();
            if lm > MAX_FORCE { lx = lx / lm * MAX_FORCE; ly = ly / lm * MAX_FORCE; }
            ax += lx;
            ay += ly;
        }

        // Apply cohesion
        if coh_ct > 0 {
            let mut cx = coh_x / coh_ct as f32 - px;
            let mut cy = coh_y / coh_ct as f32 - py;
            let cm = (cx * cx + cy * cy).sqrt();
            if cm > 0.0 { cx /= cm; cy /= cm; }
            cx = cx * MAX_SPEED - vx;
            cy = cy * MAX_SPEED - vy;
            let cm = (cx * cx + cy * cy).sqrt();
            if cm > MAX_FORCE { cx = cx / cm * MAX_FORCE; cy = cy / cm * MAX_FORCE; }
            let sm = 1.0 + self.sociability * 0.3;
            ax += cx * sm;
            ay += cy * sm;
        }

        // Flee from predators
        let mut flee_x = 0.0f32;
        let mut flee_y = 0.0f32;
        self.fear = if self.mutations.has(Mutations::BULLIED) { 0.3 } else { 0.0 };
        let flee_radius = 100.0 + (1.0 - self.bravery) * 50.0;
        let flee_radius_sq = flee_radius * flee_radius;

        // Check if in shelter
        let mut in_shelter = false;
        for &(sx, sy, sr) in shelter_positions {
            let dx = sx - px;
            let dy = sy - py;
            if dx * dx + dy * dy < sr * sr {
                in_shelter = true;
                self.fear = if self.mutations.has(Mutations::BULLIED) { 0.2 } else { 0.0 };
                break;
            }
        }

        if !in_shelter {
            for &(pred_x, pred_y) in predator_positions {
                let dx = px - pred_x;
                let dy = py - pred_y;
                let dsq = dx * dx + dy * dy;
                if dsq < flee_radius_sq {
                    let d = dsq.sqrt();
                    let inv = 1.0 / (d + 0.001);
                    flee_x += dx * inv;
                    flee_y += dy * inv;
                    self.fear = self.fear.max(1.0 - d / flee_radius);
                }
            }
        }

        if flee_x != 0.0 || flee_y != 0.0 {
            let fm = (flee_x * flee_x + flee_y * flee_y).sqrt();
            if fm > 0.0 { flee_x /= fm; flee_y /= fm; }
            let fs = MAX_SPEED * (1.5 + self.bravery * 0.3);
            flee_x = flee_x * fs - vx;
            flee_y = flee_y * fs - vy;
            let fm = (flee_x * flee_x + flee_y * flee_y).sqrt();
            let fl = MAX_FORCE * 3.0;
            if fm > fl { flee_x = flee_x / fm * fl; flee_y = flee_y / fm * fl; }
            let flee_mult = if self.fear > 0.3 { 4.0 } else { 1.0 } * (4.0 - self.bravery);
            ax += flee_x * flee_mult;
            ay += flee_y * flee_mult;
        }

        // Avoid obstacles
        let mut obs_x = 0.0f32;
        let mut obs_y = 0.0f32;
        let obs_radius_sq = if self.fear > 0.3 { 6400.0 } else { 2500.0 };

        for &(ox, oy) in obstacle_positions {
            let dx = px - ox;
            let dy = py - oy;
            let dsq = dx * dx + dy * dy;
            if dsq < obs_radius_sq {
                let inv = 1.0 / (dsq + 0.001);
                obs_x += dx * inv;
                obs_y += dy * inv;
            }
        }

        if obs_x != 0.0 || obs_y != 0.0 {
            let om = (obs_x * obs_x + obs_y * obs_y).sqrt();
            if om > 0.0 { obs_x /= om; obs_y /= om; }
            let os = MAX_SPEED * if self.fear > 0.3 { 1.5 } else { 1.0 };
            obs_x = obs_x * os - vx;
            obs_y = obs_y * os - vy;
            let om = (obs_x * obs_x + obs_y * obs_y).sqrt();
            let ol = MAX_FORCE * if self.fear > 0.3 { 3.0 } else { 2.0 };
            if om > ol { obs_x = obs_x / om * ol; obs_y = obs_y / om * ol; }
            let obs_mult = if self.fear > 0.3 { 3.0 } else { 1.0 };
            ax += obs_x * obs_mult;
            ay += obs_y * obs_mult;
        }

        self.acceleration.x += ax;
        self.acceleration.y += ay;
    }

    pub fn update(&mut self, width: f32, height: f32, day_phase: f32) -> bool {
        self.age += 1;
        if self.age > self.max_age && !self.mutations.has(Mutations::IMMORTAL) {
            return false;
        }

        match self.state {
            BoidState::Fishing => {
                self.fish_timer = self.fish_timer.saturating_sub(1);
                if self.fish_timer == 0 {
                    self.state = BoidState::Normal;
                }
                return true;
            }
            BoidState::Perching => {
                self.fatigue = (self.fatigue - 0.8).max(0.0);
                self.energy -= 0.003 * self.energy_drain;
                if (day_phase > 0.5 && self.fatigue < 20.0) || self.fear > 0.5 {
                    self.state = BoidState::Normal;
                    let angle = rand_f32() * std::f32::consts::TAU;
                    self.velocity = Vec2::from_angle(angle, 2.0);
                }
                return self.energy > 0.0;
            }
            BoidState::Collapsed => {
                self.collapse_timer = self.collapse_timer.saturating_sub(1);
                if self.collapse_timer == 0 {
                    self.state = BoidState::Normal;
                    self.fatigue = 50.0;
                }
                self.energy -= 0.01 * self.energy_drain;
                return self.energy > 0.0;
            }
            BoidState::Normal => {}
        }

        // Energy and fatigue
        self.energy -= (0.012 + self.hunger * 0.005) * self.energy_drain;
        self.fatigue += (0.025 - self.laziness * 0.01) * self.fatigue_resistance;

        if self.fear > 0.0 {
            self.energy -= 0.04 * self.energy_drain;
            self.fatigue += 0.04 * self.fatigue_resistance;
        }

        if self.mutations.has(Mutations::NOCTURNAL) && day_phase < 0.4 {
            self.energy += 0.005;
            self.fatigue -= 0.01;
        }

        // Physics
        let age_slow = (1.0 - (self.age as f32 / self.max_age as f32) * 0.5).max(0.5);
        let speed_mult = (0.6 + day_phase * 0.4) * age_slow * self.speed_mult;
        let nocturnal_bonus = if self.mutations.has(Mutations::NOCTURNAL) && day_phase < 0.4 { 1.3 } else { 1.0 };
        let current_max_speed = MAX_SPEED * speed_mult * (1.0 + self.fear * 0.5) * nocturnal_bonus;

        self.velocity.add_mut(self.acceleration);
        self.velocity.limit_mut(current_max_speed);
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

// Simple random number generator (xorshift)
fn rand_f32() -> f32 {
    static mut SEED: u32 = 12345;
    unsafe {
        SEED ^= SEED << 13;
        SEED ^= SEED >> 17;
        SEED ^= SEED << 5;
        (SEED as f32) / (u32::MAX as f32)
    }
}

pub fn set_seed(seed: u32) {
    unsafe {
        static mut SEED: u32 = 12345;
        SEED = seed;
    }
}
