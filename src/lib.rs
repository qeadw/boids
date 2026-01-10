mod vector;
mod spatial;
mod boid;
mod predator;
mod bug;
mod world;

use wasm_bindgen::prelude::*;
use world::World;

#[cfg(feature = "console_error_panic_hook")]
#[wasm_bindgen(start)]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

/// The main simulation world exposed to JavaScript
#[wasm_bindgen]
pub struct WasmWorld {
    world: World,
}

#[wasm_bindgen]
impl WasmWorld {
    /// Create a new simulation world
    #[wasm_bindgen(constructor)]
    pub fn new(width: f32, height: f32, start_boids: u32) -> WasmWorld {
        WasmWorld {
            world: World::new(width, height, start_boids),
        }
    }

    /// Advance simulation by one tick
    /// cursor_mode: 0 = none, 1 = attract, 2 = repel
    pub fn tick(&mut self, cursor_x: f32, cursor_y: f32, cursor_mode: i32, cursor_strength: f32) {
        self.world.tick(cursor_x, cursor_y, cursor_mode, cursor_strength);
    }

    /// Get simulation statistics
    /// Returns: [boid_count, predator_count, bug_count, day_phase]
    pub fn get_stats(&self) -> Vec<f32> {
        let (boids, predators, bugs, day_phase) = self.world.get_stats();
        vec![boids as f32, predators as f32, bugs as f32, day_phase]
    }

    /// Get boid render data as flat Float32Array
    /// Format: [x, y, vx, vy, hue, energy, max_energy, size_mult, mutations, fear, state, ...]
    /// 11 floats per boid
    pub fn get_boid_data(&self) -> Vec<f32> {
        self.world.get_boid_render_data()
    }

    /// Get predator render data as flat Float32Array
    /// Format: [x, y, vx, vy, energy, is_leader, generation, ...]
    /// 7 floats per predator
    pub fn get_predator_data(&self) -> Vec<f32> {
        self.world.get_predator_render_data()
    }

    /// Get bug render data as flat Float32Array
    /// Format: [x, y, hue, size, ...]
    /// 4 floats per bug
    pub fn get_bug_data(&self) -> Vec<f32> {
        self.world.get_bug_render_data()
    }

    /// Add a boid at position
    pub fn add_boid(&mut self, x: f32, y: f32, species: u8) {
        self.world.add_boid(x, y, species);
    }

    /// Add multiple boids at position
    pub fn add_boids(&mut self, x: f32, y: f32, count: u32) {
        for i in 0..count {
            let ox = (i as f32 - count as f32 / 2.0) * 5.0 + (rand_f32() - 0.5) * 20.0;
            let oy = (rand_f32() - 0.5) * 20.0;
            let species = if rand_f32() > 0.5 { 0 } else { 1 };
            self.world.add_boid(x + ox, y + oy, species);
        }
    }

    /// Add a predator at position
    pub fn add_predator(&mut self, x: f32, y: f32) {
        self.world.add_predator(x, y);
    }

    /// Add an obstacle at position
    pub fn add_obstacle(&mut self, x: f32, y: f32) {
        self.world.add_obstacle(x, y);
    }

    /// Add a food source at position
    pub fn add_food(&mut self, x: f32, y: f32) {
        self.world.add_food(x, y);
    }

    /// Get current width
    pub fn width(&self) -> f32 {
        self.world.width
    }

    /// Get current height
    pub fn height(&self) -> f32 {
        self.world.height
    }

    /// Reset the world
    pub fn reset(&mut self, width: f32, height: f32, start_boids: u32) {
        self.world = World::new(width, height, start_boids);
    }

    /// Get obstacle positions as flat array [x, y, x, y, ...]
    pub fn get_obstacle_data(&self) -> Vec<f32> {
        let mut data = Vec::with_capacity(self.world.obstacles.len() * 2);
        for &(x, y) in &self.world.obstacles {
            data.push(x);
            data.push(y);
        }
        data
    }

    /// Get food source data [x, y, amount, ...]
    pub fn get_food_data(&self) -> Vec<f32> {
        let mut data = Vec::with_capacity(self.world.food_sources.len() * 3);
        for &(x, y, amount) in &self.world.food_sources {
            data.push(x);
            data.push(y);
            data.push(amount);
        }
        data
    }
}

fn rand_f32() -> f32 {
    static mut SEED: u32 = 77777;
    unsafe {
        SEED ^= SEED << 13;
        SEED ^= SEED >> 17;
        SEED ^= SEED << 5;
        (SEED as f32) / (u32::MAX as f32)
    }
}
