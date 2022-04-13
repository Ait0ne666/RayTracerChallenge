use std::f64::consts::PI;

use crate::prelude::equal_floats;

use super::*;

#[test]
fn adding_point_to_vector() {
    assert_eq!(
        point(2.0, 4.0, 6.0) + vector(-1.0, 0.0, 1.0),
        Vector::new(1.0, 4.0, 7.0, 1.0)
    );
}

#[test]
fn subtracting_points() {
    assert_eq!(
        point(2.0, 4.0, 6.0) - point(-1.0, 0.0, 1.0),
        Vector::new(3.0, 4.0, 5.0, 0.0)
    );
}

#[test]
fn subtracting_vector_from_point() {
    assert_eq!(
        point(2.0, 4.0, 6.0) - vector(-1.0, 0.0, 1.0),
        Vector::new(3.0, 4.0, 5.0, 1.0)
    );
}

#[test]
fn subtracting_vector_from_vector() {
    assert_eq!(
        vector(2.0, 4.0, 6.0) - vector(-1.0, 0.0, 1.0),
        Vector::new(3.0, 4.0, 5.0, 0.0)
    );
}

#[test]
fn vector_negates_correctly() {
    assert_eq!(-vector(1.0, -2.0, 3.0), Vector::new(-1.0, 2.0, -3.0, 0.0));
}

#[test]
fn vector_is_multiplied_by_scalar_correctly() {
    assert_eq!(
        3.5 * vector(1.0, -2.0, 3.0),
        Vector::new(3.5, -7.0, 10.5, 0.0)
    );
}

#[test]
fn vector_is_divided_by_scalar_correctly() {
    assert_eq!(
        vector(1.0, -2.0, 3.0) / 2.0,
        Vector::new(0.5, -1.0, 1.5, 0.0)
    );
}

#[test]
fn correctly_calculates_magnitude() {
    assert_eq!(vector(1.0, 0.0, 0.0).magnitude(), 1.0);
}

#[test]
fn correctly_calculates_magnitude_2() {
    assert_eq!(vector(0.0, 1.0, 0.0).magnitude(), 1.0);
}

#[test]
fn correctly_calculates_magnitude_3() {
    let answer: f64 = 14.0;

    assert_eq!(vector(1.0, 2.0, 3.0).magnitude(), answer.sqrt());
}

#[test]
fn correctly_calculates_magnitude_4() {
    let answer: f64 = 14.0;

    assert_eq!(vector(-1.0, -2.0, -3.0).magnitude(), answer.sqrt());
}

#[test]
fn correctly_normalizes_vector() {
    assert_eq!(vector(4.0, 0.0, 0.0).normalize(), vector(1.0, 0.0, 0.0));
}

#[test]
fn correctly_normalizes_vector_2() {
    assert!(vector(1.0, 2.0, 3.0)
        .normalize()
        .appr_equal(vector(0.26726, 0.53452, 0.80178)));
}

#[test]
fn shows_correct_magnitude_for_normalized_vector() {
    assert_eq!(vector(1.0, 2.0, 3.0).normalize().magnitude(), 1.0);
}

#[test]
fn correctly_calculates_dot_product_of_2_vectors() {
    assert_eq!(dot(vector(1.0, 2.0, 3.0), vector(2.0, 3.0, 4.0)), 20.0);
}

#[test]
fn correctly_calculates_cross_product_of_2_vectors() {
    assert_eq!(
        cross(vector(1.0, 2.0, 3.0), vector(2.0, 3.0, 4.0)),
        vector(-1.0, 2.0, -1.0)
    );
    assert_eq!(
        cross(vector(2.0, 3.0, 4.0), vector(1.0, 2.0, 3.0)),
        vector(1.0, -2.0, 1.0)
    );
}

#[test]
fn colors_are_multiplied_correctly() {
    assert!((color(1.0, 0.1, 0.4) * color(2.0, 3.0, 4.0)).appr_equal(color(2.0, 0.3, 1.6)));
}

#[test]
fn correctly_change_pixel_on_canvas() {
    let mut canvas = Canvas::new(10, 20);
    let red = Color::new(1.0, 0.0, 0.0);

    canvas.write_pixel(2, 3, red);

    let result_pixel = &canvas.pixel_at(2, 3);
    assert_eq!(*result_pixel, Pixel::new(2, 3, red));
}

