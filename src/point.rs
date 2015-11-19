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

//! Points are fixed positions in affine space with no length or direction. This
//! disinguishes them from vectors, which have a length and direction, but do
//! not have a fixed position.

use std::fmt;
use std::mem;
use std::ops::*;

use rust_num::{One, Zero};

use approx::ApproxEq;
use array::Array;
use matrix::Matrix;
use num::{BaseNum, BaseFloat};
use vector::*;

/// A point in 2-dimensional space.
#[derive(PartialEq, Eq, Copy, Clone, Hash, RustcEncodable, RustcDecodable)]
pub struct Point2<S> { pub x: S, pub y: S }

/// A point in 3-dimensional space.
#[derive(PartialEq, Eq, Copy, Clone, Hash, RustcEncodable, RustcDecodable)]
pub struct Point3<S> { pub x: S, pub y: S, pub z: S }


impl<S: BaseNum> Point2<S> {
    #[inline]
    pub fn new(x: S, y: S) -> Point2<S> {
        Point2 { x: x, y: y }
    }
}

impl<S: BaseNum> Point3<S> {
    #[inline]
    pub fn new(x: S, y: S, z: S) -> Point3<S> {
        Point3 { x: x, y: y, z: z }
    }
}

impl<S: BaseNum> Point3<S> {
    #[inline]
    pub fn from_homogeneous(v: Vector4<S>) -> Point3<S> {
        let e = v.truncate() * (S::one() / v.w);
        Point3::new(e.x, e.y, e.z)  //FIXME
    }

    #[inline]
    pub fn to_homogeneous(self) -> Vector4<S> {
        Vector4::new(self.x, self.y, self.z, S::one())
    }
}

/// Specifies the numeric operations for point types.
pub trait Point: Copy + Clone where
    // FIXME: Ugly type signatures - blocked by rust-lang/rust#24092
    Self: Array<Element = <Self as Point>::Scalar>,
    // FIXME: blocked by rust-lang/rust#20671
    //
    // for<'a, 'b> &'a Self: Add<&'b V, Output = Self>,
    // for<'a, 'b> &'a Self: Sub<&'b Self, Output = V>,
    //
    // for<'a> &'a Self: Mul<S, Output = Self>,
    // for<'a> &'a Self: Div<S, Output = Self>,
    // for<'a> &'a Self: Rem<S, Output = Self>,
{
    /// The associated scalar.
    ///
    /// Due to the equality constraints demanded by `Self::Vector`, this is effectively just an
    /// alias to `Self::Vector::Scalar`.
    type Scalar: BaseNum;
    /// The associated displacement vector.
    type Vector: Vector<Scalar = Self::Scalar>;

    /// Create a point at the origin.
    fn origin() -> Self;

    /// Create a point from a vector.
    fn from_vec(v: Self::Vector) -> Self;
    /// Convert a point to a vector.
    fn to_vec(self) -> Self::Vector;

    /// Multiply each component by a scalar, returning the new point.
    #[must_use]
    fn mul_s(self, scalar: Self::Scalar) -> Self;
    /// Divide each component by a scalar, returning the new point.
    #[must_use]
    fn div_s(self, scalar: Self::Scalar) -> Self;
    /// Subtract a scalar from each component, returning the new point.
    #[must_use]
    fn rem_s(self, scalar: Self::Scalar) -> Self;

    /// Add a vector to this point, returning the new point.
    #[must_use]
    fn add_v(self, v: Self::Vector) -> Self;
    /// Subtract another point from this one, returning a new vector.
    fn sub_p(self, p: Self) -> Self::Vector;

    /// Multiply each component by a scalar, in-place.
    fn mul_self_s(&mut self, scalar: Self::Scalar);
    /// Divide each component by a scalar, in-place.
    fn div_self_s(&mut self, scalar: Self::Scalar);
    /// Take the remainder of each component by a scalar, in-place.
    fn rem_self_s(&mut self, scalar: Self::Scalar);

    /// Add a vector to this point, in-place.
    fn add_self_v(&mut self, v: Self::Vector);

    /// This is a weird one, but its useful for plane calculations.
    fn dot(self, v: Self::Vector) -> Self::Scalar;

    #[must_use]
    fn min(self, p: Self) -> Self;

    #[must_use]
    fn max(self, p: Self) -> Self;
}

