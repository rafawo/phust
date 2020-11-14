// Copyright (c) 2020-2021 Rafael Alcaraz Mercado. All rights reserved.
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

extern crate num_traits;

/// Vector in 3 dimensions.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
#[cfg_attr(feature = "serde-serde", derive(Serialize, Deserialize))]
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
            self.x = self.x / length;
            self.y = self.y / length;
            self.z = self.z / length;
        }
        self
    }
}
