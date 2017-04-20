use cgmath::Rad;
use cgmath::prelude::*;
use rand::{self, Rng};
use physics::Dimensions;

mod draw;

pub use self::draw::{Drawable, DrawSystem, PreDrawSystem};

pub fn generate(dim: &Dimensions, points: usize) -> Terrain {
    let noise = Noise::new(dim.game_width(), dim.game_height(), points);

    let mut hmap = Vec::with_capacity(dim.game_width() as usize);

    for x in 0..dim.game_width() {
        hmap.push(noise.interp(x) as u16);
    }

    Terrain {
        max_height: dim.game_height(),
        heightmap: hmap,
    }
}

#[derive(Debug)]
pub struct Terrain {
    pub max_height: u32,
    pub heightmap: Vec<u16>,
}

impl Terrain {
    pub fn get_height(&self, x: f32) -> f32 {
        match x.floor() {
            x if x < 0.0 => self.heightmap[0] as f32,
            x if x >= (self.heightmap.len() - 1) as f32 => *self.heightmap.last().unwrap() as f32,
            i => {
                let y0 = self.heightmap[i as usize] as f32;
                let y1 = self.heightmap[i as usize + 1] as f32;
                let t = x - (i as f32);
                y0 + t * (y1 - y0)
            }
        }
    }
    pub fn get_normal_dir(&self, x: f32) -> Rad<f32> {
        if x < 1.0 {
            self.get_normal_dir(1.0)
        } else if x > (self.heightmap.len() - 2) as f32 {
            self.get_normal_dir((self.heightmap.len() - 2) as f32)
        } else {
            let i = x.floor() as usize;
            let y0 = self.heightmap[i] as f32;
            let y1 = self.heightmap[i + 1] as f32;
            Rad::atan(y1 - y0)
        }
    }
}

struct Noise {
    min: f64,
    max: f64,
    t: Vec<f64>,
    p: Vec<f64>,
}

impl Noise {
    fn new(width: u32, height: u32, count: usize) -> Noise {
        let min = (height as f64) * 0.3;
        let max = (height as f64) * 0.7;
        let mut rng = rand::thread_rng();
        let mut t = Vec::new();
        let mut p = Vec::new();
        let dx = width as f64 / count as f64;
        t.push(-dx);
        p.push(rng.gen_range(min, max));
        let mut last_t = rng.gen_range(-dx, 0.0);
        t.push(last_t);
        p.push(rng.gen_range(min, max));
        while last_t <= (width as f64) {
            last_t += rng.gen_range(0.0, dx * 2.0);
            t.push(last_t);
            p.push(rng.gen_range(min, max));
        }
        last_t += rng.gen_range(0.0, dx * 2.0);
        t.push(last_t);
        p.push(rng.gen_range(min, max));
        Noise {
            min: (height as f64) * 0.2,
            max: (height as f64) * 0.8,
            t: t,
            p: p,
        }
    }

    fn m(&self, k: usize) -> f64 {
        let m1 = (self.p[k + 1] - self.p[k]) / (self.t[k + 1] - self.t[k]);
        let m2 = (self.p[k] - self.p[k - 1]) / (self.t[k] - self.t[k - 1]);
        0.5 * (m1 + m2)
    }

    fn find_k(&self, x: f64) -> usize {
        let mut i = 1;
        while i < (self.t.len() - 3) {
            if self.t[i + 1] > x {
                break;
            }
            i += 1;
        }
        i
    }

    fn interp(&self, x: u32) -> f64 {
        let x = x as f64;
        let k0 = self.find_k(x);
        let k1 = k0 + 1;
        let t = (x - self.t[k0]) / (self.t[k1] - self.t[k0]);
        let t2 = t * t;
        let t3 = t * t2;
        let p0 = self.p[k0] * (2.0 * t3 - 3.0 * t2 + 1.0);
        let m0 = self.m(k0) * (t3 - 2.0 * t2 + t);
        let p1 = self.p[k1] * (-2.0 * t3 + 3.0 * t2);
        let m1 = self.m(k1) * (t3 - t2);
        let y = p0 + m0 + p1 + m1;
        trace!("t: {}, Y: {}, K0: {}, K1: {}, P0: {}, P1: {}",
               t,
               y,
               k0,
               k1,
               self.p[k0],
               self.p[k1]);
        if y < self.min {
            debug!("Cutoff necessary: t: {}, Y: {}, K0: {}, K1: {}, P0: {}, P1: {}",
                   t,
                   y,
                   k0,
                   k1,
                   self.p[k0],
                   self.p[k1]);
            self.min
        } else if y > self.max {
            debug!("Cutoff necessary: t: {}, Y: {}, K0: {}, K1: {}, P0: {}, P1: {}",
                   t,
                   y,
                   k0,
                   k1,
                   self.p[k0],
                   self.p[k1]);
            self.max
        } else {
            y
        }
    }
}
