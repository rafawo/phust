// Copyright (c) 2020-2021 Rafael Alcaraz Mercado. All rights reserved.
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

extern crate num_traits;
extern crate serde;

#[cfg(test)]
mod vector3_test;

use serde::{Deserialize, Serialize};

/// Vector in 3 dimensions.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Vector3<F: num_traits::Float = f64> {
    pub x: F,
    pub y: F,
    pub z: F,
}

impl<F: num_traits::Float> Vector3<F> {
    /// Creates a vector with all its coordinates at origin (0, 0, 0).
    pub fn origin() -> Self {
        Self {
            x: num_traits::zero(),
            y: num_traits::zero(),
            z: num_traits::zero(),
        }
    }

    /// Creates a new vector with the specified coordinates.
    pub fn new(x: F, y: F, z: F) -> Self {
        Self { x, y, z }
    }

    /// Returns a new vector with a copy of coordinate `x`
    /// and the others set to `0`.
    pub fn x(&self) -> Self {
        Self {
            x: self.x,
            y: num_traits::zero(),
            z: num_traits::zero(),
        }
    }

    /// Returns a new vector with a copy of coordinate `y`
    /// and the others set to `0`.
    pub fn y(&self) -> Self {
        Self {
            x: num_traits::zero(),
            y: self.y,
            z: num_traits::zero(),
        }
    }

    /// Returns a new vector with a copy of coordinate `z`
    /// and the others set to `0`.
    pub fn z(&self) -> Self {
        Self {
            x: num_traits::zero(),
            y: num_traits::zero(),
            z: self.z,
        }
    }

    /// Returns the magnitude of the vector.
    /// Magnitude represents the length of the vector.
    pub fn magnitude(&self) -> F {
        (self.squared_magnitude()).sqrt()
    }

