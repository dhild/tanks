use rand::{self, Rng};
use specs;

#[derive(Debug)]
pub struct Terrain {
    pub max_height: usize,
    pub heightmap: Vec<u16>,
}

impl specs::Component for Terrain {
    type Storage = specs::HashMapStorage<Terrain>;
}

impl Terrain {
    pub fn generate(width: usize, height: usize, points: usize) -> Terrain {
        assert!(width > 3 && height > 3 && width < u16::max_value() as usize &&
                height < u16::max_value() as usize);

        let noise = Noise::new(width, height, points);

        let mut hmap = Vec::with_capacity(width);

        for x in 0..width {
            hmap.push(noise.interp(x) as u16);
        }

        Terrain {
            max_height: height,
            heightmap: hmap,
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
    fn new(width: usize, height: usize, count: usize) -> Noise {
        let min = (height as f64) * 0.3;
        let max = (height as f64) * 0.7;
        let mut rng = rand::thread_rng();
        let mut t = Vec::new();
        let mut p = Vec::new();
        let dx = width as f64 / count as f64;
        let mut last_t = -dx;
        t.push(last_t);
        p.push(rng.gen_range(min, max));
        while last_t <= (width as f64) {
            last_t += rng.gen_range(0.0, dx * 2.0);
            t.push(last_t);
            p.push(rng.gen_range(min, max));
        }
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

    fn find_t(&self, x: f64, min: usize, max: usize) -> usize {
        if max - min <= 1 {
            if self.t[max] <= x {
                return max;
            }
            return min;
        }
        let mid = min + ((max - min) / 2);
        if self.t[mid] <= x {
            self.find_t(x, mid, max)
        } else {
            self.find_t(x, min, mid)
        }
    }

    fn interp(&self, x: usize) -> f64 {
        let x = x as f64;
        let k0 = self.find_t(x, 1, self.t.len() - 3);
        let k1 = k0 + 1;
        let t = (x - self.t[k0]) / (self.t[k1] - self.t[k0]);
        let t2 = t * t;
        let t3 = t * t2;
        let p0 = self.p[k0] * (2.0 * t3 - 3.0 * t2 + 1.0);
        let m0 = self.m(k0) * (t3 - 2.0 * t2 + t);
        let p1 = self.p[k1] * (-2.0 * t3 + 3.0 * t2);
        let m1 = self.m(k1) * (t3 - t2);
        let y = p0 + m0 + p1 + m1;
        trace!("X: {}, Y: {}, K0: {}, K1: {}, T0: {}, T1: {}",
               x,
               y,
               k0,
               k1,
               self.t[k0],
               self.t[k1]);
        if y < self.min {
            self.min
        } else if y > self.max {
            self.max
        } else {
            y
        }
    }
}
