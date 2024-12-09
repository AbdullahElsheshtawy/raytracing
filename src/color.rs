use crate::{interval::Interval, vec3::Vec3};

pub fn linear_to_gamma(linear_component: f32) -> f32 {
    match linear_component > 0.0 {
        true => linear_component.sqrt(),
        false => 0.0,
    }
}

pub fn write_color(color: &Vec3) {
    // Extract and apply a linear gamma transform for gamma 2
    // let (r, g, b) = (
    //     linear_to_gamma(color.x()),
    //     linear_to_gamma(color.y()),
    //     linear_to_gamma(color.z()),
    // );

    // Translate all the [0, 1] component values to the byte range [0, 255].
    // let intensity = Interval::new(0.000, 0.999);
    // let rb = (256_f32 * intensity.clamp(r)) as i32;
    // let gb = (256_f32 * intensity.clamp(g)) as i32;
    // let bb = (256_f32 * intensity.clamp(b)) as i32;
    // println!("{} {} {}", rb, gb, bb);
}