impl<S: BaseNum> Array for Point2<S> {
    type Element = S;

    fn sum(self) -> S {
        self.x + self.y
    }

    fn product(self) -> S {
        self.x * self.y
    }

    fn min(self) -> S {
        self.x.partial_min(self.y)
    }

    fn max(self) -> S {
        self.x.partial_max(self.y)
    }
}

impl<S: BaseNum> Point for Point2<S> {
    type Scalar = S;
    type Vector = Vector2<S>;

    #[inline]
    fn origin() -> Point2<S> {
        Point2::new(S::zero(), S::zero())
    }

    #[inline]
    fn from_vec(v: Vector2<S>) -> Point2<S> {
        Point2::new(v.x, v.y)
    }

    #[inline]
    fn to_vec(self) -> Vector2<S> {
        Vector2::new(self.x, self.y)
    }

    #[inline] fn mul_s(self, scalar: S) -> Point2<S> { self * scalar }
    #[inline] fn div_s(self, scalar: S) -> Point2<S> { self / scalar }
    #[inline] fn rem_s(self, scalar: S) -> Point2<S> { self % scalar }
    #[inline] fn add_v(self, v: Vector2<S>) -> Point2<S> { self + v }
    #[inline] fn sub_p(self, p: Point2<S>) -> Vector2<S> { self - p }

    #[inline]
    fn mul_self_s(&mut self, scalar: S) {
        self.x = self.x * scalar;
        self.y = self.y * scalar;
    }

    #[inline]
    fn div_self_s(&mut self, scalar: S) {
        self.x = self.x / scalar;
        self.y = self.y / scalar;
    }

    #[inline]
    fn rem_self_s(&mut self, scalar: S) {
        self.x = self.x % scalar;
        self.y = self.y % scalar;
    }

    #[inline]
    fn add_self_v(&mut self, v: Vector2<S>) {
        self.x = self.x + v.x;
        self.y = self.y + v.y;
    }

    #[inline]
    fn dot(self, v: Vector2<S>) -> S {
        self.x * v.x +
        self.y * v.y
    }

    #[inline]
    fn min(self, p: Point2<S>) -> Point2<S> {
        Point2::new(self.x.partial_min(p.x), self.y.partial_min(p.y))
    }

    #[inline]
    fn max(self, p: Point2<S>) -> Point2<S> {
        Point2::new(self.x.partial_max(p.x), self.y.partial_max(p.y))
    }
}

impl<S: BaseNum> Array for Point3<S> {
    type Element = S;

    fn sum(self) -> S {
        self.x + self.y + self.z
    }

    fn product(self) -> S {
        self.x * self.y * self.z
    }

    fn min(self) -> S {
        self.x.partial_min(self.y).partial_min(self.z)
    }

    fn max(self) -> S {
        self.x.partial_max(self.y).partial_max(self.z)
    }
}

impl<S: BaseNum> Point for Point3<S> {
    type Scalar = S;
    type Vector = Vector3<S>;

    #[inline]
    fn origin() -> Point3<S> {
        Point3::new(S::zero(), S::zero(), S::zero())
    }

    #[inline]
    fn from_vec(v: Vector3<S>) -> Point3<S> {
        Point3::new(v.x, v.y, v.z)
    }

    #[inline]
    fn to_vec(self) -> Vector3<S> {
        Vector3::new(self.x, self.y, self.z)
    }

    #[inline] fn mul_s(self, scalar: S) -> Point3<S> { self * scalar }
    #[inline] fn div_s(self, scalar: S) -> Point3<S> { self / scalar }
    #[inline] fn rem_s(self, scalar: S) -> Point3<S> { self % scalar }
    #[inline] fn add_v(self, v: Vector3<S>) -> Point3<S> { self + v }
    #[inline] fn sub_p(self, p: Point3<S>) -> Vector3<S> { self - p }

    #[inline]
    fn mul_self_s(&mut self, scalar: S) {
        self.x = self.x * scalar;
        self.y = self.y * scalar;
        self.z = self.z * scalar;
    }

    #[inline]
    fn div_self_s(&mut self, scalar: S) {
        self.x = self.x / scalar;
        self.y = self.y / scalar;
        self.z = self.z / scalar;
    }

