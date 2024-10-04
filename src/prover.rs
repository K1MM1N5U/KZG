use ark_ec::{pairing::Pairing};
use ark_ff::{Field};
// use ark_bls12_381::{Bls12_381, G1Projective as G1, G2Projective as G2, Fr as ScalarField};
use ark_std::{UniformRand, ops::Mul};

pub struct Prover<E: Pairing> {
    pub pp_g1 : Vec<E::G1>,
    pub pp_g2 : Vec<E::G2>,
    pub degree : usize,
    pub poly : Vec<E::ScalarField>,
    pub poly_commit : E::G2,     

}

impl <E: Pairing> Prover<E> {
    pub fn new(pp_g1 : Vec<E::G1>, pp_g2 : Vec<E::G2>, degree : usize)-> Self{
        let mut rng = ark_std::test_rng();
        // let mut rng = rand::rngs::OsRng;
        let g : E::G2 = E::G2::rand(&mut rng);
        Self{
            pp_g1,
            pp_g2,
            degree,
            poly : vec![], 
            poly_commit : g.mul(E::ScalarField::ZERO),
        }

    }
    pub fn polygen(&mut self) -> E::G2{
        // let mut rng = ark_std::test_rng();
        let mut rng = rand::rngs::OsRng;
        for i in 0..self.degree+1 {
            let num = E::ScalarField::rand(&mut rng);
            self.poly.push(num);
            self.poly_commit = self.poly_commit + self.pp_g2[i].mul(num);
        }
        return self.poly_commit;
    }

    pub fn challenge(&self, a: E::ScalarField) -> (E::ScalarField, E::G2){
        let mut b : E::ScalarField = E::ScalarField::ZERO;
        for i in 0..self.degree+1{
            b = b + self.poly[i] * a.pow(&[i as u64]);
        }

        let mut h : Vec<E::ScalarField> = vec![];
        let a_inv : E::ScalarField = (a).inverse().unwrap();
        h.push((b-self.poly[0])* a_inv);
        for i in 1..self.degree {
            let tmp =  h[i-1]- self.poly[i];
            h.push(tmp * a_inv);
            
        }
        // println!("Last coeff: {}", h[self.degree-1] == self.poly[self.degree]);
        let mut h_tau : E::G2 = self.pp_g2[0].mul(h[0]);
        for i in 1..self.degree {
            h_tau = h_tau + self.pp_g2[i].mul(h[i]);
        }
        return (b, h_tau);
    }
    
}
