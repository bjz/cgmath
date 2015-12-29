// Copyright 2013-2014 The CGMath Developers. For a full listing of the authors,
// refer to the Cargo.toml file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fmt;
use std::mem;
use std::ops::*;

use rand::{Rand, Rng};
use rust_num::{Float, One, Zero};
use rust_num::traits::cast;

use angle::{Angle, Rad};
use approx::ApproxEq;
use matrix::{Matrix3, Matrix4};
use num::BaseFloat;
use point::Point3;
use rotation::{Rotation, Rotation3, Basis3};
use vector::{Vector3, Vector, EuclideanVector};


/// A [quaternion](https://en.wikipedia.org/wiki/Quaternion) in scalar/vector
/// form.
#[derive(Copy, Clone, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Quaternion<S> {
    pub s: S,
    pub v: Vector3<S>,
}

impl<S: BaseFloat> Quaternion<S> {
    /// Construct a new quaternion from one scalar component and three
    /// imaginary components
    #[inline]
    pub fn new(w: S, xi: S, yj: S, zk: S) -> Quaternion<S> {
        Quaternion::from_sv(w, Vector3::new(xi, yj, zk))
    }

    /// Construct a new quaternion from a scalar and a vector
    #[inline]
    pub fn from_sv(s: S, v: Vector3<S>) -> Quaternion<S> {
        Quaternion { s: s, v: v }
    }

    /// The additive identity, ie: `q = 0 + 0i + 0j + 0i`
    #[inline]
    pub fn zero() -> Quaternion<S> {
        Quaternion::new(S::zero(), S::zero(), S::zero(), S::zero())
    }

    /// The multiplicative identity, ie: `q = 1 + 0i + 0j + 0i`
    #[inline]
    pub fn one() -> Quaternion<S> {
        Quaternion::from_sv(S::one(), Vector3::zero())
    }

    /// The dot product of the quaternion and `q`.
    #[inline]
    pub fn dot(self, other: Quaternion<S>) -> S {
        self.s * other.s + self.v.dot(other.v)
    }

    /// The conjugate of the quaternion.
    #[inline]
    pub fn conjugate(self) -> Quaternion<S> {
        Quaternion::from_sv(self.s, -self.v)
    }

    /// The squared magnitude of the quaternion. This is useful for
    /// magnitude comparisons where the exact magnitude does not need to be
    /// calculated.
    #[inline]
    pub fn magnitude2(self) -> S {
        self.s * self.s + self.v.length2()
    }

    /// The magnitude of the quaternion
    ///
    /// # Performance notes
    ///
    /// For instances where the exact magnitude of the quaternion does not need
    /// to be known, for example for quaternion-quaternion magnitude comparisons,
    /// it is advisable to use the `magnitude2` method instead.
    #[inline]
    pub fn magnitude(self) -> S {
        self.magnitude2().sqrt()
    }

    /// Normalize this quaternion, returning the new quaternion.
    #[inline]
    pub fn normalize(self) -> Quaternion<S> {
        self * (S::one() / self.magnitude())
    }

    /// Do a normalized linear interpolation with `other`, by `amount`.
    pub fn nlerp(self, other: Quaternion<S>, amount: S) -> Quaternion<S> {
        (self * (S::one() - amount) + other * amount).normalize()
    }
}

impl_operator!(<S: BaseFloat> Neg for Quaternion<S> {
    fn neg(quat) -> Quaternion<S> {
        Quaternion::from_sv(-quat.s, -quat.v)
    }
});

impl_operator!(<S: BaseFloat> Mul<S> for Quaternion<S> {
    fn mul(lhs, rhs) -> Quaternion<S> {
        Quaternion::from_sv(lhs.s * rhs, lhs.v * rhs)
    }
});
impl_assignment_operator!(<S: BaseFloat> MulAssign<S> for Quaternion<S> {
    fn mul_assign(&mut self, scalar) { self.s *= scalar; self.v *= scalar; }
});

impl_operator!(<S: BaseFloat> Div<S> for Quaternion<S> {
    fn div(lhs, rhs) -> Quaternion<S> {
        Quaternion::from_sv(lhs.s / rhs, lhs.v / rhs)
    }
});
impl_assignment_operator!(<S: BaseFloat> DivAssign<S> for Quaternion<S> {
    fn div_assign(&mut self, scalar) { self.s /= scalar; self.v /= scalar; }
});

