pub mod custom_vector {
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

    #[derive(Debug)]
    pub struct Vector3D<T> {
        pub x: T,
        pub y: T,
        pub z: T,
    }

    impl<T> Vector3D<T> {
        pub fn new(x: T, y: T, z: T) -> Self {
            Self { x: x, y: y, z: z }
        }
    }

    impl<T: Add<Output = T>> Add for Vector3D<T> {
        type Output = Self;

        fn add(self, other: Self) -> Self::Output {
            Self {
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z,
            }
        }
    }

    impl<T: Sub<Output = T>> Sub for Vector3D<T> {
        type Output = Self;

        fn sub(self, other: Self) -> Self::Output {
            Self {
                x: self.x - other.x,
                y: self.y - other.y,
                z: self.z - other.z,
            }
        }
    }
}
