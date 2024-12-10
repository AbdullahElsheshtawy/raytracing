pub fn linear_to_gamma(linear_component: f32) -> f32 {
    // match linear_component > 0.0 {
    //     true => linear_component.sqrt(),
    //     false => 0.0,
    // }

    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}
