use na::DMatrix;
use rand_distr::StandardNormal;

use crate::Scheme;

/// Geometric Brownian Motion
pub struct GeometricBM {
    /// Drift rate
    drift: f64,
    /// Diffuse rate
    diffuse: f64,
}

impl GeometricBM {
    pub fn new(drift: f64, diffuse: f64) -> Self {
        Self { drift, diffuse }
    }
}

impl Scheme for GeometricBM {
    fn simulate(
        &self,
        asset_price: f64,
        maturity: f64,
        num_paths: usize,
        num_steps: usize,
    ) -> DMatrix<f64> {
        let time_step = maturity / (num_steps + 1) as f64;
        let dw = DMatrix::from_distribution(
            num_paths,
            num_steps,
            &StandardNormal,
            &mut rand::thread_rng(),
        );
        let mut paths = DMatrix::zeros(num_paths, num_steps + 1);

        // Explicit Euler
        // TODO: Other schemes like semi-implicit Euler
        paths.column_mut(0).fill(asset_price);
        for col in 0..num_steps {
            let column = paths
                .column(col)
                .map(f64::ln)
                .add_scalar((self.drift - 0.5 * self.diffuse * self.diffuse) * time_step)
                + self.diffuse * time_step.sqrt() * dw.column(col);
            paths.set_column(col + 1, &column.map(f64::exp));
        }

        paths
    }
}
