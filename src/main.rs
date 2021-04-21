mod vectors;

use vectors::custom_vector::Vector3D;

fn main() {
    let new_vector = Vector3D::new(20, 30, 50);

    println!("Hello, world!, {:?}", new_vector);
}
