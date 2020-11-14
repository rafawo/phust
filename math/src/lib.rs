// Copyright (c) 2020-2021 Rafael Alcaraz Mercado. All rights reserved.
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

extern crate num_traits as num;

/// Vector in 3 dimensions.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
#[cfg_attr(feature = "serde-serde", derive(Serialize, Deserialize))]
pub struct Vector3<R: num::real::Real = f64> {
    pub x: R,
    pub y: R,
    pub z: R,
}

impl<R: num::real::Real> Vector3<R> {
    /// Creates a vector with all its coordinates at origin (0, 0, 0).
    pub fn origin() -> Self {
        Self {
            x: num::Zero::zero(),
            y: num::Zero::zero(),
            z: num::Zero::zero(),
        }
    }

    /// Creates a new vector with the specified coordinates.
    pub fn new(x: R, y: R, z: R) -> Self {
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
}