#[test]
fn matrix_created_correctly() {
    assert_eq!(
        matrix([[1.0, 2.0], [1.3, 2.3], [2.0, 3.0]]),
        Matrix {
            elements: [[1.0, 2.0], [1.3, 2.3], [2.0, 3.0]]
        }
    );
}

#[test]
fn matrixes_multiplied_correctly() {
    assert_eq!(
        matrix([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0]
        ]) * matrix([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0]
        ]),
        Matrix {
            elements: [
                [20.0, 22.0, 50.0, 48.0],
                [44.0, 54.0, 114.0, 108.0],
                [40.0, 58.0, 110.0, 102.0],
                [16.0, 26.0, 46.0, 42.0]
            ]
        }
    );
}

#[test]
fn matrixes_multiplied_correctly_by_vector() {
    assert_eq!(
        matrix([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0]
        ]) * Vector::new(1.0, 2.0, 3.0, 1.0),
        Vector::new(18.0, 24.0, 33.0, 1.0)
    );
}

#[test]
fn matrixes_multiplied_correctly_by_identity() {
    assert_eq!(
        matrix([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0]
        ]) * Matrix::<4, 4>::identity(),
        matrix([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0]
        ])
    );
}

#[test]
fn identity_multiplied_correctly_by_vector() {
    assert_eq!(
        Matrix::<4, 4>::identity() * Vector::new(1.0, 2.0, 3.0, 1.0),
        Vector::new(1.0, 2.0, 3.0, 1.0)
    );
}

#[test]
fn correctly_transmogrifies_matrix() {
    assert_eq!(
        matrix([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0]
        ])
        .transpose(),
        matrix([
            [1.0, 5.0, 9.0, 5.0],
            [2.0, 6.0, 8.0, 4.0],
            [3.0, 7.0, 7.0, 3.0],
            [4.0, 8.0, 6.0, 2.0]
        ])
    );
}

#[test]
fn identity_transposition_return_identity() {
    assert_eq!(
        Matrix::<4, 4>::identity().transpose(),
        Matrix::<4, 4>::identity()
    );
}

#[test]
fn correctly_count_determinant_of_a_2x2_matrix() {
    assert_eq!(
        matrix([[1.0, 9.0, 5.0], [3.0, 7.0, 3.0], [4.0, 6.0, 2.0]]).submatrix(1, 1),
        matrix([[1.0, 5.0], [4.0, 2.0]])
    );
}

#[test]
fn correctly_determins_submatrix() {
    assert_eq!(matrix([[1.0, 5.0], [-3.0, 2.0]],).determinant(), 17.0);
}

#[test]
fn correctly_determins_minor() {
    assert_eq!(
        matrix([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]).minor(1, 0),
        25.0
    );
}

#[test]
fn correctly_determins_cofactor() {
    assert_eq!(
        matrix([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]).cofactor(0, 0),
        matrix([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]).minor(0, 0)
    );
    assert_eq!(
        matrix([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]).cofactor(1, 0),
        -matrix([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]).minor(1, 0)
    );
}

#[test]
fn correctly_count_determinant_of_a_3x3_matrix() {
    assert_eq!(
        matrix([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0],]).cofactor(0, 0),
        56.0
    );
    assert_eq!(
        matrix([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0],]).cofactor(0, 1),
        12.0
    );

    assert_eq!(
        matrix([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0],]).cofactor(0, 2),
        -46.0
    );

    assert_eq!(
        matrix([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0],]).determinant(),
        -196.0
    );
}

#[test]
fn correctly_count_determinant_of_a_4x4_matrix() {
    let matrix = matrix([
        [-2.0, -8.0, 3.0, 5.0],
        [-3.0, 1.0, 7.0, 3.0],
        [1.0, 2.0, -9.0, 6.0],
        [-6.0, 7.0, 7.0, -9.0],
    ]);

    assert_eq!(matrix.cofactor(0, 0), 690.0);
    assert_eq!(matrix.cofactor(0, 1), 447.0);

    assert_eq!(matrix.cofactor(0, 2), 210.0);
    assert_eq!(matrix.cofactor(0, 3), 51.0);

    assert_eq!(matrix.determinant(), -4071.0);
}

#[test]
fn determines_that_matrix_is_invertible_correctly() {
    let matrix = matrix([
        [6.0, 4.0, 4.0, 4.0],
        [5.0, 5.0, 7.0, 6.0],
        [4.0, -9.0, 3.0, -7.0],
        [9.0, 1.0, 7.0, -6.0],
    ]);

    assert_eq!(matrix.isInvertible(), true);
}

