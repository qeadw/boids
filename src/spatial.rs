use std::collections::HashMap;

/// Spatial hash for O(1) neighbor queries
pub struct SpatialHash {
    cell_size: f32,
    cells: HashMap<(i32, i32), Vec<usize>>,
}

impl SpatialHash {
    pub fn new(cell_size: f32) -> Self {
        Self {
            cell_size,
            cells: HashMap::new(),
        }
    }

    #[inline]
    fn get_key(&self, x: f32, y: f32) -> (i32, i32) {
        (
            (x / self.cell_size).floor() as i32,
            (y / self.cell_size).floor() as i32,
        )
    }

    pub fn clear(&mut self) {
        self.cells.clear();
    }

    pub fn insert(&mut self, index: usize, x: f32, y: f32) {
        let key = self.get_key(x, y);
        self.cells.entry(key).or_insert_with(Vec::new).push(index);
    }

    /// Returns indices of entities in nearby cells
    pub fn get_nearby(&self, x: f32, y: f32, radius: i32) -> Vec<usize> {
        let mut results = Vec::new();
        let (cx, cy) = self.get_key(x, y);

        for dx in -radius..=radius {
            for dy in -radius..=radius {
                let key = (cx + dx, cy + dy);
                if let Some(indices) = self.cells.get(&key) {
                    results.extend(indices.iter().copied());
                }
            }
        }
        results
    }

    /// More efficient version that takes a pre-allocated buffer
    pub fn get_nearby_into(&self, x: f32, y: f32, radius: i32, buffer: &mut Vec<usize>) {
        buffer.clear();
        let (cx, cy) = self.get_key(x, y);

        for dx in -radius..=radius {
            for dy in -radius..=radius {
                let key = (cx + dx, cy + dy);
                if let Some(indices) = self.cells.get(&key) {
                    buffer.extend(indices.iter().copied());
                }
            }
        }
    }
}
