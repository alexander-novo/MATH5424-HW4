use indicatif::ProgressIterator;
use nalgebra::{DMatrix, LU};
use rand_distr::StandardNormal;
use std::{
    error::Error,
    fs::{create_dir_all, File},
    io::Write,
};

fn main() -> Result<(), Box<dyn Error>> {
    create_dir_all("./out/")?;
    let f = File::create("out/output7.txt")?;
    for n in ceil_logspace(1., 3., 1000).progress() {
        // A= randn(n, n)/sqrt(n)
        let a = DMatrix::<f64>::from_distribution(n, n, &StandardNormal, &mut rand::thread_rng())
            / (n as f64).sqrt();

        // $\(\|A\|_{max} = \max_{i,j} |a_{ij}|\)$
        let a_max = a.amax();
        let u_max = LU::new(a).u().amax();

        // $\(\rho_{pp} = \sfrac{\|U\|_{max}}{\|A\|_{max}}\)$
        let rho_pp = u_max / a_max;

        writeln!(&f, "{n} {rho_pp}")?;
    }

    Ok(()) // :)
}

/// Recreates Matlab ceil(logspace(a, b, n)) generates n points between decades 10^a and 10^b.
fn ceil_logspace(a: f64, b: f64, n: usize) -> impl ExactSizeIterator<Item = usize> {
    let temp = (b - a) / (n as f64 - 1.);
    (0..n).map(move |i| 10_f64.powf((i as f64) * temp + a).ceil() as usize)
}