impl_operator!(<S: BaseFloat> Mul<Vector3<S> > for Quaternion<S> {
    fn mul(lhs, rhs) -> Vector3<S> {{
        let rhs = rhs.clone();
        let two: S = cast(2i8).unwrap();
        let tmp = lhs.v.cross(rhs) + (rhs * lhs.s);
        (lhs.v.cross(tmp) * two) + rhs
    }}
});

impl_operator!(<S: BaseFloat> Add<Quaternion<S> > for Quaternion<S> {
    fn add(lhs, rhs) -> Quaternion<S> {
        Quaternion::from_sv(lhs.s + rhs.s, lhs.v + rhs.v)
    }
});
impl_assignment_operator!(<S: BaseFloat> AddAssign<Quaternion<S> > for Quaternion<S> {
    fn add_assign(&mut self, other) { self.s += other.s; self.v += other.v; }
});

impl_operator!(<S: BaseFloat> Sub<Quaternion<S> > for Quaternion<S> {
    fn sub(lhs, rhs) -> Quaternion<S> {
        Quaternion::from_sv(lhs.s - rhs.s, lhs.v - rhs.v)
    }
});
impl_assignment_operator!(<S: BaseFloat> SubAssign<Quaternion<S> > for Quaternion<S> {
    fn sub_assign(&mut self, other) { self.s -= other.s; self.v -= other.v; }
});

impl_operator!(<S: BaseFloat> Mul<Quaternion<S> > for Quaternion<S> {
    fn mul(lhs, rhs) -> Quaternion<S> {
        Quaternion::new(lhs.s * rhs.s - lhs.v.x * rhs.v.x - lhs.v.y * rhs.v.y - lhs.v.z * rhs.v.z,
                        lhs.s * rhs.v.x + lhs.v.x * rhs.s + lhs.v.y * rhs.v.z - lhs.v.z * rhs.v.y,
                        lhs.s * rhs.v.y + lhs.v.y * rhs.s + lhs.v.z * rhs.v.x - lhs.v.x * rhs.v.z,
                        lhs.s * rhs.v.z + lhs.v.z * rhs.s + lhs.v.x * rhs.v.y - lhs.v.y * rhs.v.x)
    }
});

impl<S: BaseFloat> ApproxEq for Quaternion<S> {
    type Epsilon = S;

    #[inline]
    fn approx_eq_eps(&self, other: &Quaternion<S>, epsilon: &S) -> bool {
        self.s.approx_eq_eps(&other.s, epsilon) &&
        self.v.approx_eq_eps(&other.v, epsilon)
    }
}

impl<S: BaseFloat> Quaternion<S> {
    /// Spherical Linear Intoperlation
    ///
    /// Return the spherical linear interpolation between the quaternion and
    /// `other`. Both quaternions should be normalized first.
    ///
    /// # Performance notes
    ///
    /// The `acos` operation used in `slerp` is an expensive operation, so
    /// unless your quarternions are far away from each other it's generally
    /// more advisable to use `nlerp` when you know your rotations are going
    /// to be small.
    ///
    /// - [Understanding Slerp, Then Not Using It]
    ///   (http://number-none.com/product/Understanding%20Slerp,%20Then%20Not%20Using%20It/)
    /// - [Arcsynthesis OpenGL tutorial]
    ///   (http://www.arcsynthesis.org/gltut/Positioning/Tut08%20Interpolation.html)
    pub fn slerp(self, other: Quaternion<S>, amount: S) -> Quaternion<S> {
        let dot = self.dot(other);
        let dot_threshold = cast(0.9995f64).unwrap();

        // if quaternions are close together use `nlerp`
        if dot > dot_threshold {
            self.nlerp(other, amount)
        } else {
            // stay within the domain of acos()
            // TODO REMOVE WHEN https://github.com/mozilla/rust/issues/12068 IS RESOLVED
            let robust_dot = if dot > S::one() {
                S::one()
            } else if dot < -S::one() {
                -S::one()
            } else {
                dot
            };

            let theta = Rad::acos(robust_dot.clone());

            let scale1 = Rad::sin(theta * (S::one() - amount));
            let scale2 = Rad::sin(theta * amount);

            (self * scale1 + other * scale2) * Rad::sin(theta).recip()
        }
    }

