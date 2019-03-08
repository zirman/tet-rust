use std::fmt::{Display, Debug};
use std::f32::consts;

pub trait Scalar: Copy + Display + Debug {
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
    const PI: Self;
    fn lt(Self, Self) -> bool;
    fn gt(Self, Self) -> bool;
    fn lte(Self, Self) -> bool;
    fn gte(Self, Self) -> bool;
    fn add(Self, Self) -> Self;
    fn sub(Self, Self) -> Self;
    fn mul(Self, Self) -> Self;
    fn div(Self, Self) -> Self;
    fn neg(Self) -> Self;
    fn sin(Self) -> Self;
    fn cos(Self) -> Self;
    fn tan(Self) -> Self;
    fn rads(Self) -> Self;
    fn new(f32) -> Self;
    fn f32(Self) -> f32;
    fn f64(Self) -> f64;
}

impl Scalar for i32 {
    const ZERO: i32 = 0;
    const ONE: i32 = 1000;
    const TWO: i32 = i32::ONE * 2;
    const PI: i32 = (consts::PI * (i32::ONE as f32)) as i32;
    fn lt(lhs: Self, rhs: Self) -> bool {
        lhs < rhs
    }
    fn gt(lhs: Self, rhs: Self) -> bool {
        lhs > rhs
    }
    fn lte(lhs: Self, rhs: Self) -> bool {
        lhs <= rhs
    }
    fn gte(lhs: Self, rhs: Self) -> bool {
        lhs >= rhs
    }
    fn add(lhs: Self, rhs: Self) -> Self {
        lhs + rhs
    }
    fn sub(lhs: Self, rhs: Self) -> Self {
        lhs - rhs
    }
    fn mul(lhs: Self, rhs: Self) -> Self {
        (lhs * rhs) / Self::ONE
    }
    fn div(n: Self, d: Self) -> Self {
        (n * Self::ONE) / d
    }
    fn neg(x: Self) -> Self {
        -x
    }
    fn sin(r: Self) -> Self {
        (f32::sin((r as f32) / (Self::ONE as f32)) * (Self::ONE as f32)) as Self
    }
    fn cos(r: Self) -> Self {
        (f32::cos((r as f32) / (Self::ONE as f32)) * (Self::ONE as f32)) as Self
    }
    fn tan(r: Self) -> Self {
        (f32::tan((r as f32) / (Self::ONE as f32)) * (Self::ONE as f32)) as Self
    }
    fn rads(r: Self) -> i32 {
        (f32::to_radians((r as f32) / (Self::ONE as f32)) * (Self::ONE as f32)) as Self
    }
    fn new(x: f32) -> Self {
        (x * (Self::ONE as f32)) as Self
    }
    fn f32(x: Self) -> f32 {
        x as f32
    }
    fn f64(x: Self) -> f64 {
        x as f64
    }
}

impl Scalar for f32 {
    const ZERO: f32 = 0.0;
    const ONE: f32 = 1.0;
    const TWO: f32 = 2.0;
    const PI: f32 = consts::PI;
    fn lt(lhs: Self, rhs: Self) -> bool {
        lhs < rhs
    }
    fn gt(lhs: Self, rhs: Self) -> bool {
        lhs > rhs
    }
    fn lte(lhs: Self, rhs: Self) -> bool {
        lhs <= rhs
    }
    fn gte(lhs: Self, rhs: Self) -> bool {
        lhs >= rhs
    }
    fn add(lhs: Self, rhs: Self) -> Self {
        lhs + rhs
    }
    fn sub(lhs: Self, rhs: Self) -> Self {
        lhs - rhs
    }
    fn mul(lhs: Self, rhs: Self) -> Self {
        lhs * rhs
    }
    fn div(n: Self, d: Self) -> Self {
        n / d
    }
    fn neg(x: Self) -> Self {
        -x
    }
    fn sin(r: Self) -> Self {
        Self::sin(r)
    }
    fn cos(r: Self) -> Self {
        Self::cos(r)
    }
    fn tan(r: Self) -> Self {
        Self::tan(r)
    }
    fn rads(r: Self) -> Self {
        Self::to_radians(r)
    }
    fn new(x: Self) -> Self {
        x
    }
    fn f32(x: Self) -> Self {
        x
    }
    fn f64(x: Self) -> f64 {
        x as f64
    }
}