#[test]
fn determines_that_matrix_is_non_invertible_correctly() {
    let matrix = matrix([
        [-4.0, 2.0, -2.0, -3.0],
        [9.0, 6.0, 2.0, 6.0],
        [0.0, -5.0, 0.0, -5.0],
        [0.0, 0.0, 0.0, 0.0],
    ]);

    assert_eq!(matrix.isInvertible(), false);
}

// 8 | -5 | 9 | 2 |
// | 7 | 5 | 6 | 1 |
// | -6 | 0 | 9 | 6 |
// | -3 | 0 | -9 | -4 |
// Then inverse(A) is the following 4x4 matrix:
// | -0.15385 | -0.15385 | -0.28205 | -0.53846 |
// | -0.07692 | 0.12308 | 0.02564 | 0.03077 |
// | 0.35897 | 0.35897 | 0.43590 | 0.92308 |
// | -0.69231 | -0.69231 | -0.76923 | -1.92308 |

#[test]
fn correctly_invertes_matrix() {
    let mx = matrix([
        [8.0, -5.0, 9.0, 2.0],
        [7.0, 5.0, 6.0, 1.0],
        [-6.0, 0.0, 9.0, 6.0],
        [-3.0, 0.0, -9.0, -4.0],
    ])
    .inverse()
    .unwrap();

    let inverse = matrix([
        [-0.15385, -0.15385, -0.28205, -0.53846],
        [-0.07692, 0.12308, 0.02564, 0.03077],
        [0.35897, 0.35897, 0.43590, 0.92308],
        [-0.69231, -0.69231, -0.76923, -1.92308],
    ]);

    assert_eq!(mx.approx_equal(inverse), true);
}

#[test]
fn correctly_invertes_another_matrix() {
    let mx = matrix([
        [9.0, 3.0, 0.0, 9.0],
        [-5.0, -2.0, -6.0, -3.0],
        [-4.0, 9.0, 6.0, 4.0],
        [-7.0, 6.0, 6.0, 2.0],
    ])
    .inverse()
    .unwrap();

    let inverse = matrix([
        [-0.04074, -0.07778, 0.14444, -0.22222],
        [-0.07778, 0.03333, 0.36667, -0.33333],
        [-0.02901, -0.14630, -0.10926, 0.12963],
        [0.17778, 0.06667, -0.26667, 0.33333],
    ]);

    assert_eq!(mx.approx_equal(inverse), true);
}

#[test]
fn multiplies_by_the_inverse_correctly() {
    let mx = matrix([
        [3.0, -9.0, 7.0, 3.0],
        [3.0, -8.0, 2.0, -9.0],
        [-4.0, 4.0, 4.0, 1.0],
        [-6.0, 5.0, -1.0, 1.0],
    ]);

    let inverse = matrix([
        [8.0, 2.0, 2.0, 2.0],
        [3.0, -1.0, 7.0, 0.0],
        [7.0, 0.0, 5.0, 4.0],
        [6.0, -2.0, 0.0, 5.0],
    ]);

    let mult = mx * inverse;

    assert_eq!(mx.approx_equal(mult * inverse.inverse().unwrap()), true);
}

#[test]
fn correctly_translates_points() {
    let p = point(-3.0, 4.0, 5.0);
    let transform = translation(5.0, -3.0, 2.0);

    assert_eq!(transform * p, point(2.0, 1.0, 7.0));
}

#[test]
fn correctly_translates_points_in_reverse() {
    let p = point(-3.0, 4.0, 5.0);
    let transform = translation(5.0, -3.0, 2.0);

    assert_eq!(transform.inverse().unwrap() * p, point(-8.0, 7.0, 3.0));
}

#[test]
fn _translation_doesnt_change_vector() {
    let v = vector(-3.0, 4.0, 5.0);
    let transform = translation(5.0, -3.0, 2.0);

    assert_eq!(transform * v, v);
}

#[test]
fn correctly_scales_points() {
    let p = point(-4.0, 6.0, 8.0);
    let transform = scale(2.0, 3.0, 4.0);

    assert_eq!(transform * p, point(-8.0, 18.0, 32.0));
}

