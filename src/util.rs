// util.rs

use bevy::math::{Vec2, Vec3};

/// Converts a `Vec3` to `Vec2` by dropping the z element.
///
/// This function is useful when you need to operate on 2D vectors,
/// but your data is in 3D space (common in 2D games with 3D coordinates).
///
/// # Arguments
///
/// * `vec3`: The `Vec3` to be converted into `Vec2`.
///
/// # Examples
///
/// ```
/// use bevy::math::Vec3;
/// use my_crate::util::convert_vec3_to_vec2; // Replace `my_crate` with your actual crate name
///
/// let vec3 = Vec3::new(1.0, 2.0, 3.0);
/// let vec2 = convert_vec3_to_vec2(vec3);
/// assert_eq!(vec2, Vec2::new(1.0, 2.0));
/// ```
pub fn convert_vec3_to_vec2(vec3: Vec3) -> Vec2 {
    Vec2::new(vec3.x, vec3.y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_vec3_to_vec2() {
        let vec3 = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = convert_vec3_to_vec2(vec3);
        assert_eq!(vec2, Vec2::new(1.0, 2.0));
    }
}