    /// Returns the squared magnitude of the vector.
    pub fn squared_magnitude(&self) -> F {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    /// Flips the sign of all the coordinates of the vector.
    pub fn invert(&self) -> Self {
        let mut copy = *self;
        copy.inplace_invert();
        copy
    }

    /// Flips the sign of all the coordinates of the vector.
    ///
    /// # Remarks
    /// This function follows the Builder pattern, so it can be chained to other
    /// methods that modify the vector.
    pub fn inplace_invert(&mut self) -> &mut Self {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        self
    }

    /// Transforms a non-zero vector into a vector of unit length.
    pub fn normalize(&self) -> Self {
        let mut copy = *self;
        copy.inplace_normalize();
        copy
    }

    /// Transforms a non-zero vector into a vector of unit length.
    ///
    /// # Remarks
    /// This function follows the Builder pattern, so it can be chained to other
    /// methods that modify the vector.
    pub fn inplace_normalize(&mut self) -> &mut Self {
        let length = self.magnitude();
        if length > num_traits::zero() {
            self.inplace_scalar_div(length);
        }
        self
    }

    /// Scalar addition of the vector.
    pub fn scalar_add(&self, scalar: F) -> Self {
        let mut copy = *self;
        copy.inplace_scalar_add(scalar);
        copy
    }

    /// Scalar addition of the vector.
    ///
    /// # Remarks
    /// This function follows the Builder pattern, so it can be chained to other
    /// methods that modify the vector.
    pub fn inplace_scalar_add(&mut self, scalar: F) -> &mut Self {
        self.x = self.x + scalar;
        self.y = self.y + scalar;
        self.z = self.z + scalar;
        self
    }

    /// Scalar substraction of the vector.
    pub fn scalar_sub(&self, scalar: F) -> Self {
        let mut copy = *self;
        copy.inplace_scalar_sub(scalar);
        copy
    }

    /// Scalar substraction of the vector.
    ///
    /// # Remarks
    /// This function follows the Builder pattern, so it can be chained to other
    /// methods that modify the vector.
    pub fn inplace_scalar_sub(&mut self, scalar: F) -> &mut Self {
        self.inplace_scalar_add(-scalar)
    }

    /// Scalar multiplication of the vector.
    pub fn scalar_mul(&self, scalar: F) -> Self {
        let mut copy = *self;
        copy.inplace_scalar_mul(scalar);
        copy
    }

    /// Scalar multiplication of the vector.
    ///
    /// # Remarks
    /// This function follows the Builder pattern, so it can be chained to other
    /// methods that modify the vector.
    pub fn inplace_scalar_mul(&mut self, scalar: F) -> &mut Self {
        self.x = self.x * scalar;
        self.y = self.y * scalar;
        self.z = self.z * scalar;
        self
    }

    /// Scalar division of the vector.
    pub fn scalar_div(&self, scalar: F) -> Self {
        let mut copy = *self;
        copy.inplace_scalar_div(scalar);
        copy
    }

    /// Scalar division of the vector.
    ///
    /// # Remarks
    /// This function follows the Builder pattern, so it can be chained to other
    /// methods that modify the vector.
    pub fn inplace_scalar_div(&mut self, scalar: F) -> &mut Self {
        self.x = self.x / scalar;
        self.y = self.y / scalar;
        self.z = self.z / scalar;
        self
    }

    /// Adds the vector to another one.
    pub fn vector_add(&self, other: &Vector3<F>) -> Self {
        let mut copy = *self;
        copy.inplace_vector_add(other);
        copy
    }

    /// Adds the vector to another one.
    ///
    /// # Remarks
    /// This function follows the Builder pattern, so it can be chained to other
    /// methods that modify the vector.
    pub fn inplace_vector_add(&mut self, other: &Vector3<F>) -> &mut Self {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
        self.z = self.z + other.z;
        self
    }

    /// Substracts the vector to another one.
    pub fn vector_sub(&self, other: &Vector3<F>) -> Self {
        let mut copy = *self;
        copy.inplace_vector_sub(other);
        copy
    }

    /// Substracts the vector to another one.
    ///
    /// # Remarks
    /// This function follows the Builder pattern, so it can be chained to other
    /// methods that modify the vector.
    pub fn inplace_vector_sub(&mut self, other: &Vector3<F>) -> &mut Self {
        self.x = self.x - other.x;
        self.y = self.y - other.y;
        self.z = self.z - other.z;
        self
    }

    /// Multiplies the vector to another one.
    pub fn vector_mul(&self, other: &Vector3<F>) -> Self {
        let mut copy = *self;
        copy.inplace_vector_mul(other);
        copy
    }

    /// Multiplies the vector to another one.
    ///
    /// # Remarks
    /// This function follows the Builder pattern, so it can be chained to other
    /// methods that modify the vector.
    pub fn inplace_vector_mul(&mut self, other: &Vector3<F>) -> &mut Self {
        self.x = self.x * other.x;
        self.y = self.y * other.y;
        self.z = self.z * other.z;
        self
    }

    /// Divides the vector to another one.
    pub fn vector_div(&self, other: &Vector3<F>) -> Self {
        let mut copy = *self;
        copy.inplace_vector_div(other);
        copy
    }

    /// Divides the vector to another one.
    ///
    /// # Remarks
    /// This function follows the Builder pattern, so it can be chained to other
    /// methods that modify the vector.
    pub fn inplace_vector_div(&mut self, other: &Vector3<F>) -> &mut Self {
        self.x = self.x / other.x;
        self.y = self.y / other.y;
        self.z = self.z / other.z;
        self
    }

    /// Calculates the dot product of two vectors, aka Scalar Product, Inner Product.
    /// The returned scalar calculates the magnitude of one vector in the direction of another.
    pub fn dot_product(&self, other: &Vector3<F>) -> F {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    /// Calculates the angle in radians between two vectors.
    pub fn theta(&self, other: &Vector3<F>) -> F {
        self.normalize().dot_product(&other.normalize()).acos()
    }

    /// Calculates the cross product of two vectors, aka Vector Product.
    /// The resulting vector represents the component of `other` that is not
    /// in the direction of `self`, scaled by the magnitude of `self`. It's also
    /// used to represent the direction that is at right angles to both vectors.
    pub fn cross_product(&self, other: &Vector3<F>) -> Vector3<F> {
        let mut copy = *self;
        copy.inplace_cross_product(other);
        copy
    }

    /// Calculates the cross product of two vectors, aka Vector Product.
    /// The resulting vector represents the component of `other` that is not
    /// in the direction of `self`, scaled by the magnitude of `self`. It's also
    /// used to represent the direction that is at right angles to both vectors.
    ///
    /// # Remarks
    /// This function follows the Builder pattern, so it can be chained to other
    /// methods that modify the vector.
    pub fn inplace_cross_product(&mut self, other: &Vector3<F>) -> &mut Vector3<F> {
        let (x, y, z) = (self.x, self.y, self.z);
        self.x = (y * other.z) - (z * other.y);
        self.y = (z * other.x) - (x * other.z);
        self.z = (x * other.y) - (y * other.x);
        self
    }
}

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

macro_rules! impl_vec3_operator {
    ($trait:ident, $trait_assign:ident, $fn_name:ident, $fn_name_assign:ident, $scalar_method:ident, $scalar_method_assign:ident, $vector_method:ident, $vector_method_assign:ident) => {
        impl<F: $trait<Output = F> + num_traits::Float + Copy> $trait<F> for &Vector3<F> {
            type Output = Vector3<F>;
            fn $fn_name(self, other: F) -> Vector3<F> {
                self.$scalar_method(other)
            }
        }

        impl<F: $trait<Output = F> + num_traits::Float + Copy> $trait<F> for Vector3<F> {
            type Output = Vector3<F>;
            fn $fn_name(self, other: F) -> Vector3<F> {
                self.$scalar_method(other)
            }
        }

        impl<F: $trait_assign + num_traits::Float + Copy> $trait_assign<F> for Vector3<F> {
            fn $fn_name_assign(&mut self, other: F) {
                self.$scalar_method_assign(other);
            }
        }

        impl<F: $trait<Output = F> + num_traits::Float + Copy> $trait<&Vector3<F>> for &Vector3<F> {
            type Output = Vector3<F>;
            fn $fn_name(self, other: &Vector3<F>) -> Vector3<F> {
                self.$vector_method(other)
            }
        }

        impl<F: $trait<Output = F> + num_traits::Float + Copy> $trait<Vector3<F>> for &Vector3<F> {
            type Output = Vector3<F>;
            fn $fn_name(self, other: Vector3<F>) -> Vector3<F> {
                self.$vector_method(&other)
            }
        }

        impl<F: $trait<Output = F> + num_traits::Float + Copy> $trait<&Vector3<F>> for Vector3<F> {
            type Output = Vector3<F>;
            fn $fn_name(self, other: &Vector3<F>) -> Vector3<F> {
                self.$vector_method(other)
            }
        }

        impl<F: $trait<Output = F> + num_traits::Float + Copy> $trait<Vector3<F>> for Vector3<F> {
            type Output = Vector3<F>;
            fn $fn_name(self, other: Vector3<F>) -> Vector3<F> {
                self.$vector_method(&other)
            }
        }

        impl<F: $trait_assign + num_traits::Float + Copy> $trait_assign<&Vector3<F>>
            for Vector3<F>
        {
            fn $fn_name_assign(&mut self, other: &Vector3<F>) {
                self.$vector_method_assign(other);
            }
        }

        impl<F: $trait_assign + num_traits::Float + Copy> $trait_assign<Vector3<F>> for Vector3<F> {
            fn $fn_name_assign(&mut self, other: Vector3<F>) {
                self.$vector_method_assign(&other);
            }
        }
    };
}

impl_vec3_operator!(
    Add,
    AddAssign,
    add,
    add_assign,
    scalar_add,
    inplace_scalar_add,
    vector_add,
    inplace_vector_add
);
impl_vec3_operator!(
    Sub,
    SubAssign,
    sub,
    sub_assign,
    scalar_sub,
    inplace_scalar_sub,
    vector_sub,
    inplace_vector_sub
);
impl_vec3_operator!(
    Mul,
    MulAssign,
    mul,
    mul_assign,
    scalar_mul,
    inplace_scalar_mul,
    vector_mul,
    inplace_vector_mul
);
impl_vec3_operator!(
    Div,
    DivAssign,
    div,
    div_assign,
    scalar_div,
    inplace_scalar_div,
    vector_div,
    inplace_vector_div
);
