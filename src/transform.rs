//! Dedicated transformation types as the combination of primitives.
//! 
//! Note that you may want to us these types over the corresponding type of
//! homogeneous transformation matrix because they are faster in most operations,
//! especially composition and inverse.
use crate::*;

use std::ops::*;

macro_rules! isometries {
    ($($ison:ident => ($mt:ident, $rt:ident, $vt:ident, $t:ident)),+) => {
        $(
        /// An Isometry, aka a "rigid body transformation".
        ///
        /// Defined as the combination of a rotation *and then* a translation.
        /// 
        /// You may want to us this type over the corresponding type of
        /// homogeneous transformation matrix because it will be faster in most operations,
        /// especially composition and inverse.
        #[derive(Clone, Copy, Debug)]
        #[repr(C)]
        pub struct $ison {
            pub translation: $vt,
            pub rotation: $rt,
        }

        derive_default_identity!($ison);

        impl $ison {
            #[inline]
            pub fn new(translation: $vt, rotation: $rt) -> Self {
                Self { translation, rotation }
            }

            #[inline]
            pub fn identity() -> Self {
                Self { rotation: $rt::identity(), translation: $vt::zero() }
            }

            /// Add a rotation *before* this isometry.
            /// 
            /// This means the rotation will only affect the rotational
            /// part of this isometry, not the translational part.
            #[inline]
            pub fn prepend_rotation(&mut self, rotor: $rt) {
                self.rotation = rotor * self.rotation;
            }

            /// Add a rotation *after* this isometry.
            /// 
            /// This means the rotation will affect both the rotational and
            /// translational parts of this isometry, since it is being applied
            /// 'after' this isometry's translational part.
            pub fn append_rotation(&mut self, rotor: $rt) {
                self.rotation = rotor * self.rotation;
                self.translation = rotor * self.translation;
            }

            /// Add a translation *before* this isometry.
            /// 
            /// Doing so will mean that the translation being added will get
            /// transformed by this isometry's rotational part.
            #[inline]
            pub fn prepend_translation(&mut self, translation: $vt) {
                self.translation += self.rotation * translation;
            }

            /// Add a translation *after* this isometry.
            /// 
            /// Doing so will mean that the translation being added will *not*
            /// transformed by this isometry's rotational part.
            #[inline]
            pub fn append_translation(&mut self, translation: $vt) {
                self.translation += translation;
            }

            /// Prepend transformation by another isometry.
            /// 
            /// This means that the transformation being applied will take place
            /// *before* this isometry, i.e. both its translation and rotation will be
            /// rotated by this isometry's rotational part.
            #[inline]
            pub fn prepend_isometry(&mut self, other: Self) {
                *self = *self * other;
            }

            /// Append transformation by another isometry.
            /// 
            /// This means that the transformation being applied will take place
            /// *after* this isometry, i.e. *this isometry's* translation and rotation will be
            /// rotated by the *other* isometry's rotational part.
            #[inline]
            pub fn append_isometry(&mut self, other: Self) {
                *self = other * *self;
            }

            #[inline]
            pub fn inverse(&mut self) {
                self.rotation.reverse();
                self.translation = self.rotation * (-self.translation);
            }

            #[inline]
            pub fn inversed(mut self) -> Self {
                self.inverse();
                self
            }

            #[inline]
            pub fn transform_vec(&self, mut vec: $vt) -> $vt {
                vec = self.rotation * vec;
                vec += self.translation;
                vec
            }

            #[inline]
            pub fn into_homogeneous_matrix(self) -> $mt {
                $mt::from_translation(self.translation)
                    * self.rotation.into_matrix().into_homogeneous()
            }
        }

        impl Mul<$ison> for $rt {
            type Output = $ison;
            #[inline]
            fn mul(self, mut iso: $ison) -> $ison {
                iso.append_rotation(self);
                iso
            }
        }

        impl Mul<$rt> for $ison {
            type Output = $ison;
            #[inline]
            fn mul(mut self, rotor: $rt) -> $ison {
                self.prepend_rotation(rotor);
                self
            }
        }

        impl Mul<$vt> for $ison {
            type Output = $vt;
            #[inline]
            fn mul(self, vec: $vt) -> $vt {
                self.transform_vec(vec)
            }
        }

        impl Mul<$ison> for $ison {
            type Output = Self;
            #[inline]
            fn mul(self, base: $ison) -> $ison {
                let trans = self.transform_vec(base.translation);
                let rot = self.rotation * base.rotation;
                $ison::new(trans, rot)
            }
        }
        )+
    }
}

isometries!(
    Isometry2 => (Mat3, Rotor2, Vec2, f32), WIsometry2 => (Wat3, WRotor2, Wec2, f32x4),
    Isometry3 => (Mat4, Rotor3, Vec3, f32), WIsometry3 => (Wat4, WRotor3, Wec3, f32x4)
);

