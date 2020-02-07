use crate::{Bls12Scalar, Bls12Base};
use std::ops::{Add, Mul};


// Parameters taken from the implementation of Bls12-377 in Zexe found here:
// https://github.com/scipr-lab/zexe/blob/master/algebra/src/curves/bls12_377/g1.rs

const A: Bls12Base = Bls12Base { limbs: [0x0, 0x0, 0x0, 0x0, 0x0, 0x0] };

const B: Bls12Base = Bls12Base {
    limbs: [
        0x2cdffffffffff68,
        0x51409f837fffffb1,
        0x9f7db3a98a7d3ff2,
        0x7b4e97b76e7c6305,
        0x4cf495bf803c84e8,
        0x8d6661e2fdf49a,
    ]
};

const COFACTOR: &'static [u64] = &[0x0, 0x170b5d4430000000];

const COFACTOR_INV: Bls12Scalar = Bls12Scalar {
    limbs: [
        2013239619100046060,
        4201184776506987597,
        2526766393982337036,
        1114629510922847535,
    ]
};

pub const G1_GENERATOR_X: Bls12Base = Bls12Base {
    limbs: [
        0x260f33b9772451f4,
        0xc54dd773169d5658,
        0x5c1551c469a510dd,
        0x761662e4425e1698,
        0xc97d78cc6f065272,
        0xa41206b361fd4d,
    ]
};

pub const G1_GENERATOR_Y: Bls12Base = Bls12Base {
    limbs: [
        0x8193961fb8cb81f3,
        0x638d4c5f44adb8,
        0xfafaf3dad4daf54a,
        0xc27849e2d655cd18,
        0x2ec3ddb401d52814,
        0x7da93326303c71,
    ]
};

#[derive(Eq, PartialEq)]
pub struct G1ProjectivePoint {
    pub x: Bls12Base,
    pub y: Bls12Base,
    pub z: Bls12Base,
}

impl Add<G1ProjectivePoint> for G1ProjectivePoint {
    type Output = G1ProjectivePoint;

    /// Safe version of addition with non-zero checks
    /// From https://www.hyperelliptic.org/EFD/g1p/data/shortw/projective/addition/add-1998-cmo-2
    fn add(self, rhs: G1ProjectivePoint) -> Self::Output {
        if self.is_zero() {
            rhs
        }else if rhs.is_zero() {
            self
        }else if self.x == -rhs.x {
            //TODO: return the zero element
            self
        }else {
            let y1z2 = self.y * rhs.z;
            let x1z2 = self.x * rhs.z;
            let z1z2 = self.z * rhs.z;
            let u = rhs.y * self.z - y1z2;
            let uu = u * u;
            let v = rhs.x * self.z - x1z2;
            let vv = v * v;
            let vvv = v * vv;
            let r = vv * x1z2;
            let a = uu * z1z2 - vvv - r * 2u64;
            let x3 = v * a;
            let y3 = u * (r - a) - vvv * y1z2;
            let z3 = vvv * z1z2;
            G1ProjectivePoint{x: x3, y: y3, z: z3}
        }
    }
}

impl Mul<G1ProjectivePoint> for Bls12Scalar {
    type Output = G1ProjectivePoint;

    fn mul(self, rhs: G1ProjectivePoint) -> Self::Output {

        let mut g = rhs;
        let mut sum = G1ProjectivePoint::ZERO;
        for limb in self.limbs.iter() {
            for j in 0..64 {
                if (limb >> j & 1u64) != 0u64 {
                    sum = sum + g;
                }
                g = g.double();
            }
        }
        sum
    }
}

impl G1ProjectivePoint {
    pub fn is_zero(&self) -> bool {
        self.z == Bls12Base::ZERO
    }

    const ZERO : G1ProjectivePoint = G1ProjectivePoint{x: Bls12Base::ZERO, y: Bls12Base::ZERO, z: Bls12Base::ZERO};

    /// Doubling of a G1 point
    /// From https://www.hyperelliptic.org/EFD/g1p/data/shortw/projective/doubling/dbl-2007-bl
    pub fn double(&self) -> G1ProjectivePoint {
        let w = self.x * self.x * 3u64;
        let s = self.y * self.z;
        let ss = s * s;
        let sss = s * ss;
        let r = self.y * s;
        let b = self.x * r;
        let h = w * w - b * 8u64;
        let x3 = h * s * 2u64;
        let y3 = w * (b * 4u64 - h) - r * r * 8u64;
        let z3 = sss * 8u64;
        G1ProjectivePoint{x: x3, y: y3, z: z3}
    }

    pub fn new(x: Bls12Base, y: Bls12Base, z: Bls12Base) -> G1ProjectivePoint {
        assert!(G1ProjectivePoint::is_on_curve(x, y, z) /*&& is_in_subgroup(x, y, z)*/);
        G1ProjectivePoint{x: x, y: y, z: z}
    }

    fn is_on_curve(x: Bls12Base, y: Bls12Base, z: Bls12Base) -> bool {
        if z == Bls12Base::ZERO {
            true
        } else {
            let y = y / z;
            let x = x / z;
            y * y == x * x * x + B
        }
    }

    /*
    fn is_in_subgroup(x: Bls12Base, y: Bls12Base, z: Bls12Base) -> bool {

    }
    */
}