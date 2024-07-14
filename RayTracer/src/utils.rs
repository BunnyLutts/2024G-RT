use lazy_static::lazy_static;
use rand::{
    distributions::{Open01, Uniform},
    thread_rng, Rng,
};
use std::{iter::Sum, mem::swap, ops::{Add, AddAssign}};
use std::cmp::Ordering;

pub fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}

pub fn rand01() -> f64 {
    thread_rng().sample::<f64, _>(Open01)
}

pub fn linear_to_gamma(x: f64) -> f64 {
    if x > 0.0 {
        x.sqrt()
    } else {
        x
    }
}

// Sorting by quicksort
pub fn inplace_sort<T>(v: &mut Vec<T>, begin: usize, end: usize, cmp_fn: &impl Fn(&T, &T) -> std::cmp::Ordering) {
    if end - begin < 2 {
        return; 
    }

    let pivot_index = partition(v, begin, end, cmp_fn);
    inplace_sort(v, begin, pivot_index, cmp_fn); 
    inplace_sort(v, pivot_index + 1, end, cmp_fn); 
}

fn partition<T>(v: &mut Vec<T>, begin: usize, end: usize, cmp_fn: &impl Fn(&T, &T) -> std::cmp::Ordering) -> usize {
    let p = thread_rng().gen_range(begin..end);
    v.swap(p, end - 1);  
    let mut i = begin;
    for j in begin..end - 1 {
        if cmp_fn(&v[j], &v[end-1]) != std::cmp::Ordering::Greater {
            v.swap(i, j);
            i += 1;
        }
    }
    v.swap(i, end - 1); 
    i 
}


#[cfg(test)]
mod tests_sort {
    use crate::utils::inplace_sort;