#[test]
fn correctly_scales_vectors() {
    let p = vector(-4.0, 6.0, 8.0);
    let transform = scale(2.0, 3.0, 4.0);

    assert_eq!(transform * p, vector(-8.0, 18.0, 32.0));
}

#[test]
fn correctly_scales_vectors_in_reverse() {
    let p = vector(-4.0, 6.0, 8.0);
    let transform = scale(2.0, 3.0, 4.0);

    assert_eq!(transform.inverse().unwrap() * p, vector(-2.0, 2.0, 2.0));
}

#[test]
fn correctly_reflects_point() {
    let p = point(-4.0, 6.0, 8.0);
    let transform = scale(-1.0, 1.0, 1.0);

    assert_eq!(transform * p, point(4.0, 6.0, 8.0));
}

#[test]
fn correctly_rotates_point_around_x() {
    let p = point(0.0, 1.0, 0.0);
    let transform = rotateX(PI / 4.0);
    let transform2 = rotateX(PI / 2.0);
    assert_eq!(
        (transform * p).appr_equal(point(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0)),
        true
    );
    assert_eq!((transform2 * p).appr_equal(point(0.0, 0.0, 1.0)), true);
}

#[test]
fn correctly_rotates_point_around_x_by_inverse() {
    let p = point(0.0, 1.0, 0.0);
    let transform = rotateX(PI / 4.0);
    assert_eq!(
        (transform.inverse().unwrap() * p).appr_equal(point(
            0.0,
            2.0_f64.sqrt() / 2.0,
            -2.0_f64.sqrt() / 2.0
        )),
        true
    );
}

#[test]
fn correctly_rotates_point_around_y() {
    let p = point(0.0, 0.0, 1.0);
    let transform = rotateY(PI / 4.0);
    let transform2 = rotateY(PI / 2.0);

    assert_eq!(
        (transform * p).appr_equal(point(2.0_f64.sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0)),
        true
    );
    assert_eq!((transform2 * p).appr_equal(point(1.0, 0.0, 0.0)), true);
}

#[test]
fn correctly_rotates_point_around_z() {
    let p = point(0.0, 1.0, 0.0);
    let transform = rotateZ(PI / 4.0);
    let transform2 = rotateZ(PI / 2.0);

    assert_eq!(
        (transform * p).appr_equal(point(-2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0)),
        true
    );
    assert_eq!((transform2 * p).appr_equal(point(-1.0, 0.0, 0.0)), true);
}

#[test]
fn correctly_uses_shearing_for_xy() {
    let p = point(2.0, 3.0, 4.0);
    let transform = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);

    assert_eq!((transform * p).appr_equal(point(5.0, 3.0, 4.0)), true);
}

#[test]
fn correctly_uses_shearing_for_xz() {
    let p = point(2.0, 3.0, 4.0);
    let transform = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);

    assert_eq!((transform * p).appr_equal(point(6.0, 3.0, 4.0)), true);
}

#[test]
fn correctly_uses_shearing_for_yx() {
    let p = point(2.0, 3.0, 4.0);
    let transform = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);

    assert_eq!((transform * p).appr_equal(point(2.0, 5.0, 4.0)), true);
}

#[test]
fn correctly_uses_shearing_for_yz() {
    let p = point(2.0, 3.0, 4.0);
    let transform = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);

    assert_eq!((transform * p).appr_equal(point(2.0, 7.0, 4.0)), true);
}

#[test]
fn correctly_uses_shearing_for_zx() {
    let p = point(2.0, 3.0, 4.0);
    let transform = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);

    assert_eq!((transform * p).appr_equal(point(2.0, 3.0, 6.0)), true);
}

#[test]
fn correctly_uses_shearing_for_zy() {
    let p = point(2.0, 3.0, 4.0);
    let transform = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);

    assert_eq!((transform * p).appr_equal(point(2.0, 3.0, 7.0)), true);
}

#[test]
fn correctly_combines_transformations() {
    let p = point(1.0, 0.0, 1.0);
    let rotate = rotateX(PI / 2.0);
    let scale = scale(5.0, 5.0, 5.0);
    let translate = translation(10.0, 5.0, 7.0);

    assert_eq!(translate * scale * rotate * p, point(15.0, 0.0, 7.0));
}