    /// Convert a Quaternion to Eular angles
    ///     This is a polar singularity aware conversion
    ///
    ///  Based on:
    /// - [Maths - Conversion Quaternion to Euler]
    ///   (http://www.euclideanspace.com/maths/geometry/rotations/conversions/quaternionToEuler/)
    pub fn to_euler(self) -> (Rad<S>, Rad<S>, Rad<S>) {
        let sig: S = cast(0.499f64).unwrap();
        let two: S = cast(2f64).unwrap();
        let one: S = cast(1f64).unwrap();

        let (qw, qx, qy, qz) = (self.s, self.v.x, self.v.y, self.v.z);
        let (sqw, sqx, sqy, sqz) = (qw * qw, qx * qx, qy * qy, qz * qz);

        let unit = sqx + sqy + sqz + sqw;
        let test = qx * qy + qz * qw;

        if test > sig * unit {
            (
                Rad::turn_div_4(),
                Rad::zero(),
                Rad::atan2(qx, qw) * two,
            )
        } else if test < -sig * unit {
            (
                -Rad::turn_div_4(),
                Rad::zero(),
                Rad::atan2(qx, qw) * two,
            )
        } else {
            (
                Rad::asin(two * (qx * qy + qz * qw)),
                Rad::atan2(two * (qy * qw - qx * qz), one - two * (sqy + sqz)),
                Rad::atan2(two * (qx * qw - qy * qz), one - two * (sqx + sqz)),
            )
        }
    }
}

impl<S: BaseFloat> From<Quaternion<S>> for Matrix3<S> {
    /// Convert the quaternion to a 3 x 3 rotation matrix
    fn from(quat: Quaternion<S>) -> Matrix3<S> {
        let x2 = quat.v.x + quat.v.x;
        let y2 = quat.v.y + quat.v.y;
        let z2 = quat.v.z + quat.v.z;

        let xx2 = x2 * quat.v.x;
        let xy2 = x2 * quat.v.y;
        let xz2 = x2 * quat.v.z;

        let yy2 = y2 * quat.v.y;
        let yz2 = y2 * quat.v.z;
        let zz2 = z2 * quat.v.z;

        let sy2 = y2 * quat.s;
        let sz2 = z2 * quat.s;
        let sx2 = x2 * quat.s;

        Matrix3::new(S::one() - yy2 - zz2, xy2 + sz2, xz2 - sy2,
                     xy2 - sz2, S::one() - xx2 - zz2, yz2 + sx2,
                     xz2 + sy2, yz2 - sx2, S::one() - xx2 - yy2)
    }
}

impl<S: BaseFloat> From<Quaternion<S>> for Matrix4<S> {
    /// Convert the quaternion to a 4 x 4 rotation matrix
    fn from(quat: Quaternion<S>) -> Matrix4<S> {
        let x2 = quat.v.x + quat.v.x;
        let y2 = quat.v.y + quat.v.y;
        let z2 = quat.v.z + quat.v.z;

        let xx2 = x2 * quat.v.x;
        let xy2 = x2 * quat.v.y;
        let xz2 = x2 * quat.v.z;

        let yy2 = y2 * quat.v.y;
        let yz2 = y2 * quat.v.z;
        let zz2 = z2 * quat.v.z;

        let sy2 = y2 * quat.s;
        let sz2 = z2 * quat.s;
        let sx2 = x2 * quat.s;

        Matrix4::new(S::one() - yy2 - zz2, xy2 + sz2, xz2 - sy2, S::zero(),
                     xy2 - sz2, S::one() - xx2 - zz2, yz2 + sx2, S::zero(),
                     xz2 + sy2, yz2 - sx2, S::one() - xx2 - yy2, S::zero(),
                     S::zero(), S::zero(), S::zero(), S::one())
    }
}

impl<S: BaseFloat> fmt::Debug for Quaternion<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} + {:?}i + {:?}j + {:?}k",
                self.s,
                self.v.x,
                self.v.y,
                self.v.z)
    }
}

// Quaternion Rotation impls

impl<S: BaseFloat> From<Quaternion<S>> for Basis3<S> {
    #[inline]
    fn from(quat: Quaternion<S>) -> Basis3<S> { Basis3::from_quaternion(&quat) }
}

impl<S: BaseFloat> Rotation<Point3<S>> for Quaternion<S> {
    #[inline]
    fn one() -> Quaternion<S> { Quaternion::one() }

    #[inline]
    fn look_at(dir: Vector3<S>, up: Vector3<S>) -> Quaternion<S> {
        Matrix3::look_at(dir, up).into()
    }

    #[inline]
    fn between_vectors(a: Vector3<S>, b: Vector3<S>) -> Quaternion<S> {
        //http://stackoverflow.com/questions/1171849/
        //finding-quaternion-representing-the-rotation-from-one-vector-to-another
        Quaternion::from_sv(S::one() + a.dot(b), a.cross(b)).normalize()
    }