    #[inline]
    fn rem_self_s(&mut self, scalar: S) {
        self.x = self.x % scalar;
        self.y = self.y % scalar;
        self.z = self.z % scalar;
    }

    #[inline]
    fn add_self_v(&mut self, v: Vector3<S>) {
        self.x = self.x + v.x;
        self.y = self.y + v.y;
        self.z = self.z + v.z;
    }

    #[inline]
    fn dot(self, v: Vector3<S>) -> S {
        self.x * v.x +
        self.y * v.y +
        self.z * v.z
    }

    #[inline]
    fn min(self, p: Point3<S>) -> Point3<S> {
        Point3::new(self.x.partial_min(p.x), self.y.partial_min(p.y), self.z.partial_min(p.z))
    }

    #[inline]
    fn max(self, p: Point3<S>) -> Point3<S> {
        Point3::new(self.x.partial_max(p.x), self.y.partial_max(p.y), self.z.partial_max(p.z))
    }
}


macro_rules! impl_approx_eq {
    ($PointN:ident { $($field:ident),+ }) => {
        impl<S: BaseFloat> ApproxEq for $PointN<S> {
            type Epsilon = S;

            #[inline]
            fn default_epsilon() -> S { S::default_epsilon() }

            #[inline]
            fn default_max_relative() -> S { S::default_max_relative() }

            #[inline]
            fn default_max_ulps() -> u32 { S::default_max_ulps() }

            #[inline]
            fn relative_eq(&self, other: &$PointN<S>, epsilon: S, max_relative: S) -> bool {
                $(S::relative_eq(&self.$field, &other.$field, epsilon, max_relative))&&+
            }

            #[inline]
            fn ulps_eq(&self, other: &$PointN<S>, epsilon: S, max_ulps: u32) -> bool {
                $(S::ulps_eq(&self.$field, &other.$field, epsilon, max_ulps))&&+
            }
        }
    }
}

impl_approx_eq!(Point2 { x, y });
impl_approx_eq!(Point3 { x, y, z });

macro_rules! impl_operators {
    ($PointN:ident { $($field:ident),+ }, $VectorN:ident) => {
        impl<S: BaseNum> Mul<S> for $PointN<S> {
            type Output = $PointN<S>;

            #[inline]
            fn mul(self, scalar: S) -> $PointN<S> {
                $PointN::new($(self.$field * scalar),+)
            }
        }

        impl<S: BaseNum> Div<S> for $PointN<S> {
            type Output = $PointN<S>;

            #[inline]
            fn div(self, scalar: S) -> $PointN<S> {
                $PointN::new($(self.$field / scalar),+)
            }
        }

        impl<S: BaseNum> Rem<S> for $PointN<S> {
            type Output = $PointN<S>;

            #[inline]
            fn rem(self, scalar: S) -> $PointN<S> {
                $PointN::new($(self.$field % scalar),+)
            }
        }

        impl<'a, S: BaseNum> Mul<S> for &'a $PointN<S> {
            type Output = $PointN<S>;

            #[inline]
            fn mul(self, scalar: S) -> $PointN<S> {
                $PointN::new($(self.$field * scalar),+)
            }
        }

        impl<'a, S: BaseNum> Div<S> for &'a $PointN<S> {
            type Output = $PointN<S>;

            #[inline]
            fn div(self, scalar: S) -> $PointN<S> {
                $PointN::new($(self.$field / scalar),+)
            }
        }

        impl<'a, S: BaseNum> Rem<S> for &'a $PointN<S> {
            type Output = $PointN<S>;

            #[inline]
            fn rem(self, scalar: S) -> $PointN<S> {
                $PointN::new($(self.$field % scalar),+)
            }
        }

        impl<S: BaseNum> Add<$VectorN<S>> for $PointN<S> {
            type Output = $PointN<S>;

            #[inline]
            fn add(self, v: $VectorN<S>) -> $PointN<S> {
                $PointN::new($(self.$field + v.$field),+)
            }
        }

        impl<S: BaseNum> Sub<$PointN<S>> for $PointN<S> {
            type Output = $VectorN<S>;

            #[inline]
            fn sub(self, p: $PointN<S>) -> $VectorN<S> {
                $VectorN::new($(self.$field - p.$field),+)
            }
        }

        impl<'a, S: BaseNum> Add<&'a $VectorN<S>> for $PointN<S> {
            type Output = $PointN<S>;

            #[inline]
            fn add(self, v: &'a $VectorN<S>) -> $PointN<S> {
                $PointN::new($(self.$field + v.$field),+)
            }
        }

        impl<'a, S: BaseNum> Sub<&'a $PointN<S>> for $PointN<S> {
            type Output = $VectorN<S>;

            #[inline]
            fn sub(self, p: &'a $PointN<S>) -> $VectorN<S> {
                $VectorN::new($(self.$field - p.$field),+)
            }
        }

        impl<'a, S: BaseNum> Add<$VectorN<S>> for &'a $PointN<S> {
            type Output = $PointN<S>;

            #[inline]
            fn add(self, v: $VectorN<S>) -> $PointN<S> {
                $PointN::new($(self.$field + v.$field),+)
            }
        }

        impl<'a, S: BaseNum> Sub<$PointN<S>> for &'a $PointN<S> {
            type Output = $VectorN<S>;

            #[inline]
            fn sub(self, p: $PointN<S>) -> $VectorN<S> {
                $VectorN::new($(self.$field - p.$field),+)
            }
        }

        impl<'a, 'b, S: BaseNum> Add<&'a $VectorN<S>> for &'b $PointN<S> {
            type Output = $PointN<S>;

            #[inline]
            fn add(self, v: &'a $VectorN<S>) -> $PointN<S> {
                $PointN::new($(self.$field + v.$field),+)
            }
        }

        impl<'a, 'b, S: BaseNum> Sub<&'a $PointN<S>> for &'b $PointN<S> {
            type Output = $VectorN<S>;

            #[inline]
            fn sub(self, p: &'a $PointN<S>) -> $VectorN<S> {
                $VectorN::new($(self.$field - p.$field),+)
            }
        }
    }
}

