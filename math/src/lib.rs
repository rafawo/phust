// Copyright (c) 2020-2021 Rafael Alcaraz Mercado. All rights reserved.
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

extern crate num_traits;
extern crate serde;

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

    /// Flips the sign of all the coordinates of the vector.
    ///
    /// # Remarks
    /// This function follows the Builder pattern, so it can be chained to other
    /// methods that modify the vector.
    pub fn invert(&mut self) -> &mut Self {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        self
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

    /// Transforms a non-zero vector into a vector of unit length.
    pub fn normalize(&mut self) -> &mut Self {
        let length = self.magnitude();
        if length > num_traits::zero() {
            self.scalar_div(length);
        }
        self
    }

    /// Scalar addition of the vector.
    ///
    /// # Remarks
    /// This function follows the Builder pattern, so it can be chained to other
    /// methods that modify the vector.
    pub fn scalar_add(&mut self, scalar: F) -> &mut Self {
        self.x = self.x + scalar;
        self.y = self.y + scalar;
        self.z = self.z + scalar;
        self
    }

    /// Scalar substraction of the vector.
    ///
    /// # Remarks
    /// This function follows the Builder pattern, so it can be chained to other
    /// methods that modify the vector.
    pub fn scalar_sub(&mut self, scalar: F) -> &mut Self {
        self.scalar_add(-scalar)
    }

    /// Scalar multiplication of the vector.
    ///
    /// # Remarks
    /// This function follows the Builder pattern, so it can be chained to other
    /// methods that modify the vector.
    pub fn scalar_mul(&mut self, scalar: F) -> &mut Self {
        self.x = self.x * scalar;
        self.y = self.y * scalar;
        self.z = self.z * scalar;
        self
    }

    /// Scalar division of the vector.
    ///
    /// # Remarks
    /// This function follows the Builder pattern, so it can be chained to other
    /// methods that modify the vector.
    pub fn scalar_div(&mut self, scalar: F) -> &mut Self {
        self.x = self.x / scalar;
        self.y = self.y / scalar;
        self.z = self.z / scalar;
        self
    }

    /// Adds the vector to another one.
    ///
    /// # Remarks
    /// This function follows the Builder pattern, so it can be chained to other
    /// methods that modify the vector.
    pub fn vector_add(&mut self, other: &Vector3<F>) -> &mut Self {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
        self.z = self.z + other.z;
        self
    }

    /// Substracts the vector to another one.
    ///
    /// # Remarks
    /// This function follows the Builder pattern, so it can be chained to other
    /// methods that modify the vector.
    pub fn vector_sub(&mut self, other: &Vector3<F>) -> &mut Self {
        self.x = self.x - other.x;
        self.y = self.y - other.y;
        self.z = self.z - other.z;
        self
    }

    /// Multiplies the vector to another one.
    ///
    /// # Remarks
    /// This function follows the Builder pattern, so it can be chained to other
    /// methods that modify the vector.
    pub fn vector_mul(&mut self, other: &Vector3<F>) -> &mut Self {
        self.x = self.x * other.x;
        self.y = self.y * other.y;
        self.z = self.z * other.z;
        self
    }

    /// Divides the vector to another one.
    ///
    /// # Remarks
    /// This function follows the Builder pattern, so it can be chained to other
    /// methods that modify the vector.
    pub fn vector_div(&mut self, other: &Vector3<F>) -> &mut Self {
        self.x = self.x / other.x;
        self.y = self.y / other.y;
        self.z = self.z / other.z;
        self
    }
}

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

macro_rules! impl_vec3_operator {
    ($trait:ident, $trait_assign:ident, $fn_name:ident, $fn_name_assign:ident, $scalar_method:ident, $vector_method:ident) => {
        impl<F: $trait<Output = F> + num_traits::Float + Copy> $trait<F> for &Vector3<F> {
            type Output = Vector3<F>;
            fn $fn_name(self, other: F) -> Vector3<F> {
                let mut copy = *self;
                *copy.$scalar_method(other)
            }
        }

        impl<F: $trait<Output = F> + num_traits::Float + Copy> $trait<F> for Vector3<F> {
            type Output = Vector3<F>;
            fn $fn_name(self, other: F) -> Vector3<F> {
                let mut copy = self;
                *copy.$scalar_method(other)
            }
        }

        impl<F: $trait_assign + num_traits::Float + Copy> $trait_assign<F> for &mut Vector3<F> {
            fn $fn_name_assign(&mut self, other: F) {
                self.$scalar_method(other);
            }
        }

        impl<F: $trait_assign + num_traits::Float + Copy> $trait_assign<F> for Vector3<F> {
            fn $fn_name_assign(&mut self, other: F) {
                self.$scalar_method(other);
            }
        }

        impl<F: $trait<Output = F> + num_traits::Float + Copy> $trait<&Vector3<F>> for &Vector3<F> {
            type Output = Vector3<F>;
            fn $fn_name(self, other: &Vector3<F>) -> Vector3<F> {
                let mut copy = *self;
                *copy.$vector_method(other)
            }
        }

        impl<F: $trait<Output = F> + num_traits::Float + Copy> $trait<&Vector3<F>> for Vector3<F> {
            type Output = Vector3<F>;
            fn $fn_name(self, other: &Vector3<F>) -> Vector3<F> {
                let mut copy = self;
                *copy.$vector_method(other)
            }
        }
    };
}

