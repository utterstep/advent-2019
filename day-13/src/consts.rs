// total 1196 tiles on screen. Screen coords: x = [0..45], y = [0..25]

pub const MIN_X: usize = 0;
pub const MAX_X: usize = 45;
pub const X_SPAN: usize = MAX_X - MIN_X + 1;

pub const MIN_Y: usize = 0;
pub const MAX_Y: usize = 25;

pub const SEGMENT_X: i64 = -1;
pub const SEGMENT_Y: i64 = 0;

pub const TILE_CHUNK_SIZE: usize = 3;
