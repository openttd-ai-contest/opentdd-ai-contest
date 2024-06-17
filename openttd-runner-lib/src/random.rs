use rand::{Rng, thread_rng};

pub fn random_password() -> String {
    let mut rng = thread_rng();
    return (0..256).map(|_| char::from(rng.gen_range(65..91))).collect();
}