impl_operators!(Point2 { x, y }, Vector2);
impl_operators!(Point3 { x, y, z }, Vector3);

macro_rules! fixed_array_conversions {
    ($PointN:ident <$S:ident> { $($field:ident : $index:expr),+ }, $n:expr) => {
        impl<$S> Into<[$S; $n]> for $PointN<$S> {
            #[inline]
            fn into(self) -> [$S; $n] {
                match self { $PointN { $($field),+ } => [$($field),+] }
            }
        }

        impl<$S> AsRef<[$S; $n]> for $PointN<$S> {
            #[inline]
            fn as_ref(&self) -> &[$S; $n] {
                unsafe { mem::transmute(self) }
            }
        }

        impl<$S> AsMut<[$S; $n]> for $PointN<$S> {
            #[inline]
            fn as_mut(&mut self) -> &mut [$S; $n] {
                unsafe { mem::transmute(self) }
            }
        }

        impl<$S: Clone> From<[$S; $n]> for $PointN<$S> {
            #[inline]
            fn from(v: [$S; $n]) -> $PointN<$S> {
                // We need to use a clone here because we can't pattern match on arrays yet
                $PointN { $($field: v[$index].clone()),+ }
            }
        }

        impl<'a, $S> From<&'a [$S; $n]> for &'a $PointN<$S> {
            #[inline]
            fn from(v: &'a [$S; $n]) -> &'a $PointN<$S> {
                unsafe { mem::transmute(v) }
            }
        }

        impl<'a, $S> From<&'a mut [$S; $n]> for &'a mut $PointN<$S> {
            #[inline]
            fn from(v: &'a mut [$S; $n]) -> &'a mut $PointN<$S> {
                unsafe { mem::transmute(v) }
            }
        }
    }
}

fixed_array_conversions!(Point2<S> { x:0, y:1 }, 2);
fixed_array_conversions!(Point3<S> { x:0, y:1, z:2 }, 3);

