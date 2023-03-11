use ark_bw6_761::BW6_761;
use ark_ec::{pairing::Pairing, CurveGroup};
use ark_std::UniformRand;
fn pairing() {
    type G1 = <BW6_761 as Pairing>::G1;
    type G2 = <BW6_761 as Pairing>::G2;

    let mut rng = ark_std::test_rng();

    let g1 = G1::rand(&mut rng).into_affine();
    let g2 = G2::rand(&mut rng).into_affine();

    let out = <BW6_761 as Pairing>::multi_pairing([g1], [g2]);
    println!("out = {:?}", out);
}
fn main() {
    pairing();
}