impl_vec3_operator!(Add, AddAssign, add, add_assign, scalar_add, vector_add);
impl_vec3_operator!(Sub, SubAssign, sub, sub_assign, scalar_sub, vector_sub);
impl_vec3_operator!(Mul, MulAssign, mul, mul_assign, scalar_mul, vector_mul);
impl_vec3_operator!(Div, DivAssign, div, div_assign, scalar_div, vector_div);

#[cfg(test)]
mod vector3_test {
    use super::*;

    #[test]
    fn general_usage() {
        assert_eq!(
            Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0
            },
            Vector3::origin()
        );
        let mut vec3 = Vector3::<f64>::new(1.0, 2.0, 3.0);
        assert_eq!(
            Vector3 {
                x: 1.0,
                y: 0.0,
                z: 0.0
            },
            vec3.x()
        );
        assert_eq!(
            Vector3 {
                x: 0.0,
                y: 2.0,
                z: 0.0
            },
            vec3.y()
        );
        assert_eq!(
            Vector3 {
                x: 0.0,
                y: 0.0,
                z: 3.0
            },
            vec3.z()
        );
        assert_eq!(
            Vector3 {
                x: -1.0,
                y: -2.0,
                z: -3.0
            },
            *vec3.invert()
        );
        assert_eq!(
            Vector3 {
                x: 1.0,
                y: 2.0,
                z: 3.0
            },
            *vec3.invert()
        );
        assert_eq!(14.0, vec3.squared_magnitude());
        assert_eq!(4.0, vec3.magnitude().ceil());
        assert_eq!(
            Vector3 {
                x: 0.2672612419124244,
                y: 0.5345224838248488,
                z: 0.8017837257372732
            },
            *vec3.normalize()
        );
    }

    #[test]
    fn scalar_operations() {
        let mut vec3 = Vector3::<f64>::new(1.5, 1.5, 1.5);

        // Addition
        assert_eq!(
            Vector3 {
                x: 3.0,
                y: 3.0,
                z: 3.0
            },
            *vec3.scalar_add(1.5)
        );
        assert_eq!(
            Vector3 {
                x: 4.5,
                y: 4.5,
                z: 4.5
            },
            vec3 + 1.5
        );
        vec3 += 0.5;
        assert_eq!(
            Vector3 {
                x: 3.5,
                y: 3.5,
                z: 3.5
            },
            vec3
        );

        // Substraction
        assert_eq!(
            Vector3 {
                x: 1.5,
                y: 1.5,
                z: 1.5
            },
            *vec3.scalar_sub(2.0)
        );
        assert_eq!(
            Vector3 {
                x: 0.5,
                y: 0.5,
                z: 0.5
            },
            vec3 - 1.0
        );
        vec3 -= 2.5;
        assert_eq!(
            Vector3 {
                x: -1.0,
                y: -1.0,
                z: -1.0
            },
            vec3
        );

        // Multiplication
        assert_eq!(
            Vector3 {
                x: -3.0,
                y: -3.0,
                z: -3.0
            },
            *vec3.scalar_mul(3.0)
        );
        assert_eq!(
            Vector3 {
                x: -6.0,
                y: -6.0,
                z: -6.0
            },
            vec3 * 2.0
        );
        vec3 *= -2.0;
        assert_eq!(
            Vector3 {
                x: 6.0,
                y: 6.0,
                z: 6.0
            },
            vec3
        );

        // Division
        assert_eq!(
            Vector3 {
                x: 1.0,
                y: 1.0,
                z: 1.0
            },
            *vec3.scalar_div(6.0)
        );
        assert_eq!(
            Vector3 {
                x: 0.5,
                y: 0.5,
                z: 0.5
            },
            vec3 / 2.0
        );
        vec3 /= 2.0;
        assert_eq!(
            Vector3 {
                x: 0.5,
                y: 0.5,
                z: 0.5
            },
            vec3
        );
    }

    #[test]
    fn chaining_operations() {
        let mut vec3 = Vector3::<f64>::new(1.5, 1.5, 1.5);
        assert_eq!(
            Vector3 {
                x: 1.0,
                y: 1.0,
                z: 1.0
            },
            *vec3
                .scalar_add(1.5)
                .scalar_sub(1.5)
                .scalar_mul(2.0)
                .scalar_div(3.0)
        );
        assert_eq!(
            Vector3 {
                x: -0.5773502691896258,
                y: -0.5773502691896258,
                z: -0.5773502691896258
            },
            *vec3.invert().normalize()
        );
    }
}
