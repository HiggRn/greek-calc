# greek-calc

A simple Rust package for calculating option Greeks.

Currently still under development.

Now only aims for single underlying asset, only delta and gamma, only European options.

Striving to implement FDM.

## Goals

- Features:
  - [ ] Multiple assets
  - [ ] Other Greeks: Vega, etc.
  - [ ] VIX & RV options
  - [ ] American options
  - [ ] Exotic options
- Schemes:
  - [ ] Geometric Brownian Motion
  - [ ] Heston model
  - [ ] SABR model
  - [ ] General Diffusion Process
- Methods:
  - [ ] Finite Differences Method
  - [ ] Pathwise Estimation Method
  - [ ] Likelihood Ratio Method
  - [ ] Malliavin Calculus Method
