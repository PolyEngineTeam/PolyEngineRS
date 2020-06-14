use na::{Scalar, Vector2, Vector3};

pub fn vec2_to_vec3<T: Scalar + Copy>(v2: Vector2<T>, z: T) -> Vector3<T> {
    return Vector3::new(v2.x, v2.y, z);
}

pub fn vec3_to_vec2<T: Scalar + Copy>(v3: Vector3<T>) -> Vector2<T> {
    return Vector2::new(v3.x, v3.y);
}

#[cfg(test)]
mod tests {
    use super::{vec2_to_vec3, vec3_to_vec2};
    use na::{Vector2, Vector3};

    #[test]
    fn vec3_to_vec2_test() {
        assert_eq!(
            Vector2::<i32>::new(1, 2),
            vec3_to_vec2(Vector3::<i32>::new(1, 2, 3))
        );
    }

    #[test]
    fn vec2_to_vec3_test() {
        assert_eq!(
            Vector3::<i32>::new(1, 2, 3),
            vec2_to_vec3(Vector2::<i32>::new(1, 2), 3)
        );
    }
}
