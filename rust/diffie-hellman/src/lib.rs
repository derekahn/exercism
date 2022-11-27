use std::ops::Rem;

pub fn private_key(p: u64) -> u64 {
    p / 2
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    vec![g; a as usize]
        .into_iter()
        .fold(1, |acc, x| acc * x)
        .rem(p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    public_key(p, b_pub, a)
}
