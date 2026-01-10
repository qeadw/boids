use crate::boid::{Boid, BoidState, Mutations};
use crate::predator::Predator;
use crate::bug::Bug;
use crate::spatial::SpatialHash;

pub struct World {
    pub boids: Vec<Boid>,
    pub predators: Vec<Predator>,
    pub bugs: Vec<Bug>,
    pub obstacles: Vec<(f32, f32)>,
    pub shelters: Vec<(f32, f32, f32)>, // x, y, radius
    pub food_sources: Vec<(f32, f32, f32)>, // x, y, amount
    pub width: f32,
    pub height: f32,
    pub time: u32,
    pub day_time: f32,
    pub season_time: f32,
    spatial_hash: SpatialHash,
    next_pack_id: u32,
    // Reusable buffers
    nearby_buffer: Vec<usize>,
    // Cached boid data for flocking (avoids borrow issues)
    boid_cache: Vec<BoidCache>,
}

#[derive(Clone, Copy, Default)]
struct BoidCache {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    species: u8,
    is_hybrid: bool,
    state: u8, // 0=normal, 1=perching, 2=collapsed, 3=fishing
}

impl World {
    pub fn new(width: f32, height: f32, start_boids: u32) -> Self {
        let mut world = Self {
            boids: Vec::with_capacity(start_boids as usize * 2),
            predators: Vec::new(),
            bugs: Vec::new(),
            obstacles: Vec::new(),
            shelters: vec![
                (80.0, height - 100.0, 50.0),
                (width - 80.0, height - 100.0, 50.0),
            ],
            food_sources: Vec::new(),
            width,
            height,
            time: 0,
            day_time: 0.0,
            season_time: 0.0,
            spatial_hash: SpatialHash::new(50.0),
            next_pack_id: 0,
            nearby_buffer: Vec::with_capacity(100),
            boid_cache: Vec::with_capacity(start_boids as usize * 2),
        };

        // Create initial boids
        let num_s0 = (start_boids as f32 * 0.55) as u32;
        let num_s1 = start_boids - num_s0;

        for _ in 0..num_s0 {
            let x = rand_f32() * width;
            let y = rand_f32() * (height - 120.0);
            world.boids.push(Boid::new(x, y, 0, false));
        }

        for _ in 0..num_s1 {
            let x = rand_f32() * width;
            let y = rand_f32() * (height - 120.0);
            world.boids.push(Boid::new(x, y, 1, false));
        }

        world
    }