    #[test]
    fn test_inplace_sort() {
        let mut v = vec![3, 1, 2, 4, 5];
        inplace_sort(&mut v, 0, 5, &|a, b| a.cmp(b));
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
        let mut v = vec![5, 4, 3, 2, 1];
        inplace_sort(&mut v, 0, 2, &|a, b| a.cmp(b));
        assert_eq!(v, vec![4, 5, 3, 2, 1]);

        let mut v = vec![3, 6, 4, 5, 2, 1, 8, 7];
        inplace_sort(&mut v, 0, 4, &|a, b| a.cmp(b));
        inplace_sort(&mut v, 4, 8, &|a, b| a.cmp(b));
        assert_eq!(v, vec![3, 4, 5, 6, 1, 2, 7, 8])
    }
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn ones() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn squared_length(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn norm(&self) -> f64 {
        self.squared_length().sqrt()
    }

    pub fn rgb(&self) -> [u8; 3] {
        [
            (linear_to_gamma(self.x.max(0.0).min(0.999)) * 255.99) as u8,
            (linear_to_gamma(self.y.max(0.0).min(0.999)) * 255.99) as u8,
            (linear_to_gamma(self.z.max(0.0).min(0.999)) * 255.99) as u8,
        ]
    }

    pub fn normalize(&self) -> Self {
        let len = self.squared_length().sqrt();
        *self / len
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn random_xyz01() -> Self {
        Self::new(rand01(), rand01(), rand01())
    }

    pub fn random_in(min: f64, max: f64) -> Self {
        let mut rngs = thread_rng();
        let dist = Uniform::new(min, max);
        Self::new(rngs.sample(dist), rngs.sample(dist), rngs.sample(dist))
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_in(-1.0, 1.0);
            if p.squared_length() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit() -> Self {
        Self::random_in_unit_sphere().normalize()
    }

    pub fn random_in_unit_hemisphere(normal: &Self) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn random_in_unit_disk() -> Self {
        loop {
            let v = Self::new(rand01()*2.0-1.0, rand01()*2.0-1.0, 0.0);
            if v.squared_length() < 1.0 {
                return v;
            }
        }
    }

    pub fn near_zero(&self) -> bool {
        const EPS: f64 = 1e-8;
        self.x.abs() < EPS && self.y.abs() < EPS && self.z.abs() < EPS
    }

    pub fn reflect(&self, normal: &Self) -> Self {
        *self - 2.0 * self.dot(normal) * *normal
    }

    pub fn refract(&self, normal: &Self, ni_over_nt: f64) -> Self {
        let cos_theta = (-self.dot(normal)).min(1.0);
        let r_out_perp = ni_over_nt * (*self + *normal * cos_theta);
        let r_out_parallel = -(1.0 - r_out_perp.squared_length()).abs().sqrt() * *normal;
        r_out_perp + r_out_parallel
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;

    fn add(self, other: f64) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Sum for Vec3 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut sum = Vec3::zero();
        for v in iter {
            sum += v;
        }
        sum
    }
}

pub fn v3(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3::new(x, y, z)
}

#[cfg(test)]
mod tests_vec3 {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_add() {
        assert_eq!(
            Vec3::new(1.0, 0.0, -1.0) + Vec3::new(2.0, 4.0, 6.0),
            Vec3::new(3.0, 4.0, 5.0)
        )
    }

    #[test]
    fn test_add_assign() {
        let mut x = Vec3::new(1.0, 0.0, -1.0);
        x += Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(x, Vec3::new(3.0, 4.0, 5.0))
    }

    #[test]
    fn test_add_f64() {
        assert_eq!(
            Vec3::new(1.0, 0.0, -1.0) + 233.0,
            Vec3::new(234.0, 233.0, 232.0)
        )
    }

    /*
    #[test]
    fn test_add_assign_f64() {
        let mut x = Vec3::new(1.0, 0.0, -1.0);
        x += 233.0;
        assert_eq!(x, Vec3::new(234.0, 233.0, 232.0))
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            Vec3::new(1.0, 0.0, -1.0) - Vec3::new(2.0, 4.0, 6.0),
            Vec3::new(-1.0, -4.0, -7.0)
        )
    }

    #[test]
    fn test_sub_assign() {
        let mut x = Vec3::new(1.0, 0.0, -1.0);
        x -= Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(x, Vec3::new(-1.0, -4.0, -7.0))
    }

    #[test]
    fn test_sub_f64() {
        assert_eq!(Vec3::new(1.0, 0.0, -1.0) - 1.0, Vec3::new(0.0, -1.0, -2.0))
    }

    #[test]
    fn test_sub_assign_f64() {
        let mut x = Vec3::new(1.0, 0.0, -1.0);
        x -= 1.0;
        assert_eq!(x, Vec3::new(0.0, -1.0, -2.0))
    }

    #[test]
    fn test_mul() {
        assert_eq!(Vec3::new(1.0, 0.0, -1.0) * Vec3::ones(), 0.0);
    }

    #[test]
    fn test_mul_assign() {
        let mut x = Vec3::new(1.0, 0.0, -1.0);
        x *= 2.0;
        assert_eq!(x, Vec3::new(2.0, 0.0, -2.0));
    }

    #[test]
    fn test_mul_f64() {
        assert_eq!(Vec3::new(1.0, 0.0, -1.0) * 1.0, Vec3::new(1.0, 0.0, -1.0));
    }

    #[test]
    fn test_div() {
        assert_eq!(Vec3::new(1.0, -2.0, 0.0) / 2.0, Vec3::new(0.5, -1.0, 0.0));
    }

    #[test]
    fn test_elemul() {
        assert_eq!(
            Vec3::elemul(Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 2.0, 3.0)),
            Vec3::new(1.0, 4.0, 9.0)
        );
    }

    #[test]
    fn test_cross() {
        assert_eq!(
            Vec3::cross(Vec3::new(1.0, 2.0, 3.0), Vec3::new(2.0, 3.0, 4.0)),
            Vec3::new(8.0 - 9.0, 6.0 - 4.0, 3.0 - 4.0)
        );
    }

    #[test]
    fn test_neg() {
        assert_eq!(-Vec3::new(1.0, -2.0, 3.0), Vec3::new(-1.0, 2.0, -3.0));
    }
    */

    #[test]
    fn test_squared_length() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0).squared_length(), 14.0 as f64);
    }

