use clap::ValueEnum;
use rand::Rng;
use rand::{distr::StandardUniform, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rayon::prelude::*;
use union_find_rs::prelude::*;

#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum Norm {
    L1,
    LInf,
}

#[derive(Debug, Clone, Copy)]
pub struct Observables {
    /// https://arxiv.org/pdf/1610.00200
    /// S, see Equation (5)
    pub average_size: f64,
    /// Q_G, see Equation (6)
    pub size_spread: f64,
}

pub fn simulate(
    norm: Norm,
    l: usize,
    alpha: f64,
    beta: f64,
    n_samples: u64,
    seed: u64,
) -> Vec<Observables> {
    (0..n_samples)
        .into_par_iter()
        .map(|i| {
            let mut rng = ChaCha8Rng::seed_from_u64(seed);
            rng.set_stream(i);
            realize(norm, l, alpha, beta, &mut rng)
        })
        .collect()
}

struct L1;
struct LInf;

trait NormType {
    fn compute_distance(x: usize, y: usize) -> f64;
}

impl NormType for L1 {
    #[inline]
    fn compute_distance(x: usize, y: usize) -> f64 {
        // x and y are unsigned ints, we don't need to take abs
        (x + y) as f64
    }
}

impl NormType for LInf {
    #[inline]
    fn compute_distance(x: usize, y: usize) -> f64 {
        // x and y are unsigned ints, we don't need to take abs
        x.max(y) as f64
    }
}

/// Return the number of failures before the first success,
/// for a Bernoulli(p) RV
///    
/// p >= 1 => skip=0 (always success).
/// p <= epsilon => skip=large (never success).
/// Otherwise uses log-based approach for a geometric distribution.
fn geometric_skip<R: Rng + ?Sized>(p: f64, rng: &mut R) -> usize {
    match p {
        // TODO: These checks should be done outside, when computing p
        p if p >= 1.0 => 0,
        p if p <= 1E-16 => usize::MAX,
        _ => {
            let u: f64 = rng.sample(StandardUniform);
            (u.log2() / (1.0 - p).log2()) as usize
        }
    }
}

type Clusters = DisjointSets<usize>;

/// 2D long-range percolation with skip-based sampling.
/// Probability p_l = min(1, beta / l^(2 + alpha)).
fn lr_percolation_2d<N: NormType, R: Rng + ?Sized>(
    l: usize,
    alpha: f64,
    beta: f64,
    rng: &mut R,
) -> Clusters {
    let mut clusters = Clusters::with_capacity(l * l);
    for i in 0..l * l {
        clusters.make_set(i).unwrap();
    }
    for dx in 0..l {
        for dy in 0..l {
            if dx == 0 && dy == 0 {
                continue;
            }
            let periodic_dx = dx.min(l - dx);
            let periodic_dy = dy.min(l - dy);
            let distance = N::compute_distance(periodic_dx, periodic_dy);

            if distance < 1E-16 {
                // TODO: is this correct?
                // Would not (d << 1) => (p = 1) => geometric_skip = 0 => one big cluster?
                // I mean we don't want that, but why are we allowed to circumvent it?
                continue;
            }
            let p = beta / distance.powf(2.0 + alpha);

            let mut i: usize = 0;
            while i < l * l {
                let step = geometric_skip(p, rng);
                i = i.saturating_add(step);
                if i >= l * l {
                    break;
                }

                let (x1, y1) = (i / l, i % l);
                let x2 = (x1 + dx) % l;
                let y2 = (y1 + dy) % l;
                clusters
                    .union(
                        &clusters.find_set(&i).unwrap(),
                        &clusters.find_set(&(x2 * l + y2)).unwrap(),
                    )
                    .unwrap();

                i = i.saturating_add(1);
            }
        }
    }
    clusters
}

impl Observables {
    fn new(l: usize, clusters: Clusters) -> Self {
        // To prevent overflow, f64 instead of an int
        let mut sum_power2: f64 = 0.0;
        let mut sum_power4: f64 = 0.0;
        for size in clusters.into_iter().map(|c| c.len() as f64) {
            sum_power2 += size.powi(2);
            sum_power4 += size.powi(4);
        }

        Observables {
            average_size: sum_power2 / (l * l) as f64,
            size_spread: sum_power4 / sum_power2.powi(2),
        }
    }
}

fn realize<R: Rng + ?Sized>(
    norm: Norm,
    l: usize,
    alpha: f64,
    beta: f64,
    rng: &mut R,
) -> Observables {
    let clusters = match norm {
        Norm::L1 => lr_percolation_2d::<L1, _>(l, alpha, beta, rng),
        Norm::LInf => lr_percolation_2d::<LInf, _>(l, alpha, beta, rng),
    };
    Observables::new(l, clusters)
}