#[test]
fn correctly_calculates_rays_position() {
    let p = point(2.0, 3.0, 4.0);
    let v = vector(1.0, 0.0, 0.0);
    let ray = Ray::new(p, v);

    assert_eq!(ray.position(0.0), point(2.0, 3.0, 4.0));
    assert_eq!(ray.position(1.0), point(3.0, 3.0, 4.0));
    assert_eq!(ray.position(2.0), point(4.0, 3.0, 4.0));
    assert_eq!(ray.position(2.5), point(4.5, 3.0, 4.0));
}

#[test]
fn ray_correctly_intersects_sphere() {
    let p = point(0.0, 0.0, -5.0);
    let v = vector(0.0, 0.0, 1.0);
    let ray = Ray::new(p, v);
    let sphere = Sphere::new();

    let intersects = sphere.intersect(ray);

    assert_eq!(intersects.len(), 2);
    assert_eq!(equal_floats(intersects[0].t, 4.0), true);
    assert_eq!(equal_floats(intersects[1].t, 6.0), true);

    assert_eq!(intersects[0].object, intersects[1].object);
}

#[test]
fn ray_correctly_intersects_sphere2() {
    let p = point(0.0, 2.0, -5.0);
    let v = vector(0.0, 0.0, 1.0);
    let ray = Ray::new(p, v);
    let sphere = Sphere::new();

    let intersects = sphere.intersect(ray);

    assert_eq!(intersects.len(), 0);
}

#[test]
fn ray_correctly_intersects_sphere4() {
    let p = point(0.0, 1.0, -5.0);
    let v = vector(0.0, 0.0, 1.0);
    let ray = Ray::new(p, v);
    let sphere = Sphere::new();

    let intersects = sphere.intersect(ray);

    assert_eq!(intersects.len(), 2);
    assert_eq!(equal_floats(intersects[0].t, 5.0), true);
    assert_eq!(equal_floats(intersects[1].t, 5.0), true);
}

#[test]
fn ray_correctly_intersects_sphere5() {
    let p = point(0.0, 0.0, 0.0);
    let v = vector(0.0, 0.0, 1.0);
    let ray = Ray::new(p, v);
    let sphere = Sphere::new();

    let intersects = sphere.intersect(ray);

    assert_eq!(intersects.len(), 2);
    assert_eq!(equal_floats(intersects[0].t, -1.0), true);
    assert_eq!(equal_floats(intersects[1].t, 1.0), true);
}

#[test]
fn ray_correctly_intersects_sphere3() {
    let p = point(0.0, 0.0, 5.0);
    let v = vector(0.0, 0.0, 1.0);
    let ray = Ray::new(p, v);
    let sphere = Sphere::new();

    let intersects = sphere.intersect(ray);

    assert_eq!(intersects.len(), 2);
    assert_eq!(equal_floats(intersects[0].t, -6.0), true);
    assert_eq!(equal_floats(intersects[1].t, -4.0), true);
}

#[test]
fn correctly_calculates_hit() {
    let sphere = Object::from(Sphere::new());

    let i1 = Intersection::new(sphere, 1.0);
    let i2 = Intersection::new(sphere, 2.0);
    let i3 = Intersection::new(sphere, 5.0);
    let intersections = vec![i1, i2, i3];

    assert_eq!(hit(intersections).unwrap().t, 1.0);
}

#[test]
fn correctly_calculates_hit2() {
    let sphere = Object::from(Sphere::new());

    let i1 = Intersection::new(sphere, -1.0);
    let i2 = Intersection::new(sphere, 2.0);
    let i3 = Intersection::new(sphere, 1.0);
    let intersections = vec![i1, i2, i3];

    assert_eq!(hit(intersections).unwrap().t, 1.0);
}

#[test]
fn correctly_calculates_hit3() {
    let sphere = Object::from(Sphere::new());

    let i1 = Intersection::new(sphere, -1.0);
    let i2 = Intersection::new(sphere, -2.0);
    let i3 = Intersection::new(sphere, -5.0);
    let intersections = vec![i1, i2, i3];

    assert!(hit(intersections).is_none());
}

#[test]
fn correctly_translates_ray() {
    let r = Ray::new(point(1.0, 2.0, 3.0), vector(0.0, 1.0, 0.0));
    let m = translation(3.0, 4.0, 5.0);
    let r2 = r.transform(m);

    assert_eq!(r2.origin, point(4.0, 6.0, 8.0));
    assert_eq!(r2.direction, vector(0.0, 1.0, 0.0));
}