    /*
    #[test]
    fn test_length() {
        assert_eq!(
            Vec3::new(3.0, 4.0, 5.0).length(),
            ((3.0 * 3.0 + 4.0 * 4.0 + 5.0 * 5.0) as f64).sqrt()
        );
    }

    #[test]
    fn test_unit() {
        assert_eq!(Vec3::new(233.0, 0.0, 0.0).unit(), Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(
            Vec3::new(-233.0, 0.0, 0.0).unit(),
            Vec3::new(-1.0, 0.0, 0.0)
        );
    }

    #[test]
    #[should_panic]
    fn test_unit_panic() {
        Vec3::new(0.0, 0.0, 0.0).unit();
    }
    */
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

lazy_static! {
    pub static ref EMPTY: Interval = Interval::new(f64::INFINITY, f64::NEG_INFINITY);
    pub static ref UNIVERSE: Interval = Interval::new(f64::NEG_INFINITY, f64::INFINITY);
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn combine(&self, other: &Interval) -> Interval {
        Interval::new(self.min.min(other.min), self.max.max(other.max))
    }

    pub fn is_empty(&self) -> bool {
        self.min > self.max
    }

    // pub fn empty() -> Self {
    //     Self::new(f64::INFINITY, f64::NEG_INFINITY)
    // }

    // fn universe() -> Self {
    //     Self::new(f64::NEG_INFINITY, f64::INFINITY)
    // }

    pub fn len(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, value: f64) -> bool {
        value >= self.min && value <= self.max
    }

    pub fn surrounds(&self, value: f64) -> bool {
        value > self.min && value < self.max
    }

    pub fn clamp(&self, value: f64) -> f64 {
        value.max(self.min).min(self.max)
    }

    pub fn expand(&self, delta: f64) -> Interval {
        Self::new(self.min - delta, self.max + delta)
    }

    pub fn intersect(&self, other: &Interval) -> Interval{
        Interval::new(self.min.max(other.min), self.max.min(other.max))
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
        }
    }
}

impl Add<f64> for Interval {
    type Output = Interval;
    fn add(self, rhs: f64) -> Self::Output {
        Interval{min: self.min + rhs, max: self.max + rhs}
    }
}

impl Add<Interval> for f64 {
    type Output = Interval;
    fn add(self, rhs: Interval) -> Self::Output {
        Interval{min: self + rhs.min, max: self + rhs.max}
    }
}

const PERLIN_POINT_COUNT: usize = 256;
pub struct Perlin {
    // Perlin noise generation parameters
    // randfloat: Vec<f64>,
    randvec: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new() -> Self {
        let mut randvec = Vec::new();
        randvec.resize(PERLIN_POINT_COUNT, Vec3::zero());
        for i in 0..PERLIN_POINT_COUNT {
            randvec[i] = (Vec3::random_xyz01() * 2.0 - Vec3::ones()).normalize();
        }

        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();
        Self {
            randvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: &Vec3) -> f64 {
        let (mut u, mut v, mut w) = (
            p.x - p.x.floor(),
            p.y - p.y.floor(),
            p.z - p.z.floor(),
        );
        // println!("[{u}, {v}, {w}]");

        let (i, j, k) = (
            p.x.floor() as i64,
            p.y.floor() as i64,
            p.z.floor() as i64,
        );

        let mut c = [[[Vec3::zero(); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.randvec[
                        self.perm_x[((di as i64 + i)&255) as usize] ^
                        self.perm_y[((dj as i64 + j)&255) as usize] ^
                        self.perm_z[((dk as i64 + k)&255) as usize]
                    ];
                }
            }
        }
        Self::perlin_interp(&c, u, v, w)
        // c[0][0][0]
    }

    pub fn turb(&self, p: Vec3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p = 2.0 * temp_p;
        }

        accum.abs()
    }

    fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let (uu, vv, ww) = (
            u*u*(3.0 - 2.0 * u),
            v*v*(3.0 - 2.0 * v),
            w*w*(3.0 - 2.0 * w),
        );
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += c[i][j][k].dot(&weight)
                    * (i as f64 * uu + (1-i) as f64 * (1.0-uu))
                    * (j as f64 * vv + (1-j) as f64 * (1.0-vv))
                    * (k as f64 * ww + (1-k) as f64 * (1.0-ww));
                }
            }
        }
        accum
    }

    fn perlin_generate_perm() -> Vec<usize> {
        let mut p : Vec<usize> = Vec::new();
        p.resize(PERLIN_POINT_COUNT, 0);
        for i in 0..PERLIN_POINT_COUNT {
            p[i] = i;
        }
        Self::permute(&mut p);
        p
    }

    fn permute(p: &mut Vec<usize>) {
        let n = p.len();
        let mut rng = thread_rng();
        for i in (1..n).rev() {
            let target = rng.gen_range(0..=i);
            p.swap(i, target);
        }
    }
}