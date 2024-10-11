use num_complex::{Complex, ComplexFloat};
use rand::{thread_rng, Rng};

#[test]
fn compare_scalar_to_simd() {
    let mut rng = thread_rng();
    let v: Vec<Complex<f32>> = (0..1280).map(|_| rng.gen()).collect();

    let simd_sum = v.iter().sum::<Complex<f32>>();
    let scalar_sum = v.iter().fold(Complex::new(0.0, 0.0), |acc, x| acc + x);

    assert!(dbg!((simd_sum - scalar_sum).abs()) < 0.01);
}