#[test]
fn correctly_scales_ray() {
    let r = Ray::new(point(1.0, 2.0, 3.0), vector(0.0, 1.0, 0.0));
    let m = scale(2.0, 3.0, 4.0);
    let r2 = r.transform(m);

    assert_eq!(r2.origin, point(2.0, 6.0, 12.0));
    assert_eq!(r2.direction, vector(0.0, 3.0, 0.0));
}

#[test]
fn correctly_intersects_scaled_sphere() {
    let mut sphere = Sphere::new();

    sphere.setTransform(scale(2.0, 2.0, 2.0));
    let ray = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));

    let intersection = sphere.intersect(ray);

    assert_eq!(intersection.len(), 2);
    assert_eq!(intersection[0].t, 3.0);
    assert_eq!(intersection[1].t, 7.0);
}

#[test]
fn correctly_intersects_translated_sphere() {
    let mut sphere = Sphere::new();

    sphere.setTransform(translation(5.0, 0.0, 0.0));
    let ray = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));

    let intersection = sphere.intersect(ray);

    assert_eq!(intersection.len(), 0);
}

#[test]
fn correctly_calculate_norml_for_sphere() {
    let s = Sphere::new();
    let n = s.normal_at(point(
        3.0_f64.sqrt() / 3.0,
        3.0_f64.sqrt() / 3.0,
        3.0_f64.sqrt() / 3.0,
    ));

    assert_eq!(
        n,
        vector(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0
        )
    )
}

#[test]
fn correctly_calculates_normal_for_transformed_sphere() {
    let mut s = Sphere::new();
    s.setTransform(translation(0.0, 1.0, 0.0));

    let n = s.normal_at(point(0.0, 1.70711, -0.70711));
    assert_eq!(n.appr_equal(vector(0.0, 0.70711, -0.70711)), true)
}

#[test]
fn correctly_calculates_normal_for_transformed_sphere2() {
    let mut s = Sphere::new();
    let transform = scale(1.0, 0.5, 1.0) * rotateZ(PI / 5.0);
    s.setTransform(transform);

    let n = s.normal_at(point(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0));
    assert_eq!(n.appr_equal(vector(0.0, 0.97014, -0.24254)), true)
}

#[test]
fn vector_correctly_reflects_around_normal() {
    let v = vector(1.0, -1.0, 0.0);
    let n = vector(0.0, 1.0, 0.0);
    let reflect = v.reflect(n);
    assert_eq!(reflect.appr_equal(vector(1.0, 1.0, 0.0)), true);
}

#[test]
fn vector_correctly_reflects_around_normal2() {
    let v = vector(0.0, -1.0, 0.0);
    let n = vector(2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0);
    let reflect = v.reflect(n);
    assert_eq!(reflect.appr_equal(vector(1.0, 0.0, 0.0)), true);
}

#[test]
fn correctly_calculate_lightning_at_point() {
    let m = Material::default();
    let p = point(0.0, 0.0, 0.0);

    let eyev = vector(0.0, 0.0, -1.0);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = Light::point_light(point(0.0, 0.0, -10.0), color(1.0, 1.0, 1.0));
    let result = lighting(m, light, p, eyev, normalv, false);
    assert_eq!(result, color(1.9, 1.9, 1.9));
}

#[test]
fn correctly_calculate_lightning_at_point_in_shadow() {
    let m = Material::default();
    let p = point(0.0, 0.0, 0.0);

    let eyev = vector(0.0, 0.0, -1.0);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = Light::point_light(point(0.0, 0.0, -10.0), color(1.0, 1.0, 1.0));
    let result = lighting(m, light, p, eyev, normalv, true);
    assert_eq!(result, color(0.1, 0.1, 0.1));
}