macro_rules! tuple_conversions {
    ($PointN:ident <$S:ident> { $($field:ident),+ }, $Tuple:ty) => {
        impl<$S> Into<$Tuple> for $PointN<$S> {
            #[inline]
            fn into(self) -> $Tuple {
                match self { $PointN { $($field),+ } => ($($field),+) }
            }
        }

        impl<$S> AsRef<$Tuple> for $PointN<$S> {
            #[inline]
            fn as_ref(&self) -> &$Tuple {
                unsafe { mem::transmute(self) }
            }
        }

        impl<$S> AsMut<$Tuple> for $PointN<$S> {
            #[inline]
            fn as_mut(&mut self) -> &mut $Tuple {
                unsafe { mem::transmute(self) }
            }
        }

        impl<$S> From<$Tuple> for $PointN<$S> {
            #[inline]
            fn from(v: $Tuple) -> $PointN<$S> {
                // We need to use a clone here because we can't pattern match on arrays yet
                match v { ($($field),+) => $PointN { $($field: $field),+ } }
            }
        }

        impl<'a, $S> From<&'a $Tuple> for &'a $PointN<$S> {
            #[inline]
            fn from(v: &'a $Tuple) -> &'a $PointN<$S> {
                unsafe { mem::transmute(v) }
            }
        }

        impl<'a, $S> From<&'a mut $Tuple> for &'a mut $PointN<$S> {
            #[inline]
            fn from(v: &'a mut $Tuple) -> &'a mut $PointN<$S> {
                unsafe { mem::transmute(v) }
            }
        }
    }
}

tuple_conversions!(Point2<S> { x, y }, (S, S));
tuple_conversions!(Point3<S> { x, y, z }, (S, S, S));

macro_rules! index_operators {
    ($PointN:ident<$S:ident>, $n:expr, $Output:ty, $I:ty) => {
        impl<$S> Index<$I> for $PointN<$S> {
            type Output = $Output;

            #[inline]
            fn index<'a>(&'a self, i: $I) -> &'a $Output {
                let v: &[$S; $n] = self.as_ref(); &v[i]
            }
        }

        impl<$S> IndexMut<$I> for $PointN<$S> {
            #[inline]
            fn index_mut<'a>(&'a mut self, i: $I) -> &'a mut $Output {
                let v: &mut [$S; $n] = self.as_mut(); &mut v[i]
            }
        }
    }
}

index_operators!(Point2<S>, 2, S, usize);
index_operators!(Point3<S>, 3, S, usize);
index_operators!(Point2<S>, 2, [S], Range<usize>);
index_operators!(Point3<S>, 3, [S], Range<usize>);
index_operators!(Point2<S>, 2, [S], RangeTo<usize>);
index_operators!(Point3<S>, 3, [S], RangeTo<usize>);
index_operators!(Point2<S>, 2, [S], RangeFrom<usize>);
index_operators!(Point3<S>, 3, [S], RangeFrom<usize>);
index_operators!(Point2<S>, 2, [S], RangeFull);
index_operators!(Point3<S>, 3, [S], RangeFull);

impl<S: BaseNum> fmt::Debug for Point2<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:?}, {:?}]", self.x, self.y)
    }
}

