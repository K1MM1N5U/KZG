use ark_ec::{pairing::Pairing};
use ark_ff::{Field};
// use ark_bls12_381::{Bls12_381, G1Projective as G1, G2Projective as G2, Fr as ScalarField};
use ark_std::{UniformRand, ops::Mul};


pub struct Verifier<E: Pairing>{
    pub pp_g1 : Vec<E::G1>,
    pub pp_g2 : Vec<E::G2>,
    pub degree : usize,
    pub chal : E::ScalarField,
}

impl <E: Pairing> Verifier<E>{
    pub fn new(pp_g1 : Vec<E::G1>, pp_g2 : Vec<E::G2>, degree : usize)-> Self{
        Self{
            pp_g1,
            pp_g2,
            degree,
            chal : E::ScalarField::ZERO,
        }
    }

    pub fn gen_chal(&mut self) -> E::ScalarField{
        // let mut rng = ark_std::test_rng();
        let mut rng = rand::rngs::OsRng;
        self.chal = E::ScalarField::rand(&mut rng);
        return self.chal;
        }

    pub fn verify(&self, commit : E::G2, point : E::ScalarField, h_tau : E::G2){
        let e1  = E::pairing(self.pp_g1[0], commit - self.pp_g2[0].mul(point));
        let e2  = E::pairing(self.pp_g1[1] - self.pp_g1[0].mul(self.chal), h_tau);
        if e1 == e2 {
            println!("Accept");
        } else {
            println!("Reject");
        }
        
    }
}