#[test]
fn correctly_calculate_lightning_at_point2() {
    let m = Material::default();
    let p = point(0.0, 0.0, 0.0);

    let eyev = vector(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = Light::point_light(point(0.0, 0.0, -10.0), color(1.0, 1.0, 1.0));
    let result = lighting(m, light, p, eyev, normalv, false);
    assert_eq!(color(1.0, 1.0, 1.0).appr_equal(result), true);
}

#[test]
fn correctly_calculate_lightning_at_point3() {
    let m = Material::default();
    let p = point(0.0, 0.0, 0.0);

    let eyev = vector(0.0, 0.0, -1.0);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = Light::point_light(point(0.0, 10.0, -10.0), color(1.0, 1.0, 1.0));
    let result = lighting(m, light, p, eyev, normalv, false);
    assert_eq!(color(0.7364, 0.7364, 0.7364).appr_equal(result), true);
}

#[test]
fn correctly_calculate_lightning_at_point4() {
    let m = Material::default();
    let p = point(0.0, 0.0, 0.0);

    let eyev = vector(0.0, -2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = Light::point_light(point(0.0, 10.0, -10.0), color(1.0, 1.0, 1.0));
    let result = lighting(m, light, p, eyev, normalv, false);
    assert_eq!(color(1.6364, 1.6364, 1.6364).appr_equal(result), true);
}

#[test]
fn correctly_calculate_lightning_at_point5() {
    let m = Material::default();
    let p = point(0.0, 0.0, 0.0);

    let eyev = vector(0.0, 0.0, -1.0);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = Light::point_light(point(0.0, 0.0, 10.0), color(1.0, 1.0, 1.0));
    let result = lighting(m, light, p, eyev, normalv, false);
    assert_eq!(color(0.1, 0.1, 0.1).appr_equal(result), true);
}

#[test]
fn corrcetly_intersects_world() {
    let w = World::default();
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let xs = w.intersect(r);

    assert_eq!(xs.len(), 4);
    assert_eq!(xs[0].t, 4.0);
    assert_eq!(xs[1].t, 4.5);
    assert_eq!(xs[2].t, 5.5);
    assert_eq!(xs[3].t, 6.0);
}

#[test]
fn correctly_precomputes_intersection() {
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let shape = Sphere::new();
    let i = Intersection::new(Object::from(shape), 4.0);
    let comps = i.compute(r);
    assert_eq!(comps.point, point(0.0, 0.0, -1.0));
    assert_eq!(comps.eye, vector(0.0, 0.0, -1.0));
    assert_eq!(comps.normal, vector(0.0, 0.0, -1.0));
}

#[test]
fn correctly_shows_that_intersection_was_on_the_outside() {
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let shape = Sphere::new();
    let i = Intersection::new(Object::from(shape), 4.0);
    let comps = i.compute(r);
    assert_eq!(comps.inside, false)
}

#[test]
fn correctly_shows_that_intersection_was_on_the_inside() {
    let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
    let shape = Sphere::new();
    let i = Intersection::new(Object::from(shape), 1.0);
    let comps = i.compute(r);
    assert_eq!(comps.inside, true);
    assert_eq!(comps.normal, vector(0.0, 0.0, -1.0));
}

#[test]
fn correctly_computes_color_at_compute() {
    let w = World::default();
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let shape = w.objects[0];
    let i = Intersection::new(Object::from(shape), 4.0);
    let comps = i.compute(r);

    let c = w.shade(comps);
    assert_eq!(color(0.38066, 0.47583, 0.2855).appr_equal(c), true);
}

#[test]
fn correctly_computes_color_at_compute2() {
    let mut w = World::default();
    w.light_source = Light::point_light(point(0.0, 0.25, 0.0), color(1.0, 1.0, 1.0));
    let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
    let shape = w.objects[1];
    let i = Intersection::new(Object::from(shape), 0.5);
    let comps = i.compute(r);

    let c = w.shade(comps);
    assert_eq!(color(0.90498, 0.90498, 0.90498).appr_equal(c), true);
}

#[test]
fn correctly_computes_color_at_point_for_the_world() {
    let w = World::default();
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 1.0, 0.0));
    let c = w.color_at(r);

    assert_eq!(color(0.0, 0.0, 0.0).appr_equal(c), true);
}

#[test]
fn correctly_computes_color_at_point_for_the_world2() {
    let w = World::default();
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let c = w.color_at(r);

    assert_eq!(color(0.38066, 0.47583, 0.2855).appr_equal(c), true);
}

// w ← default_world()
//  And outer ← the first object in w
//  And outer.material.ambient ← 1
//  And inner ← the second object in w
//  And inner.material.ambient ← 1
//  And r ← ray(point(0, 0, 0.75), vector(0, 0, -1))
//  When c ← color_at(w, r)
//  Then c = inner.material.color

#[test]
fn correctly_computes_color_at_point_for_the_world3() {
    let mut w = World::default();
    w.objects[1].set_ambient(1.0);
    w.objects[0].set_ambient(1.0);

    let inner = w.objects[1];

    let r = Ray::new(point(0.0, 0.0, 0.75), vector(0.0, 0.0, -1.0));
    let c = w.color_at(r);

    assert_eq!(inner.material().color.appr_equal(c), true);
}

