extern crate nalgebra as na;

use na::DMatrix;

pub mod methods;
pub mod schemes;

/// The payoff function of Options
pub type Payoff = Box<dyn Fn(f64) -> f64>;

/// The basic structure of an option
pub struct Options {
    /// The payoff function
    pub payoff: Payoff,
    /// Spot price of underlying asset
    pub asset_price: f64,
    /// The maturity from current time
    pub maturity: f64,
    /// The interest rate
    pub rate: f64,
}

/// A trait for a model of underlying asset price
pub trait Scheme {
    /// Perform Monte Carlo simulation on the scheme
    /// Return all simulated paths (`num_paths`x`num_steps+1`)
    /// where `num_steps` is the number of all simulated timesteps
    fn simulate(
        &self,
        asset_price: f64,
        maturity: f64,
        num_paths: usize,
        num_steps: usize,
    ) -> DMatrix<f64>;
}

/// A trait signifying an greek-implemented method
pub trait Greeks {
    /// Calculating Delta: first order derivating against underlying asset price
    fn delta(&self, option: &Options, scheme: &impl Scheme) -> f64;

    /// Calculating Gamma: second order derivating against underlying asset price
    fn gamma(&self, option: &Options, scheme: &impl Scheme) -> f64;
}
