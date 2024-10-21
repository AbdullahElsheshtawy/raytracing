use rand::Rng;

pub fn rand_f32() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen::<f32>()
}

pub fn rand(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}