    pub fn tick(&mut self, cursor_x: f32, cursor_y: f32, cursor_mode: i32, cursor_strength: f32) {
        self.time += 16;
        self.day_time += 0.0015;
        let day_phase = (self.day_time.sin() + 1.0) / 2.0;
        self.season_time += 0.0002;

        // Rebuild spatial hash and cache boid data
        self.spatial_hash.clear();
        self.boid_cache.clear();
        for (i, boid) in self.boids.iter().enumerate() {
            self.spatial_hash.insert(i, boid.position.x, boid.position.y);
            self.boid_cache.push(BoidCache {
                x: boid.position.x,
                y: boid.position.y,
                vx: boid.velocity.x,
                vy: boid.velocity.y,
                species: boid.species,
                is_hybrid: boid.is_hybrid,
                state: match boid.state {
                    BoidState::Normal => 0,
                    BoidState::Perching => 1,
                    BoidState::Collapsed => 2,
                    BoidState::Fishing => 3,
                },
            });
        }

        // Spawn bugs occasionally
        let season_index = ((self.season_time % 1.0) * 4.0) as u32;
        let bug_rate = match season_index {
            1 => 0.15,
            3 => 0.03,
            _ => 0.08,
        };
        let max_bugs = match season_index {
            1 => 60,
            3 => 20,
            _ => 40,
        };

        if rand_f32() < bug_rate && self.bugs.len() < max_bugs {
            let x = rand_f32() * self.width;
            let y = rand_f32() * (self.height - 100.0);
            self.bugs.push(Bug::new(x, y));
        }

        // Update bugs
        self.bugs.retain_mut(|bug| bug.update(self.width, self.height, &self.obstacles));

        // Spawn food occasionally
        let food_chance = match season_index {
            1 => 0.003,
            3 => 0.0003,
            _ => 0.0015,
        };
        let is_day = day_phase > 0.45;
        if is_day && rand_f32() < food_chance && self.food_sources.len() < 5 {
            let x = 50.0 + rand_f32() * (self.width - 100.0);
            let y = 50.0 + rand_f32() * (self.height - 180.0);
            self.food_sources.push((x, y, 100.0));
        }

        // Update food sources
        self.food_sources.retain(|&(_, _, amount)| amount > 0.0);

        // Prepare data for predator hunting
        let boid_data: Vec<_> = self.boids.iter().map(|b| {
            (
                b.position.x,
                b.position.y,
                b.velocity.x,
                b.velocity.y,
                b.energy,
                b.state == BoidState::Collapsed,
                b.mutations.has(Mutations::TASTY),
            )
        }).collect();

        // Predators hunt
        let mut caught_indices = Vec::new();
        for predator in &mut self.predators {
            if let Some(idx) = predator.hunt(&boid_data, &self.shelters, day_phase) {
                if !caught_indices.contains(&idx) {
                    caught_indices.push(idx);
                    predator.energy = (predator.energy + 45.0).min(150.0);
                    predator.kills += 1;
                }
            }
        }

        // Remove caught boids
        caught_indices.sort_unstable();
        for idx in caught_indices.into_iter().rev() {
            self.boids.swap_remove(idx);
        }

        // Update predators
        self.predators.retain_mut(|pred| pred.update(self.width, self.height, &self.obstacles));

        // Get predator positions for boid flocking
        let predator_positions: Vec<_> = self.predators.iter()
            .map(|p| (p.position.x, p.position.y))
            .collect();

        // Update boids - use index-based approach to avoid borrow issues
        let boid_count = self.boids.len();
        for i in 0..boid_count {
            // Get nearby boids from cache
            self.spatial_hash.get_nearby_into(
                self.boid_cache[i].x,
                self.boid_cache[i].y,
                2,
                &mut self.nearby_buffer,
            );

            // Calculate flocking forces using cached data
            let (ax, ay) = self.calculate_flocking_forces(i, day_phase, &predator_positions);

            // Apply forces
            self.boids[i].acceleration.x += ax;
            self.boids[i].acceleration.y += ay;

            // Apply cursor force
            if cursor_mode != 0 {
                let dx = cursor_x - self.boids[i].position.x;
                let dy = cursor_y - self.boids[i].position.y;
                let dsq = dx * dx + dy * dy;
                if dsq < 10000.0 {
                    let d = dsq.sqrt();
                    let (mut ccx, mut ccy) = if cursor_mode == 1 { (dx, dy) } else { (-dx, -dy) };
                    let ccm = (ccx * ccx + ccy * ccy).sqrt();
                    if ccm > 0.0 { ccx /= ccm; ccy /= ccm; }
                    let force = 0.15 * 2.0 * cursor_strength * (1.0 - d / 100.0);
                    self.boids[i].acceleration.x += ccx * force;
                    self.boids[i].acceleration.y += ccy * force;
                }
            }
        }

        // Boids eat bugs - separate pass
        for boid in &mut self.boids {
            for bug in &mut self.bugs {
                let dx = boid.position.x - bug.position.x;
                let dy = boid.position.y - bug.position.y;
                if dx * dx + dy * dy < 144.0 {
                    bug.energy = 0.0;
                    boid.energy = (boid.energy + 8.0).min(boid.max_energy);
                    boid.fatigue = (boid.fatigue - 2.0).max(0.0);
                }
            }
        }

        // Boids interact with food - separate pass
        for boid in &mut self.boids {
            if boid.state != BoidState::Normal || boid.fear > 0.3 {
                continue;
            }
            for food in &mut self.food_sources {
                if food.2 <= 0.0 { continue; }
                let dx = food.0 - boid.position.x;
                let dy = food.1 - boid.position.y;
                let dsq = dx * dx + dy * dy;
                let hm = 0.5 + boid.hunger;
                if dsq < 4900.0 * hm * hm {
                    if dsq < 144.0 {
                        food.2 -= 0.4;
                        boid.energy = (boid.energy + 1.0).min(boid.max_energy);
                        boid.fatigue = (boid.fatigue - 0.3).max(0.0);
                    } else {
                        let d = dsq.sqrt();
                        let force = 0.15 * 0.4 * hm;
                        boid.acceleration.x += dx / d * force;
                        boid.acceleration.y += dy / d * force;
                    }
                }
            }
        }

        // Update boid physics
        self.boids.retain_mut(|boid| boid.update(self.width, self.height, day_phase));

        // Remove dead bugs
        self.bugs.retain(|bug| bug.energy > 0.0);
    }

