// Copyright (c) 2020-2021 Rafael Alcaraz Mercado. All rights reserved.
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

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
        *vec3.inplace_invert()
    );
    assert_eq!(
        Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0
        },
        *vec3.inplace_invert()
    );
    assert_eq!(14.0, vec3.squared_magnitude());
    assert_eq!(4.0, vec3.magnitude().ceil());
    assert_eq!(
        Vector3 {
            x: 0.2672612419124244,
            y: 0.5345224838248488,
            z: 0.8017837257372732
        },
        *vec3.inplace_normalize()
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
        *vec3.inplace_scalar_add(1.5)
    );
    assert_eq!(
        Vector3 {
            x: 4.5,
            y: 4.5,
            z: 4.5
        },
        vec3 + 1.5
    );
    assert_eq!(
        Vector3 {
            x: 4.5,
            y: 4.5,
            z: 4.5
        },
        &vec3 + 1.5
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
        *vec3.inplace_scalar_sub(2.0)
    );
    assert_eq!(
        Vector3 {
            x: 0.5,
            y: 0.5,
            z: 0.5
        },
        vec3 - 1.0
    );
    assert_eq!(
        Vector3 {
            x: 0.5,
            y: 0.5,
            z: 0.5
        },
        &vec3 - 1.0
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
        *vec3.inplace_scalar_mul(3.0)
    );
    assert_eq!(
        Vector3 {
            x: -6.0,
            y: -6.0,
            z: -6.0
        },
        vec3 * 2.0
    );
    assert_eq!(
        Vector3 {
            x: -6.0,
            y: -6.0,
            z: -6.0
        },
        &vec3 * 2.0
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
        *vec3.inplace_scalar_div(6.0)
    );
    assert_eq!(
        Vector3 {
            x: 0.5,
            y: 0.5,
            z: 0.5
        },
        vec3 / 2.0
    );
    assert_eq!(
        Vector3 {
            x: 0.5,
            y: 0.5,
            z: 0.5
        },
        &vec3 / 2.0
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
            .inplace_scalar_add(1.5)
            .inplace_scalar_sub(1.5)
            .inplace_scalar_mul(2.0)
            .inplace_scalar_div(3.0)
    );
    assert_eq!(
        Vector3 {
            x: -0.5773502691896258,
            y: -0.5773502691896258,
            z: -0.5773502691896258
        },
        *vec3.inplace_invert().inplace_normalize()
    );
}

#[test]
fn vector_operators() {
    let mut a = Vector3::<f64>::new(1.0, 1.0, 1.0);
    let b = Vector3::<f64>::new(2.0, 2.0, 2.0);

    // Addition
    assert_eq!(
        Vector3 {
            x: 3.0,
            y: 3.0,
            z: 3.0
        },
        a + b
    );
    assert_eq!(
        Vector3 {
            x: 3.0,
            y: 3.0,
            z: 3.0
        },
        &a + &b
    );
    assert_eq!(
        Vector3 {
            x: 3.0,
            y: 3.0,
            z: 3.0
        },
        &a + b
    );
    assert_eq!(
        Vector3 {
            x: 3.0,
            y: 3.0,
            z: 3.0
        },
        a + &b
    );
    a += b;
    a += &b;
    assert_eq!(
        Vector3 {
            x: 5.0,
            y: 5.0,
            z: 5.0
        },
        a
    );
    a = Vector3::new(1.0, 1.0, 1.0);

    // Substraction
    assert_eq!(
        Vector3 {
            x: -1.0,
            y: -1.0,
            z: -1.0
        },
        a - b
    );
    assert_eq!(
        Vector3 {
            x: -1.0,
            y: -1.0,
            z: -1.0
        },
        &a - &b
    );
    assert_eq!(
        Vector3 {
            x: -1.0,
            y: -1.0,
            z: -1.0
        },
        &a - b
    );
    assert_eq!(
        Vector3 {
            x: -1.0,
            y: -1.0,
            z: -1.0
        },
        a - &b
    );
    a -= b;
    a -= &b;
    assert_eq!(
        Vector3 {
            x: -3.0,
            y: -3.0,
            z: -3.0
        },
        a
    );
    a = Vector3::new(1.0, 1.0, 1.0);

    // Multiplication
    assert_eq!(
        Vector3 {
            x: 2.0,
            y: 2.0,
            z: 2.0
        },
        a * b
    );
    a *= b;
    a *= &b;
    assert_eq!(
        Vector3 {
            x: 4.0,
            y: 4.0,
            z: 4.0
        },
        a
    );
    a = Vector3::new(1.0, 1.0, 1.0);

    // Division
    assert_eq!(
        Vector3 {
            x: 0.5,
            y: 0.5,
            z: 0.5
        },
        a / b
    );
    assert_eq!(
        Vector3 {
            x: 0.5,
            y: 0.5,
            z: 0.5
        },
        &a / &b
    );
    assert_eq!(
        Vector3 {
            x: 0.5,
            y: 0.5,
            z: 0.5
        },
        &a / b
    );
    assert_eq!(
        Vector3 {
            x: 0.5,
            y: 0.5,
            z: 0.5
        },
        a / &b
    );
    a /= b;
    a /= &b;
    assert_eq!(
        Vector3 {
            x: 0.25,
            y: 0.25,
            z: 0.25
        },
        a
    );
}

#[test]
fn product() {
    let a = Vector3::<f64>::new(1.0, 2.0, 3.0);
    let b = Vector3::<f64>::new(3.0, 2.0, 1.0);
    assert_eq!(3.0 + 4.0 + 3.0, a.dot_product(&b));
    assert_eq!(0.7751933733103613, a.theta(&b));
    assert_eq!(
        Vector3 {
            x: -4.0,
            y: 8.0,
            z: -4.0
        },
        a.cross_product(&b)
    );
}
