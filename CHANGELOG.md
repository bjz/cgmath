# Change Log

All notable changes to this project will be documented in this file, following
the format defined at [keepachangelog.com](http://keepachangelog.com/).
This project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

### Changed

- Reorder euler angles in quaternion conversions from `(yaw, pitch, roll)` to
  `(pitch, yaw, roll)` to be consistent with the ordering in matrix conversions.
- Base the `Rotation::from_angle_{x, y, z}` default implementations off
  `Rotation::from_euler` as opposed to `Rotation::from_axis_angle`.
- Fix implementation of `Quaternion::from_axis_angle`

## [v0.7.0] - 2015-12-23

### Added
- Add missing by-ref and by-val permutations of `Vector`, `Matrix`, `Point`,
  `Quaternion` and `Angle` operators.
- Ease lifetime constraints by removing `'static` from some scalar type
  parameters.
- Weaken type constraints on `perspective` function to take an `Into<Rad<S>>`.
- Add `Angle::new` for constructing angles from a unitless scalar.
- Implement assignment operators for nightly builds, enabled by the `"unstable"`
  feature.

### Changed
- `Vector`, `Matrix`, `Point`, and `Angle` are now constrained to require
  specific operators to be overloaded. This means that generic code can now use
  operators, instead of the operator methods.
- Take a `Rad` for `ProjectionFov::fovy`, rather than arbitrary `Angle`s. This
  simplifies the signature of `PerspectiveFov` from `PerspectiveFov<S, A>` to
  `PerspectiveFov<S>`.
- The following trait constraints were removed from `Angle`: `Debug`,
  `ScalarConv`, `Into<Rad<S>>`, `Into<Deg<S>>`.
- `Angle` no longer requires `One`, and the implementations have been removed
  from `Deg` and `Rad`. This is because angles do not close over multiplication,
  and therefore cannot have a multiplicative identity. If we were truly accurate,
  `Angle * Angle` would return an `Angle^2` (not supported by the current api).
- Make remainder operators on `Angle`s make sense from the perspective of
  dimensional analysis.
- Moved free trigonometric functions onto `Angle`.

### Removed
- Remove redundant `Point::{min, max}` methods - these are now covered by the
  `Array::{min, max}` methods that were introduced in 0.5.0.
- Removed `ToComponents`, `ToComponents2`, and `ToComponents3`. If you were
  relying on `ToComponents::decompose`, you can produce the same effect by
  accessing the fields on `Decomposed` directly. To create the scale vector,
  use: `Vector::from_value(transform.scale)`.
- Removed `CompositeTransform`, `CompositeTransform2`, and `CompositeTransform3`.
- Remove `Vector::one`. Vectors don't really have a multiplicative identity.
  If you really want a `one` vector, you can do something like:
  `Vector::from_value(1.0)`.
- Remove operator methods from `Vector`, `Matrix`, `Point`, and `Angle` traits
  in favor of operator overloading.
- Remove `*_self` methods from `Vector`, `Matrix`, `Point`, and `Angle`. The
  operator methods can be used via the unstable assignment operators.
- Remove `#[derive(Hash)]` from `Deg` and `Rad`. This could never really be used
  these types, because they expect to be given a `BaseFloat` under normal
  circumstances.

## [v0.6.0] - 2015-12-12

### Added
- This CHANGELOG for keeping track of notable changes.
- `Matrix4::{from_scale, from_nonuniform_scale}` for easily constructing
  homogeneous scale matrices.

### Changed
- Renamed `SquareMatrix::one` to `SquareMatrix::identity`. `identity` is easier
  to search for,
  and the more common name for the multiplicative identity for matrices.
- Matrix impls have now been constrained to `S: BaseFloat`.

## [v0.5.0] - 2015-11-20

### Changed
- Take many point and vector parameters by value.
- Take point and vector operator overloads by value.
- Divide `Matrix` trait into `Matrix` and `SquareMatrix`, opening the door for
  non-square matrices in the future.
- Make many trait type parameters associated types.
- Move element-wise methods from `Vector` and `Point` onto the `Array1` trait,
  and rename it to `Array`.
- Make pointer access methods on `Array` match the naming scheme of those in the
  standard library.

### Removed
- Removed collision types: `Ray`, `Plane`, `Frustum`, `Aabb2`, `Aabb3` `Obb2`,
  `Obb3` `Sphere`, `Cylinder`. These can now be found at
  [csherratt/collision-rs](https://github.com/csherratt/collision-rs).
- Remove `Array2` trait, moving methods onto the `Matrix` trait.

## [v0.4.0] - 2015-10-25

## [v0.3.1] - 2015-09-20

## [v0.3.0] - 2015-09-20

## [v0.2.0] - 2015-05-11

## [v0.1.6] - 2015-05-10

## [v0.1.5] - 2015-04-25

## [v0.1.4] - 2015-04-24

## [v0.1.3] - 2015-04-06

## [v0.1.2] - 2015-04-01

## [v0.1.1] - 2015-03-25

## [v0.1.0] - 2015-03-15

## [v0.0.8] - 2015-03-09

## [v0.0.7] - 2015-03-01

## [v0.0.6] - 2015-02-21

## [v0.0.5] - 2015-02-16

## [v0.0.4] - 2015-02-11

## [v0.0.3] - 2015-02-08

## v0.0.1 - 2014-06-24

[Unreleased]: https://github.com/bjz/cgmath/compare/v0.7.0...HEAD
[v0.7.0]: https://github.com/bjz/cgmath/compare/v0.6.0...v0.7.0
[v0.6.0]: https://github.com/bjz/cgmath/compare/v0.5.0...v0.6.0
[v0.5.0]: https://github.com/bjz/cgmath/compare/v0.4.0...v0.5.0
[v0.4.0]: https://github.com/bjz/cgmath/compare/v0.3.1...v0.4.0
[v0.3.1]: https://github.com/bjz/cgmath/compare/v0.3.0...v0.3.1
[v0.3.0]: https://github.com/bjz/cgmath/compare/v0.2.0...v0.3.0
[v0.2.0]: https://github.com/bjz/cgmath/compare/v0.1.6...v0.2.0
[v0.1.6]: https://github.com/bjz/cgmath/compare/v0.1.5...v0.1.6
[v0.1.5]: https://github.com/bjz/cgmath/compare/v0.1.4...v0.1.5
[v0.1.4]: https://github.com/bjz/cgmath/compare/v0.1.3...v0.1.4
[v0.1.3]: https://github.com/bjz/cgmath/compare/v0.1.2...v0.1.3
[v0.1.2]: https://github.com/bjz/cgmath/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/bjz/cgmath/compare/v0.1.0...v0.1.1
[v0.1.0]: https://github.com/bjz/cgmath/compare/v0.0.8...v0.1.0
[v0.0.8]: https://github.com/bjz/cgmath/compare/v0.0.7...v0.0.8
[v0.0.7]: https://github.com/bjz/cgmath/compare/v0.0.6...v0.0.7
[v0.0.6]: https://github.com/bjz/cgmath/compare/v0.0.5...v0.0.6
[v0.0.5]: https://github.com/bjz/cgmath/compare/v0.0.4...v0.0.5
[v0.0.4]: https://github.com/bjz/cgmath/compare/v0.0.3...v0.0.4
[v0.0.3]: https://github.com/bjz/cgmath/compare/v0.0.1...v0.0.3
