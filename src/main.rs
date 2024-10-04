use ark_bls12_381::{Bls12_381, G1Projective as G1, G2Projective as G2, Fr as ScalarField};
use ark_std::UniformRand;

mod setup;
mod prover;
mod verifier;


fn main() {
    let d = 15;
    let mut rng = rand::rngs::OsRng;
    
    // Setup
    let mut kzg_instance = setup::KZG::<Bls12_381>::new(
        G1::rand(&mut rng),
        G2::rand(&mut rng),
        d
    );
    let tau = ScalarField::rand(&mut rng);
    let (pp_g1, pp_g2, degree) = kzg_instance.setup(tau); 
    
    // Prover setup
    let mut prover = prover::Prover::<Bls12_381>::new(
        pp_g1.clone(), pp_g2.clone(), degree
    );
    let commit : G2 = prover.polygen();
    
    // Verifier setup
    let mut verifier = verifier::Verifier::<Bls12_381>::new(
        pp_g1.clone(), pp_g2.clone(), degree
    );
    
    for i in 0..10{
        let x : ScalarField = verifier.gen_chal();

        // Proof generation
        let (pt, h_tau) : (ScalarField, G2) = prover.challenge(x);

        // Verification
        println!{"{}th result for challenge: {}", i, x};
        verifier.verify(commit, pt, h_tau);
    }
}
