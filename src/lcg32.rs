#![allow(unused)]
#![allow(unused_imports)]
use std::fs::File;
use std::io;
use std::io::prelude::*;

pub struct Lcg32 {
    state: u64,
    scaler: u64,
    step: u64,
    modulus: u64,
}

impl Lcg32 {
    pub fn new(state: u64, scaler: u64, step: u64, modulus: u64) -> Self {
        let mut lcg32 = Self {
            state,
            scaler,
            step,
            modulus,
        };
        let _ = lcg32.rand();
        lcg32
    }

    pub fn rand(&mut self) -> u32 {
        self.state = self.state.wrapping_mul(self.scaler).wrapping_add(self.step) % self.modulus;
        self.state as u32
    }

    pub fn salt() -> u64 {
        let mut file = File::open("/dev/urandom").expect("The fuck? Where'd /dev/urandom go?");
        let mut buffer = [0; 8];
        let mut salt: u64 = 0;
        file.read_exact(&mut buffer)
            .expect("Unable to read from /dev/urandom");
        for i in 0..buffer.len() {
            salt += (buffer[i] as u64) << (i * 8); //build us a u64, one byte atta time.
        }
        salt
    }
}

impl Default for Lcg32 {
    fn default() -> Self {
        let mut lcg32 = Self {
            state: Self::salt(),
            scaler: Self::salt(),
            step: Self::salt(),
            modulus: Self::salt(),
        };
        let _ = lcg32.rand();
        lcg32
    }
}

impl Iterator for Lcg32 {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.rand())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SET_SIZE: u32 = 1000000;

    #[test]
    fn it_works() {
        let mut rng: Lcg32 = Default::default();
        for i in 0..18 {
            if i % 3 == 0 {
                println!();
                print!("\t")
            }
            print!("{}, ", rng.rand());
        }
        println!();
    }

    #[test]
    fn standard_deviation_default() {
        let mut rng: Lcg32 = Default::default();
        let mut set: Vec<u32> = Vec::new();
        let mut sum: u64 = 0;

        const SET_SIZE: i32 = 10000000;

        for s in 0..SET_SIZE {
            let r = rng.rand();
            sum += r as u64;
            set.push(r);
        }
        let avg = sum as f64 / SET_SIZE as f64;
        let mut square_diff_set: Vec<u64> = Vec::new();
        let mut diff_sum: f64 = 0.0;
        for i in set {
            diff_sum += (i as f64 - avg).powi(2);
        }
        let sigma = (diff_sum / SET_SIZE as f64).sqrt() as u64;
        println!("default set sigma: {}", sigma);
    }

    #[test]
    fn standard_deviation_seeded() {
        let mut rng: Lcg32 = Lcg32 {
            state: 42,
            ..Default::default()
        };
        let mut set: Vec<u32> = Vec::new();
        let mut sum: u64 = 0;
        for s in 0..SET_SIZE {
            let r = rng.rand();
            sum += r as u64;
            set.push(r);
        }
        let avg = sum as f64 / SET_SIZE as f64;
        let mut square_diff_set: Vec<u64> = Vec::new();
        let mut diff_sum: f64 = 0.0;
        for i in set {
            diff_sum += (i as f64 - avg).powi(2);
        }
        let sigma = (diff_sum / SET_SIZE as f64).sqrt() as u64;
        println!("seeded set sigma: {}", sigma);
    }

    #[test]
    fn standard_deviation_no_salt() {
        let mut rng: Lcg32 = Lcg32::new(1, 2, 3, 4);
        let mut set: Vec<u32> = Vec::new();
        let mut sum: u64 = 0;
        for s in 0..SET_SIZE {
            let r = rng.rand();
            sum += r as u64;
            set.push(r);
        }
        let avg = sum as f64 / SET_SIZE as f64;
        let mut square_diff_set: Vec<u64> = Vec::new();
        let mut diff_sum: f64 = 0.0;
        for i in set {
            diff_sum += (i as f64 - avg).powi(2);
        }
        let sigma = (diff_sum / SET_SIZE as f64).sqrt() as u64;
        println!("no salt set sigma: {}", sigma);
    }
}