    fn calculate_flocking_forces(&self, i: usize, day_phase: f32, predator_positions: &[(f32, f32)]) -> (f32, f32) {
        let boid = &self.boids[i];
        if boid.state != BoidState::Normal { return (0.0, 0.0); }

        let px = boid.position.x;
        let py = boid.position.y;
        let vx = boid.velocity.x;
        let vy = boid.velocity.y;
        let species = boid.species;
        let is_hybrid = boid.is_hybrid;
        let sociability = boid.sociability;
        let bravery = boid.bravery;
        let is_bullied = boid.mutations.has(Mutations::BULLIED);

        const MAX_SPEED: f32 = 4.0;
        const MAX_FORCE: f32 = 0.15;

        let mut sep_x = 0.0f32;
        let mut sep_y = 0.0f32;
        let mut ali_x = 0.0f32;
        let mut ali_y = 0.0f32;
        let mut coh_x = 0.0f32;
        let mut coh_y = 0.0f32;
        let mut sep_ct = 0u32;
        let mut ali_ct = 0u32;
        let mut coh_ct = 0u32;

        let coh_radius = 50.0 * (1.0 + (1.0 - day_phase) * 0.5 + sociability * 0.3);
        let coh_radius_sq = coh_radius * coh_radius;

        for &idx in &self.nearby_buffer {
            if idx == i || idx >= self.boid_cache.len() { continue; }
            let other = &self.boid_cache[idx];
            if other.state != 0 { continue; } // Not normal

            let dx = px - other.x;
            let dy = py - other.y;
            let dsq = dx * dx + dy * dy;

            if dsq < 625.0 {
                let inv = 1.0 / (dsq + 0.001);
                sep_x += dx * inv;
                sep_y += dy * inv;
                sep_ct += 1;
            }

            let can_flock = is_hybrid || other.is_hybrid || other.species == species;
            if can_flock {
                if dsq < 2500.0 {
                    ali_x += other.vx;
                    ali_y += other.vy;
                    ali_ct += 1;
                }
                if dsq < coh_radius_sq {
                    coh_x += other.x;
                    coh_y += other.y;
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
            let sm = 1.0 + sociability * 0.3;
            ax += cx * sm;
            ay += cy * sm;
        }

        // Flee from predators
        let mut flee_x = 0.0f32;
        let mut flee_y = 0.0f32;
        let mut fear: f32 = if is_bullied { 0.3 } else { 0.0 };
        let flee_radius = 100.0 + (1.0 - bravery) * 50.0;
        let flee_radius_sq = flee_radius * flee_radius;

        // Check if in shelter
        let mut in_shelter = false;
        for &(sx, sy, sr) in &self.shelters {
            let dx = sx - px;
            let dy = sy - py;
            if dx * dx + dy * dy < sr * sr {
                in_shelter = true;
                fear = if is_bullied { 0.2 } else { 0.0 };
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
                    fear = fear.max(1.0 - d / flee_radius);
                }
            }
        }

        if flee_x != 0.0 || flee_y != 0.0 {
            let fm = (flee_x * flee_x + flee_y * flee_y).sqrt();
            if fm > 0.0 { flee_x /= fm; flee_y /= fm; }
            let fs = MAX_SPEED * (1.5 + bravery * 0.3);
            flee_x = flee_x * fs - vx;
            flee_y = flee_y * fs - vy;
            let fm = (flee_x * flee_x + flee_y * flee_y).sqrt();
            let fl = MAX_FORCE * 3.0;
            if fm > fl { flee_x = flee_x / fm * fl; flee_y = flee_y / fm * fl; }
            let flee_mult = if fear > 0.3 { 4.0 } else { 1.0 } * (4.0 - bravery);
            ax += flee_x * flee_mult;
            ay += flee_y * flee_mult;
        }

        // Avoid obstacles
        let mut obs_x = 0.0f32;
        let mut obs_y = 0.0f32;
        let obs_radius_sq = if fear > 0.3 { 6400.0 } else { 2500.0 };

        for &(ox, oy) in &self.obstacles {
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
            let os = MAX_SPEED * if fear > 0.3 { 1.5 } else { 1.0 };
            obs_x = obs_x * os - vx;
            obs_y = obs_y * os - vy;
            let om = (obs_x * obs_x + obs_y * obs_y).sqrt();
            let ol = MAX_FORCE * if fear > 0.3 { 3.0 } else { 2.0 };
            if om > ol { obs_x = obs_x / om * ol; obs_y = obs_y / om * ol; }
            let obs_mult = if fear > 0.3 { 3.0 } else { 1.0 };
            ax += obs_x * obs_mult;
            ay += obs_y * obs_mult;
        }

        (ax, ay)
    }

    pub fn add_boid(&mut self, x: f32, y: f32, species: u8) {
        self.boids.push(Boid::new(x, y, species, false));
    }

    pub fn add_predator(&mut self, x: f32, y: f32) {
        self.predators.push(Predator::new(x, y, self.next_pack_id, 0));
        self.next_pack_id += 1;
    }

    pub fn add_obstacle(&mut self, x: f32, y: f32) {
        self.obstacles.push((x, y));
    }

    pub fn add_food(&mut self, x: f32, y: f32) {
        self.food_sources.push((x, y, 100.0));
    }

    pub fn get_stats(&self) -> (u32, u32, u32, f32) {
        (
            self.boids.len() as u32,
            self.predators.len() as u32,
            self.bugs.len() as u32,
            (self.day_time.sin() + 1.0) / 2.0,
        )
    }

    /// Returns flat array: [x, y, vx, vy, hue, energy, max_energy, size_mult, mutations, fear, state, ...]
    pub fn get_boid_render_data(&self) -> Vec<f32> {
        let mut data = Vec::with_capacity(self.boids.len() * 11);
        for boid in &self.boids {
            data.push(boid.position.x);
            data.push(boid.position.y);
            data.push(boid.velocity.x);
            data.push(boid.velocity.y);
            data.push(boid.hue);
            data.push(boid.energy);
            data.push(boid.max_energy);
            data.push(boid.size_mult);
            data.push(boid.mutations.raw() as f32);
            data.push(boid.fear);
            data.push(match boid.state {
                BoidState::Normal => 0.0,
                BoidState::Perching => 1.0,
                BoidState::Collapsed => 2.0,
                BoidState::Fishing => 3.0,
            });
        }
        data
    }

    /// Returns flat array: [x, y, vx, vy, energy, is_leader, generation, ...]
    pub fn get_predator_render_data(&self) -> Vec<f32> {
        let mut data = Vec::with_capacity(self.predators.len() * 7);
        for pred in &self.predators {
            data.push(pred.position.x);
            data.push(pred.position.y);
            data.push(pred.velocity.x);
            data.push(pred.velocity.y);
            data.push(pred.energy);
            data.push(if pred.is_leader { 1.0 } else { 0.0 });
            data.push(pred.generation as f32);
        }
        data
    }

    /// Returns flat array: [x, y, hue, size, ...]
    pub fn get_bug_render_data(&self) -> Vec<f32> {
        let mut data = Vec::with_capacity(self.bugs.len() * 4);
        for bug in &self.bugs {
            data.push(bug.position.x);
            data.push(bug.position.y);
            data.push(bug.hue);
            data.push(bug.size);
        }
        data
    }
}

fn rand_f32() -> f32 {
    static mut SEED: u32 = 99999;
    unsafe {
        SEED ^= SEED << 13;
        SEED ^= SEED >> 17;
        SEED ^= SEED << 5;
        (SEED as f32) / (u32::MAX as f32)
    }
}