    #[inline]
    fn rotate_vector(&self, vec: Vector3<S>) -> Vector3<S> { self * vec }

    #[inline]
    fn concat(&self, other: &Quaternion<S>) -> Quaternion<S> { self * other }

    #[inline]
    fn concat_self(&mut self, other: &Quaternion<S>) { *self = &*self * other; }

    #[inline]
    fn invert(&self) -> Quaternion<S> { self.conjugate() / self.magnitude2() }

    #[inline]
    fn invert_self(&mut self) { *self = self.invert() }
}

impl<S: BaseFloat> Rotation3<S> for Quaternion<S> {
    #[inline]
    fn from_axis_angle(axis: Vector3<S>, angle: Rad<S>) -> Quaternion<S> {
        let (s, c) = Rad::sin_cos(angle * cast(0.5f64).unwrap());
        let axis = Vector3::new(axis.z, axis.y, axis.x); // Fix ordering to match pitch, yaw, roll
        Quaternion::from_sv(c, axis * s)
    }

    /// Create a quaternion from a set of euler angles.
    ///
    /// # Parameters
    ///
    /// - `x`: the angular rotation around the `x` axis (pitch).
    /// - `y`: the angular rotation around the `y` axis (yaw).
    /// - `z`: the angular rotation around the `z` axis (roll).
    ///
    /// # References
    ///
    /// - [Maths - Conversion Euler to Quaternion]
    ///   (http://www.euclideanspace.com/maths/geometry/rotations/conversions/eulerToQuaternion/index.htm)
    fn from_euler(x: Rad<S>, y: Rad<S>, z: Rad<S>) -> Quaternion<S> {
        let (s_x, c_x) = Rad::sin_cos(x * cast(0.5f64).unwrap());
        let (s_y, c_y) = Rad::sin_cos(y * cast(0.5f64).unwrap());
        let (s_z, c_z) = Rad::sin_cos(z * cast(0.5f64).unwrap());

        Quaternion::new(c_y * c_x * c_z - s_y * s_x * s_z,
                        s_y * s_x * c_z + c_y * c_x * s_z,
                        s_y * c_x * c_z + c_y * s_x * s_z,
                        c_y * s_x * c_z - s_y * c_x * s_z)
    }
}

impl<S: BaseFloat> Into<[S; 4]> for Quaternion<S> {
    #[inline]
    fn into(self) -> [S; 4] {
        match self.into() { (w, xi, yj, zk) => [w, xi, yj, zk] }
    }
}

impl<S: BaseFloat> AsRef<[S; 4]> for Quaternion<S> {
    #[inline]
    fn as_ref(&self) -> &[S; 4] {
        unsafe { mem::transmute(self) }
    }
}

impl<S: BaseFloat> AsMut<[S; 4]> for Quaternion<S> {
    #[inline]
    fn as_mut(&mut self) -> &mut [S; 4] {
        unsafe { mem::transmute(self) }
    }
}

impl<S: BaseFloat> From<[S; 4]> for Quaternion<S> {
    #[inline]
    fn from(v: [S; 4]) -> Quaternion<S> {
        Quaternion::new(v[0], v[1], v[2], v[3])
    }
}

impl<'a, S: BaseFloat> From<&'a [S; 4]> for &'a Quaternion<S> {
    #[inline]
    fn from(v: &'a [S; 4]) -> &'a Quaternion<S> {
        unsafe { mem::transmute(v) }
    }
}

impl<'a, S: BaseFloat> From<&'a mut [S; 4]> for &'a mut Quaternion<S> {
    #[inline]
    fn from(v: &'a mut [S; 4]) -> &'a mut Quaternion<S> {
        unsafe { mem::transmute(v) }
    }
}

impl<S: BaseFloat> Into<(S, S, S, S)> for Quaternion<S> {
    #[inline]
    fn into(self) -> (S, S, S, S) {
        match self { Quaternion { s, v: Vector3 { x, y, z } } => (s, x, y, z) }
    }
}

impl<S: BaseFloat> AsRef<(S, S, S, S)> for Quaternion<S> {
    #[inline]
    fn as_ref(&self) -> &(S, S, S, S) {
        unsafe { mem::transmute(self) }
    }
}

impl<S: BaseFloat> AsMut<(S, S, S, S)> for Quaternion<S> {
    #[inline]
    fn as_mut(&mut self) -> &mut (S, S, S, S) {
        unsafe { mem::transmute(self) }
    }
}

