//! Helper math functions

/// Finds the roots of a quadratic formula
///
/// Roots = (-b +- sqrt(b^2 - 4ac)) / 2a
pub fn quadratic(a: f64, b: f64, c: f64) -> (f64, f64) {
    assert!(a != 0.0, "No valid solutions for quadratic equation");
    let delta = b * b - 4.0 * a * c;
    assert!(
        delta >= 0.0,
        "No real solutions exist for quadratic equation"
    );
    let root1 = (-b - delta.sqrt()) / (2.0 * a);
    let root2 = (-b + delta.sqrt()) / (2.0 * a);
    (root1, root2)
}
