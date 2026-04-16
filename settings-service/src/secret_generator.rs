use rand::Rng;

pub fn generate(len: usize) -> String {
    let mut rng = rand::rng();
    let mut s = String::new();
    for _ in 0..len {
        let c = rng.random_range(0..3);
        match c {
            0 => s.push(rng.random_range('0'..='9')),
            1 => s.push(rng.random_range('a'..='z')),
            2 => s.push(rng.random_range('A'..='Z')),
            _ => unreachable!(),
        }
    }
    s
}
