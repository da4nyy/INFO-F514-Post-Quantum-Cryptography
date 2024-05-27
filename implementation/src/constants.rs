#![allow(unused)]

pub struct MayoParams<'a> {
    pub m: usize,
    pub n: usize,
    pub o: usize,
    pub k: usize,
    pub q: usize,
    pub f_tail: &'a [u8], 
    pub O_bytes: usize,
    pub v_bytes: usize,
    pub P1_bytes: usize,
    pub P2_bytes: usize,
    pub P3_bytes: usize,
    pub L_bytes: usize,
    pub csk_bytes: usize,
    pub esk_bytes: usize,
    pub cpk_bytes: usize,
    pub epk_bytes: usize,
    pub sig_bytes: usize,
    pub salt_bytes: usize,
    pub digest_bytes: usize,
    pub pk_seed_bytes: usize,
    pub sk_seed_bytes: usize,
}

impl MayoParams<'_> {
    pub fn param_v(&self) -> usize {self.n - self.o}
    pub fn param_a_cols(&self) -> usize {self.k * self.o + 1}
}