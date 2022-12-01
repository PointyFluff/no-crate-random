#![allow(unused)]
#![allow(unused_imports)]

use std::time::{SystemTime, UNIX_EPOCH};

pub struct PCG32 {
    state: u64,
    step: u64,
}

impl PCG32 {
    pub const STEP: u64 = 1442695040888963407;
    pub const MULT: u64 = 6364136223846793005;

    pub fn new() -> Self {
        let mut p: PCG32 = Self {
            state: 0,
            step: Self::STEP,
        };
        p.set_state();
        p
    }

    pub fn time_seed(mut self) -> Self {
        self.state = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() //as u64;
            .wrapping_shl(49) as u64;
        self
    }

    pub fn seed(mut self, seed: u64) -> Self {
        self.state = seed;
        self.set_state();
        self
    }

    pub fn step(mut self, step: u64) -> Self {
        self.step = step;
        self.set_state();
        self
    }

    pub fn rand(&mut self) -> u32 {
        let old_state: u64 = self.state;
        let new_state: u64 = old_state
            .wrapping_mul(Self::MULT)
            .wrapping_add((self.step << 1) | 1);

        let xor_shifted: u32 = ((old_state >> 18 ^ old_state) >> 27) as u32;
        let rot: u32 = (old_state >> 59) as u32;
        self.state = new_state;
        (xor_shifted.rotate_right(rot) | xor_shifted.rotate_left(rot.wrapping_neg() & 31))
            .wrapping_shl(1)
    }

    pub fn range(&mut self, bottom: u32, top: u32) -> u32 {
        let mut r: u32 = self.rand() % (top + 1);
        if r < bottom {
            r += bottom;
        }
        r
    }

    // ensure state is in a usable state.
    fn set_state(&mut self) {
        let old_state: u64 = self.state;
        self.state = old_state
            .wrapping_mul(Self::MULT)
            .wrapping_add((self.step << 1) | 1);
    }
}

mod tests {
    use super::*;

    #[test]
    fn not_same() {
        let mut pcg_a = PCG32::new().seed(314159);
        let mut pcg_b: PCG32 = PCG32::new().seed(314159);
        assert_eq!(pcg_a.rand(), pcg_b.rand());
    }

    #[test]
    fn no_birthday() {
        todo!("TODO: Write an actual test.")
    }
}

fn main() {}