impl<S: BaseFloat> From<(S, S, S, S)> for Quaternion<S> {
    #[inline]
    fn from(v: (S, S, S, S)) -> Quaternion<S> {
        match v { (w, xi, yj, zk) => Quaternion::new(w, xi, yj, zk) }
    }
}

impl<'a, S: BaseFloat> From<&'a (S, S, S, S)> for &'a Quaternion<S> {
    #[inline]
    fn from(v: &'a (S, S, S, S)) -> &'a Quaternion<S> {
        unsafe { mem::transmute(v) }
    }
}

impl<'a, S: BaseFloat> From<&'a mut (S, S, S, S)> for &'a mut Quaternion<S> {
    #[inline]
    fn from(v: &'a mut (S, S, S, S)) -> &'a mut Quaternion<S> {
        unsafe { mem::transmute(v) }
    }
}

macro_rules! index_operators {
    ($S:ident, $Output:ty, $I:ty) => {
        impl<$S: BaseFloat> Index<$I> for Quaternion<$S> {
            type Output = $Output;

            #[inline]
            fn index<'a>(&'a self, i: $I) -> &'a $Output {
                let v: &[$S; 4] = self.as_ref(); &v[i]
            }
        }

        impl<$S: BaseFloat> IndexMut<$I> for Quaternion<$S> {
            #[inline]
            fn index_mut<'a>(&'a mut self, i: $I) -> &'a mut $Output {
                let v: &mut [$S; 4] = self.as_mut(); &mut v[i]
            }
        }
    }
}

index_operators!(S, S, usize);
index_operators!(S, [S], Range<usize>);
index_operators!(S, [S], RangeTo<usize>);
index_operators!(S, [S], RangeFrom<usize>);
index_operators!(S, [S], RangeFull);

impl<S: BaseFloat + Rand> Rand for Quaternion<S> {
    #[inline]
    fn rand<R: Rng>(rng: &mut R) -> Quaternion<S> {
       Quaternion::from_sv(rng.gen(), rng.gen())
    }
}

#[cfg(test)]
mod tests {
    use quaternion::*;
    use vector::*;

    const QUATERNION: Quaternion<f32> = Quaternion {
        s: 1.0,
        v: Vector3 { x: 2.0, y: 3.0, z: 4.0 },
    };

    #[test]
    fn test_into() {
        let v = QUATERNION;
        {
            let v: [f32; 4] = v.into();
            assert_eq!(v, [1.0, 2.0, 3.0, 4.0]);
        }
        {
            let v: (f32, f32, f32, f32) = v.into();
            assert_eq!(v, (1.0, 2.0, 3.0, 4.0));
        }
    }

    #[test]
    fn test_as_ref() {
        let v = QUATERNION;
        {
            let v: &[f32; 4] = v.as_ref();
            assert_eq!(v, &[1.0, 2.0, 3.0, 4.0]);
        }
        {
            let v: &(f32, f32, f32, f32) = v.as_ref();
            assert_eq!(v, &(1.0, 2.0, 3.0, 4.0));
        }
    }

    #[test]
    fn test_as_mut() {
        let mut v = QUATERNION;
        {
            let v: &mut[f32; 4] = v.as_mut();
            assert_eq!(v, &mut [1.0, 2.0, 3.0, 4.0]);
        }
        {
            let v: &mut(f32, f32, f32, f32) = v.as_mut();
            assert_eq!(v, &mut (1.0, 2.0, 3.0, 4.0));
        }
    }

    #[test]
    fn test_from() {
        assert_eq!(Quaternion::from([1.0, 2.0, 3.0, 4.0]), QUATERNION);
        {
            let v = &[1.0, 2.0, 3.0, 4.0];
            let v: &Quaternion<_> = From::from(v);
            assert_eq!(v, &QUATERNION);
        }
        {
            let v = &mut [1.0, 2.0, 3.0, 4.0];
            let v: &mut Quaternion<_> = From::from(v);
            assert_eq!(v, &QUATERNION);
        }
        assert_eq!(Quaternion::from((1.0, 2.0, 3.0, 4.0)), QUATERNION);
        {
            let v = &(1.0, 2.0, 3.0, 4.0);
            let v: &Quaternion<_> = From::from(v);
            assert_eq!(v, &QUATERNION);
        }
        {
            let v = &mut (1.0, 2.0, 3.0, 4.0);
            let v: &mut Quaternion<_> = From::from(v);
            assert_eq!(v, &QUATERNION);
        }
    }
}
