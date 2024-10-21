use crate::Greeks;
use crate::Options;
use crate::Payoff;
use crate::Scheme;

/// A struct for the parameters of the Finite Differences Method (FDM)
pub struct FDM {
    /// The number of Monte Carlo simulated paths
    num_paths: usize,
    /// The number of Monte Carlo simulated time steps
    num_steps: usize,
    /// The small step of parameter
    step: f64,
}

impl FDM {
    pub fn new(num_paths: usize, num_steps: usize, step: f64) -> Self {
        Self {
            num_paths,
            num_steps,
            step,
        }
    }
}

impl Greeks for FDM {
    fn delta(&self, option: &Options, scheme: &impl Scheme) -> f64 {
        let start_lower = option.asset_price - self.step;
        let start_upper = option.asset_price + self.step;

        let payoff_lower = mean_payoff(
            &option.payoff,
            scheme,
            start_lower,
            option.maturity,
            self.num_paths,
            self.num_steps,
        );
        let payoff_upper = mean_payoff(
            &option.payoff,
            scheme,
            start_upper,
            option.maturity,
            self.num_paths,
            self.num_steps,
        );

        let discount = (-option.rate * option.maturity).exp();
        eprintln!("discount: {discount}");
        eprintln!("payoff upper: {payoff_upper}");
        eprintln!("payoff lower: {payoff_lower}");
        discount * (payoff_upper - payoff_lower) / (2.0 * self.step)
    }

    fn gamma(&self, option: &Options, scheme: &impl Scheme) -> f64 {
        let start_lower = option.asset_price - self.step;
        let start_middle = option.asset_price;
        let start_upper = option.asset_price + self.step;

        let payoff_lower = mean_payoff(
            &option.payoff,
            scheme,
            start_lower,
            option.maturity,
            self.num_paths,
            self.num_steps,
        );
        let payoff_middle = mean_payoff(
            &option.payoff,
            scheme,
            start_middle,
            option.maturity,
            self.num_paths,
            self.num_steps,
        );
        let payoff_upper = mean_payoff(
            &option.payoff,
            scheme,
            start_upper,
            option.maturity,
            self.num_paths,
            self.num_steps,
        );

        let discount = (-option.rate * option.maturity).exp();
        eprintln!("discount: {discount}");
        eprintln!("payoff upper: {payoff_upper}");
        eprintln!("payoff middle: {payoff_middle}");
        eprintln!("payoff lower: {payoff_lower}");
        discount * (payoff_upper - 2.0 * payoff_middle + payoff_lower) / (self.step * self.step)
    }
}

fn mean_payoff(
    payoff: &Payoff,
    scheme: &impl Scheme,
    start: f64,
    maturity: f64,
    num_paths: usize,
    num_steps: usize,
) -> f64 {
    let paths = scheme.simulate(start, maturity, num_paths, num_steps);
    let num_obs = paths.shape().1;
    paths.column(num_obs - 1).map(payoff).mean()
}
