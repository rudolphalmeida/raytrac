use cgmath::dot;
use cgmath::prelude::*;
use cgmath::vec3;
use cgmath::Vector3;
use rand::prelude::*;

use super::Textured;

pub struct Perlin {
    pub ranvec: Vec<Vector3<f64>>,
    pub perm_x: Vec<usize>,
    pub perm_y: Vec<usize>,
    pub perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new() -> Self {
        Perlin {
            ranvec: Perlin::perlin_generate(),
            perm_x: Perlin::perlin_generate_perm(),
            perm_y: Perlin::perlin_generate_perm(),
            perm_z: Perlin::perlin_generate_perm(),
        }
    }

    fn perlin_generate() -> Vec<Vector3<f64>> {
        let mut rng = thread_rng();
        let mut p: Vec<Vector3<f64>> = Vec::with_capacity(256);
        for _i in 0..256 {
            p.push(
                vec3::<f64>(
                    -1.0 + 2.0 * rng.gen::<f64>(),
                    -1.0 + 2.0 * rng.gen::<f64>(),
                    -1.0 + 2.0 * rng.gen::<f64>(),
                )
                .normalize(),
            );
        }

        p
    }

    fn permute(p: &mut [usize], n: usize) {
        let mut rng = thread_rng();
        for i in (0..n).rev() {
            let target = (rng.gen::<f64>() * (i + 1) as f64) as usize;
            p.swap(i, target);
        }
    }

    fn perlin_generate_perm() -> Vec<usize> {
        let mut p: Vec<usize> = Vec::with_capacity(256);
        for i in 0..256 {
            p.push(i);
        }
        Perlin::permute(&mut p, 256);

        p
    }

    pub fn noise(&self, p: Vector3<f64>) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();
        let u = u * u * (3.0 - 2.0 * u);
        let v = v * v * (3.0 - 2.0 * v);
        let w = w * w * (3.0 - 2.0 * w);
        let i = p.x.floor();
        let j = p.y.floor();
        let k = p.z.floor();

        let mut c: [[[Vector3<f64>; 2]; 2]; 2] = [
            [
                [vec3::<f64>(0.0, 0.0, 0.0), vec3::<f64>(0.0, 0.0, 0.0)],
                [vec3::<f64>(0.0, 0.0, 0.0), vec3::<f64>(0.0, 0.0, 0.0)],
            ],
            [
                [vec3::<f64>(0.0, 0.0, 0.0), vec3::<f64>(0.0, 0.0, 0.0)],
                [vec3::<f64>(0.0, 0.0, 0.0), vec3::<f64>(0.0, 0.0, 0.0)],
            ],
        ];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[self.perm_x[(i as usize + di) & 255]
                        ^ self.perm_y[(j as usize + dj) & 255]
                        ^ self.perm_z[(k as usize + dk) & 255]];
                }
            }
        }

        perlin_interp(&c, u, v, w)
    }

    fn turb(&self, p: Vector3<f64>, depth: usize) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;
        for _i in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }
}

pub struct NoiseTexture {
    pub noise: Perlin,
    pub scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        NoiseTexture {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Textured for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: Vector3<f64>) -> Vector3<f64> {
        vec3::<f64>(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + (self.scale * p.z + 10.0 * self.noise.turb(p, 7)).sin())
    }
}

fn perlin_interp(c: &[[[Vector3<f64>; 2]; 2]], u: f64, v: f64, w: f64) -> f64 {
    let mut accum = 0.0;
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight = vec3::<f64>(u - i as f64, v - j as f64, w - k as f64);
                accum += (i as f64 * uu + (1 - i) as f64 * (1.0 - uu))
                    * (j as f64 * vv + (1 - j) as f64 * (1.0 - vv))
                    * (k as f64 * ww + (1 - k) as f64 * (1.0 - ww))
                    * dot(c[i][j][k], weight);
            }
        }
    }

    accum
}
