use greek_calc::methods::*;
use greek_calc::schemes::*;
use greek_calc::*;
use statrs::distribution::Continuous;
use statrs::distribution::ContinuousCDF;
use statrs::distribution::Normal;

#[test]
fn test_call_fdm_delta() {
    let scheme = GeometricBM::new(0.05, 0.2);
    let option = Options {
        payoff: Box::new(|s| test_relu(s - 100.0)),
        asset_price: 100.0,
        maturity: 1.0,
        rate: 0.05,
    };
    let fdm = FDM::new(100000, 1000, 1e-3);

    let delta = fdm.delta(&option, &scheme);
    let d1 = ((100f64 / 100.0).ln() + 1.0 * (0.05 + 0.5 * 0.2 * 0.2)) / (0.2 * 1f64.sqrt());
    let true_delta = Normal::standard().cdf(d1);
    eprintln!("FDM delta: {delta}");
    eprintln!("True delta: {true_delta}");
    eprintln!("error rate: {}", (delta - true_delta) / true_delta);
}

#[test]
fn test_call_fdm_gamma() {
    let scheme = GeometricBM::new(0.05, 0.2);
    let option = Options {
        payoff: Box::new(|s| test_relu(s - 100.0)),
        asset_price: 100.0,
        maturity: 1.0,
        rate: 0.05,
    };
    let fdm = FDM::new(100000, 1000, 1e-3);

    let gamma = fdm.gamma(&option, &scheme);
    let d1 = ((100f64 / 100.0).ln() + 1.0 * (0.05 + 0.5 * 0.2 * 0.2)) / (0.2 * 1f64.sqrt());
    let true_gamma = Normal::standard().pdf(d1) / (0.2 * 100.0 * 1f64.sqrt());
    eprintln!("FDM gamma: {gamma}");
    eprintln!("True gamma: {true_gamma}");
    eprintln!("error rate: {}", (gamma - true_gamma) / true_gamma);
}

fn test_relu(v: f64) -> f64 {
    if v < 0.0 {
        0.0
    } else {
        v
    }
}