#[test]
fn correctly_transform_view() {
    let from = point(0.0, 0.0, 0.0);
    let to = point(0.0, 0.0, -1.0);
    let up = vector(0.0, 1.0, 0.0);
    let t = view_transform(from, to, up);
    assert_eq!(t, Matrix::<4, 4>::identity())
}

#[test]
fn correctly_transform_view2() {
    let from = point(0.0, 0.0, 0.0);
    let to = point(0.0, 0.0, 1.0);
    let up = vector(0.0, 1.0, 0.0);
    let t = view_transform(from, to, up);
    assert_eq!(t, scale(-1.0, 1.0, -1.0))
}

#[test]
fn correctly_transform_view3() {
    let from = point(0.0, 0.0, 8.0);
    let to = point(0.0, 0.0, 0.0);
    let up = vector(0.0, 1.0, 0.0);
    let t = view_transform(from, to, up);
    assert_eq!(t, translation(0.0, 0.0, -8.0))
}

#[test]
fn correctly_transform_view4() {
    let from = point(1.0, 3.0, 2.0);
    let to = point(4.0, -2.0, 8.0);
    let up = vector(1.0, 1.0, 0.0);
    let t = view_transform(from, to, up);
    let result = Matrix {
        elements: [
            [-0.50709, 0.50709, 0.67612, -2.36643],
            [0.76772, 0.60609, 0.12122, -2.82843],
            [-0.35857, 0.59761, -0.71714, 0.00000],
            [0.00000, 0.00000, 0.00000, 1.00000],
        ],
    };

    assert!(t.approx_equal(result))
}

#[test]
fn correctly_calculates_ray_at_camera_point() {
    let c = Camera::new(201, 101, PI / 2.0);
    let r = c.ray_for_pixel(100, 50);

    assert!(r.origin.appr_equal(point(0.0, 0.0, 0.0)));
    assert!(r.direction.appr_equal(vector(0.0, 0.0, -1.0)));
}

#[test]
fn correctly_calculates_ray_at_camera_point2() {
    let c = Camera::new(201, 101, PI / 2.0);
    let r = c.ray_for_pixel(0, 0);

    assert!(r.origin.appr_equal(point(0.0, 0.0, 0.0)));
    assert!(r.direction.appr_equal(vector(0.66519, 0.33259, -0.66851)));
}

#[test]
fn correctly_calculates_ray_at_camera_point3() {
    let mut c = Camera::new(201, 101, PI / 2.0);

    c.transform(rotateY(PI / 4.0) * translation(0.0, -2.0, 5.0));
    let r = c.ray_for_pixel(100, 50);

    assert!(r.origin.appr_equal(point(0.0, 2.0, -5.0)));
    assert!(r
        .direction
        .appr_equal(vector(2.0_f64.sqrt() / 2.0, 0.0, -2.0_f64.sqrt() / 2.0)));
}

#[test]
fn camera_crrectly_renders_world() {
    let w = World::default();
    let mut c = Camera::new(11, 11, PI / 2.0);
    let from = point(0.0, 0.0, -5.0);
    let to = point(0.0, 0.0, 0.0);
    let up = vector(0.0, 1.0, 0.0);
    c.transform(view_transform(from, to, up));
    let image = c.render(w);
    assert!(image
        .pixel_at(5, 5)
        .color
        .appr_equal(color(0.38066, 0.47583, 0.2855)));
}

#[test]
fn correctly_discerns_if_point_is_in_shadow() {
    let w = World::default();
    let p = point(0.0, 10.0, 0.0);
    assert_eq!(w.is_shadowed(p), false)
}



#[test]
fn correctly_discerns_if_point_is_in_shadow2() {
    let w = World::default();
    let p = point(10.0, -10.0, 10.0);
    assert_eq!(w.is_shadowed(p), true)
}

#[test]
fn correctly_discerns_if_point_is_in_shadow3() {
    let w = World::default();
    let p = point(-20.0, 20.0, -20.0);
    assert_eq!(w.is_shadowed(p), false)
}


#[test]
fn correctly_discerns_if_point_is_in_shadow4() {
    let w = World::default();
    let p = point(-2.0, 2.0, -2.0);
    assert_eq!(w.is_shadowed(p), false)
}
