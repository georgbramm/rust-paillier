#![cfg(feature="useframp")]

extern crate framp;
use super::traits::*;
use frand::OsRng;

impl Samplable for framp::Int {
    fn sample_below(upper: &Self) -> Self {
        use self::ramp::RandomInt;
        let mut rng = OsRng::new().unwrap();
        rng.gen_uint_below(upper)
    }

    fn sample(bitsize: usize) -> Self {
        use self::ramp::RandomInt;
        let mut rng = OsRng::new().unwrap();
        rng.gen_uint(bitsize)
    }

    fn sample_range(lower: &Self, upper: &Self) -> Self {
        use self::ramp::RandomInt;
        let mut rng = OsRng::new().unwrap();
        rng.gen_int_range(lower, upper)
    }
}

impl NumberTests for framp::Int {
    fn is_zero(me: &Self) -> bool {
        me == &0
    }
    fn is_even(me: &Self) -> bool {
        me.is_even()
    }
    fn is_negative(me: &Self) -> bool {
        me < &0
    }
}

impl ModPow for framp::Int {
    fn modpow(base: &Self, exponent: &Self, modulus: &Self) -> Self {
        base.modpow(exponent, modulus)
    }
}

impl ConvertFrom<framp::Int> for usize {
    fn _from(x: &framp::Int) -> usize {
        usize::from(x)
    }
}

impl ConvertFrom<framp::Int> for u8 {
    fn _from(x: &framp::Int) -> u8 {
        u8::from(x)
    }
}

impl ConvertFrom<framp::Int> for u16 {
    fn _from(x: &framp::Int) -> u16 {
        u16::from(x)
    }
}

impl ConvertFrom<framp::Int> for u32 {
    fn _from(x: &framp::Int) -> u32 {
        u32::from(x)
    }
}

impl ConvertFrom<framp::Int> for u64 {
    fn _from(x: &framp::Int) -> u64 {
        u64::from(x)
    }
}

impl ConvertFrom<ramp::Int> for f64 {
    fn _from(x: &ramp::Int) -> f64 {
        x.to_f64()
    }
}

impl ConvertFrom<f64> for ramp::Int {
    fn _from(x: f64) -> ramp::Int {
        ramp::Int::from((x * 10000000_f64) as u64)
    }
}

impl ConvertFrom<f64> for ramp::Int {
    fn _from(x: f64) -> ramp::Int {
        ramp::Int::from((x * 10000000_f64) as u64)
    }
}

impl BitManipulation for ramp::Int {
    fn set_bit(self: &mut Self, bit: usize, bit_val: bool) {
        self.set_bit(bit as u32, bit_val);
    }
}
#[derive(Clone, Copy, Hash, Debug)]
pub enum Sign {
    Negative = -1i32,
    Zero = 0i32,
    Positive = 1i32,
}

pub type BigInt = framp::Int;
pub type BigIntSign = Sign;