impl<S: BaseNum> fmt::Debug for Point3<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:?}, {:?}, {:?}]", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    mod point2 {
        use point::*;

        const POINT2: Point2<i32> = Point2 { x: 1, y: 2 };

        #[test]
        fn test_index() {
            assert_eq!(POINT2[0], POINT2.x);
            assert_eq!(POINT2[1], POINT2.y);
        }

        #[test]
        fn test_index_mut() {
            let mut p = POINT2;
            *&mut p[0] = 0;
            assert_eq!(p, [0, 2].into());
        }

        #[test]
        #[should_panic]
        fn test_index_out_of_bounds() {
            POINT2[2];
        }

        #[test]
        fn test_index_range() {
            assert_eq!(&POINT2[..0], &[]);
            assert_eq!(&POINT2[..1], &[1]);
            assert_eq!(POINT2[..0].len(), 0);
            assert_eq!(POINT2[..1].len(), 1);
            assert_eq!(&POINT2[2..], &[]);
            assert_eq!(&POINT2[1..], &[2]);
            assert_eq!(POINT2[2..].len(), 0);
            assert_eq!(POINT2[1..].len(), 1);
            assert_eq!(&POINT2[..], &[1, 2]);
            assert_eq!(POINT2[..].len(), 2);
        }

        #[test]
        fn test_into() {
            let p = POINT2;
            {
                let p: [i32; 2] = p.into();
                assert_eq!(p, [1, 2]);
            }
            {
                let p: (i32, i32) = p.into();
                assert_eq!(p, (1, 2));
            }
        }

        #[test]
        fn test_as_ref() {
            let p = POINT2;
            {
                let p: &[i32; 2] = p.as_ref();
                assert_eq!(p, &[1, 2]);
            }
            {
                let p: &(i32, i32) = p.as_ref();
                assert_eq!(p, &(1, 2));
            }
        }

        #[test]
        fn test_as_mut() {
            let mut p = POINT2;
            {
                let p: &mut [i32; 2] = p.as_mut();
                assert_eq!(p, &mut [1, 2]);
            }
            {
                let p: &mut (i32, i32) = p.as_mut();
                assert_eq!(p, &mut (1, 2));
            }
        }

        #[test]
        fn test_from() {
            assert_eq!(Point2::from([1, 2]), POINT2);
            {
                let p = &[1, 2];
                let p: &Point2<_> = From::from(p);
                assert_eq!(p, &POINT2);
            }
            {
                let p = &mut [1, 2];
                let p: &mut Point2<_> = From::from(p);
                assert_eq!(p, &POINT2);
            }
            assert_eq!(Point2::from((1, 2)), POINT2);
            {
                let p = &(1, 2);
                let p: &Point2<_> = From::from(p);
                assert_eq!(p, &POINT2);
            }
            {
                let p = &mut (1, 2);
                let p: &mut Point2<_> = From::from(p);
                assert_eq!(p, &POINT2);
            }
        }
    }

    mod point3 {
        use point::*;

        const POINT3: Point3<i32> = Point3 { x: 1, y: 2, z: 3 };

        #[test]
        fn test_index() {
            assert_eq!(POINT3[0], POINT3.x);
            assert_eq!(POINT3[1], POINT3.y);
            assert_eq!(POINT3[2], POINT3.z);
        }

        #[test]
        fn test_index_mut() {
            let mut p = POINT3;
            *&mut p[1] = 0;
            assert_eq!(p, [1, 0, 3].into());
        }

        #[test]
        #[should_panic]
        fn test_index_out_of_bounds() {
            POINT3[3];
        }

        #[test]
        fn test_index_range() {
            assert_eq!(&POINT3[..1], &[1]);
            assert_eq!(&POINT3[..2], &[1, 2]);
            assert_eq!(POINT3[..1].len(), 1);
            assert_eq!(POINT3[..2].len(), 2);
            assert_eq!(&POINT3[2..], &[3]);
            assert_eq!(&POINT3[1..], &[2, 3]);
            assert_eq!(POINT3[2..].len(), 1);
            assert_eq!(POINT3[1..].len(), 2);
            assert_eq!(&POINT3[..], &[1, 2, 3]);
            assert_eq!(POINT3[..].len(), 3);
        }

        #[test]
        fn test_into() {
            let p = POINT3;
            {
                let p: [i32; 3] = p.into();
                assert_eq!(p, [1, 2, 3]);
            }
            {
                let p: (i32, i32, i32) = p.into();
                assert_eq!(p, (1, 2, 3));
            }
        }

        #[test]
        fn test_as_ref() {
            let p = POINT3;
            {
                let p: &[i32; 3] = p.as_ref();
                assert_eq!(p, &[1, 2, 3]);
            }
            {
                let p: &(i32, i32, i32) = p.as_ref();
                assert_eq!(p, &(1, 2, 3));
            }
        }

        #[test]
        fn test_as_mut() {
            let mut p = POINT3;
            {
                let p: &mut [i32; 3] = p.as_mut();
                assert_eq!(p, &mut [1, 2, 3]);
            }
            {
                let p: &mut (i32, i32, i32) = p.as_mut();
                assert_eq!(p, &mut (1, 2, 3));
            }
        }

        #[test]
        fn test_from() {
            assert_eq!(Point3::from([1, 2, 3]), POINT3);
            {
                let p = &[1, 2, 3];
                let p: &Point3<_> = From::from(p);
                assert_eq!(p, &POINT3);
            }
            {
                let p = &mut [1, 2, 3];
                let p: &mut Point3<_> = From::from(p);
                assert_eq!(p, &POINT3);
            }
            assert_eq!(Point3::from((1, 2, 3)), POINT3);
            {
                let p = &(1, 2, 3);
                let p: &Point3<_> = From::from(p);
                assert_eq!(p, &POINT3);
            }
            {
                let p = &mut (1, 2, 3);
                let p: &mut Point3<_> = From::from(p);
                assert_eq!(p, &POINT3);
            }
        }
    }
}
