use ark_ec::{pairing::Pairing};
use ark_ff::{Field};
// use ark_bls12_381::{Bls12_381, G1Projective as G1, G2Projective as G2, Fr as ScalarField};
use ark_std::{ops::Mul};

pub struct KZG<E: Pairing> {
    pub g1 : E::G1,
    pub g2 : E::G2,
    pub g2_tau : E::G2,
    pub degree : usize,
    pub pp_g1 : Vec<E::G1>,
    pub pp_g2 : Vec<E::G2>,
}

impl <E:Pairing> KZG<E> {
    pub fn new(g1 : E::G1, g2: E::G2, degree: usize) -> Self{
        Self {
            g1,
            g2,
            g2_tau: g2.mul(E::ScalarField::ZERO),
            degree,
            pp_g1 : vec![],
            pp_g2 : vec![],
        }
    }

    pub fn setup(&mut self, secret : E::ScalarField) -> (Vec<E::G1>, Vec<E::G2>, usize) {
        for i in 0..self.degree+1 {
            self.pp_g1.push(self.g1.mul(secret.pow(&[i as u64])));
            self.pp_g2.push(self.g2.mul(secret.pow(&[i as u64])));
        }
        self.g2_tau = self.g2.mul(secret);
        
        return (self.pp_g1.clone(), self.pp_g2.clone(), self.degree);

    }
}