macro_rules! similarities {
    ($($sn:ident => ($mt:ident, $rt:ident, $vt:ident, $t:ident)),+) => {
        $(
        /// A Similarity, i.e. an Isometry but with an added uniform scaling.
        ///
        /// Defined as a uniform scaling followed by a rotation followed by a translation.
        /// 
        /// You may want to us this type over the corresponding type of
        /// homogeneous transformation matrix because it will be faster in most operations,
        /// especially composition and inverse.
        #[derive(Clone, Copy, Debug)]
        #[repr(C)]
        pub struct $sn {
            pub translation: $vt,
            pub rotation: $rt,
            pub scale: $t,
        }

        derive_default_identity!($sn);

        impl $sn {
            #[inline]
            pub fn new(translation: $vt, rotation: $rt, scale: $t) -> Self {
                Self { translation, rotation, scale }
            }

            #[inline]
            pub fn identity() -> Self {
                Self { rotation: $rt::identity(), translation: $vt::zero(), scale: $t::from(1.0) }
            }

            /// Add a scaling *before* this similarity.
            /// 
            /// This means the scaling will only affect the scaling part
            /// of this similarity, not the translational part.
            #[inline]
            pub fn prepend_scaling(&mut self, scaling: $t) {
                self.scale *= scaling;
            }

            /// Add a scaling *after* this similarity.
            /// 
            /// This means the scaling will affect both the scaling
            /// and translational parts of this similairty, since it is being
            /// applied *after* this similarity's translational part.
            #[inline]
            pub fn append_scaling(&mut self, scaling: $t) {
                self.scale *= scaling;
                self.translation *= scaling;
            }

            /// Add a rotation *before* this similarity.
            /// 
            /// This means the rotation will only affect the rotational
            /// part of this similarity, not the translational part.
            #[inline]
            pub fn prepend_rotation(&mut self, rotor: $rt) {
                self.rotation = rotor * self.rotation;
            }

            /// Add a rotation *after* this similarity.
            /// 
            /// This means the rotation will affect both the rotational and
            /// translational parts of this similarity, since it is being applied
            /// *after* this similarity's translational part.
            pub fn append_rotation(&mut self, rotor: $rt) {
                self.rotation = rotor * self.rotation;
                self.translation = rotor * self.translation;
            }

            /// Add a translation *before* this similarity.
            /// 
            /// Doing so will mean that the translation being added will get
            /// transformed by this similarity's rotational and scaling parts.
            #[inline]
            pub fn prepend_translation(&mut self, translation: $vt) {
                self.translation += self.scale * self.rotation * translation;
            }

            /// Add a translation *after* this similarity.
            /// 
            /// Doing so will mean that the translation being added will *not*
            /// transformed by this similarity's rotational or scaling parts.
            #[inline]
            pub fn append_translation(&mut self, translation: $vt) {
                self.translation += translation;
            }

            /// Prepend transformation by another similarity.
            /// 
            /// This means that the transformation being applied will take place
            /// *before* this similarity, i.e. both its translation and rotation will be
            /// rotated by the other similarity's rotational part, and its translation
            /// will be scaled by the other similarity's scaling part.
            #[inline]
            pub fn prepend_similarity(&mut self, other: Self) {
                *self = *self * other;
            }

            /// Append transformation by another similarity.
            /// 
            /// This means that the transformation being applied will take place
            /// *after* this similarity, i.e. *this similarity's* translation and rotation will be
            /// rotated by the *other* similarity's rotational part, and *this similarity's* translation
            /// will be scaled by the *other* similarity's scaling pat.
            #[inline]
            pub fn append_similarity(&mut self, other: Self) {
                *self = other * *self;
            }

            #[inline]
            pub fn inverse(&mut self) {
                self.rotation.reverse();
                self.translation = self.rotation * (-self.translation);
                self.scale = $t::from(1.0) / self.scale;
            }

            #[inline]
            pub fn inversed(mut self) -> Self {
                self.inverse();
                self
            }

            #[inline]
            pub fn transform_vec(&self, mut vec: $vt) -> $vt {
                vec = self.rotation * vec;
                vec = self.scale * vec;
                vec += self.translation;
                vec
            }

            #[inline]
            pub fn into_homogeneous_matrix(self) -> $mt {
                $mt::from_translation(self.translation)
                    * self.rotation.into_matrix().into_homogeneous()
            }
        }

        impl Mul<$sn> for $rt {
            type Output = $sn;
            #[inline]
            fn mul(self, mut iso: $sn) -> $sn {
                iso.append_rotation(self);
                iso
            }
        }

        impl Mul<$rt> for $sn {
            type Output = $sn;
            #[inline]
            fn mul(mut self, rotor: $rt) -> $sn {
                self.prepend_rotation(rotor);
                self
            }
        }

        impl Mul<$vt> for $sn {
            type Output = $vt;
            #[inline]
            fn mul(self, vec: $vt) -> $vt {
                self.transform_vec(vec)
            }
        }

        impl Mul<$sn> for $sn {
            type Output = Self;
            #[inline]
            fn mul(self, base: $sn) -> $sn {
                let trans = self.transform_vec(base.translation);
                let rot = self.rotation * base.rotation;
                let scale = self.scale * base.scale;
                $sn::new(trans, rot, scale)
            }
        }
        )+
    }
}

similarities!(
    Similarity2 => (Mat3, Rotor2, Vec2, f32), WSimilarity2 => (Wat3, WRotor2, Wec2, f32x4),
    Similarity3 => (Mat4, Rotor3, Vec3, f32), WSimilarity3 => (Wat4, WRotor3, Wec3, f32x4)
);
