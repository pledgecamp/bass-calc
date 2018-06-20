//! Equations from "Complete Response Function and System Parameters for a Loudspeaker with Passive Radiator"
//! by Douglas H. Hurlburt

use std::f64::consts::PI;
use parameters::Parameters;
use num_complex::Complex64;
type C64 = Complex64;

pub struct BassFnData {
    pub num: Vec<f64>,
    pub den: Vec<f64>
}

fn poly_calc(vec: &Vec<f64>, w: f64) -> C64 {
    vec.iter().rev().enumerate().fold(C64::new(0., 0.), |sum, (index, scale)| {
        let i = C64::new(0.0, w.clone());
        sum + scale * i.powf(index as f64)
    })
}

// Calculate a single point on the graph represented by `data` at frequency `w`
pub fn bass_fn_point(data: &BassFnData, w: f64) -> f64 {
    let num = poly_calc(&data.num, w);
    let den = poly_calc(&data.den, w);
    let n = num / den;
    n.norm_sqr().sqrt()
}

#[allow(dead_code)]
pub fn Radiator(params: &Parameters) -> BassFnData {
    let g =  0.2; // τb / Ts 0.2 is a good guesstimate
    let α = params.α.v();
    let δ = params.δ.v();
    let psi = α + δ + 1.0;
    let Ts = params.Ts.v();
    let Ts2 = Ts.powf(2.);
    let Tp = params.Tp.v();
    let Tp2 = Tp.powf(2.);
    let Qmp = params.Qmp.v();
    let Qs = params.Qs.v();

    let b4 = Ts2 * Tp2;
    let b3 = Ts2 * (Tp / Qmp);
    let b2 = Ts2;

    let a4 = Ts2 * Tp2;

    let a3 = Ts2 * Tp / Qmp +
            Tp2 * Ts / Qs +
            (g * Ts) * (α * Tp2 + (δ * Ts2));

    let a2 = Tp2 * (α + 1.) +
            Ts2 * (δ + 1.) +
            (Ts * Tp) / (Qs * Qmp) +
            (g * Ts) * ((α * Tp / Qmp) + (δ * Ts / Qs));

    let a1 = Ts * (δ + 1.) / Qs +
            Tp * (α + 1.) / Qmp +
            (g * Ts) * (α + δ);

    BassFnData {
        num: vec![b4, b3, b2, 0., 0.],
        den: vec![a4, a3, a2, a1, psi]
    }
}

#[allow(dead_code)]
pub fn DriverDisplacement(params: &Parameters) -> BassFnData {
    let g =  0.2; // τb / Ts 0.2 is a good guesstimate
    let Ts = params.Ts.v();
    let Ts2 = Ts.powf(2.);
    let Tp = params.Tp.v();
    let Tp2 = Tp.powf(2.);
    let Tb = params.Tb.v();
    let Tb2 = Tb.powf(2.);
    let α = params.α.v();
    let δ = params.δ.v();
    let Qmp = params.Qmp.v();
    let Qs = params.Qs.v();
    let psi = α + δ + 1.0;

    let b2 = psi * Tb2;

    let a4 = Ts2 * Tp2;

    let a3 = Ts2 * Tp / Qmp +
            (g * Ts) * (α * Tp2 + (δ * Ts2));

    let a2 = Tp2 * (α + 1.) +
            Ts2 * (δ + 1.) +
            (Ts * Tp) / (Qs * Qmp) +
            (g * Ts) * ((α * Tp / Qmp) + (δ * Ts / Qs));

    let a1 = Ts * (δ + 1.) / Qs +
            Tp * (α + 1.) / Qmp +
            (g * Ts) * (α + δ);

    BassFnData {
        num: vec![0., 0., b2, 0., psi],
        den: vec![a4, a3, a2, a1, psi]
    }
}

#[allow(dead_code)]
pub fn PassiveDisplacement(params: &Parameters) -> BassFnData {
    let g =  0.2; // τb / Ts 0.2 is a good guesstimate
    let α = params.α.v();
    let δ = params.δ.v();
    let psi = α + δ + 1.0;
    let Qmp = params.Qmp.v();
    let Tp = params.Tp.v();
    let Tp2 = Tp.powf(2.);
    let Ts = params.Ts.v();
    let Ts2 = Ts.powf(2.);
    let Qs = params.Qs.v();

    let a4 = Ts2 * Tp2;

    let a3 = Ts2 * Tp / Qmp +
            (g * Ts) * (α * Tp2 + (δ * Ts2));

    let a2 = Tp2 * (α + 1.) +
            Ts2 * (δ + 1.) +
            (Ts * Tp) / (Qs * Qmp) +
            (g * Ts) * ((α * Tp / Qmp) + (δ * Ts / Qs));

    let a1 = Ts * (δ + 1.) / Qs +
            Tp * (α + 1.) / Qmp +
            (g * Ts) * (α + δ);

    BassFnData {
        num: vec![0., 0., 0., 0., psi],
        den: vec![a4, a3, a2, a1, psi]
    }
}

#[allow(dead_code)]
pub fn Impedance(params: &Parameters) -> BassFnData {
    let g =  0.2; // τb / Ts 0.2 is a good guesstimate
    let α = params.α.v();
    let δ = params.δ.v();
    let psi = α + δ + 1.0;
    let Qs = params.Qs.v();
    let Qms = params.Qms.v();
    let Qes = params.Qes.v();
    let Qmp = params.Qmp.v();
    let Tb = params.Tb.v();
    let Tb2 = Tb.powf(2.);
    let Tp = params.Tp.v();
    let Tp2 = Tp.powf(2.);
    let Ts = params.Ts.v();
    let Ts2 = Ts.powf(2.);
    
    let b3 = psi * Tb2 * Ts / Qes;
    
    let b1 = psi * Ts / Qes;

    let a4 = Ts2 * Tp2;
    let a3 = Ts2 * Tp / Qmp +
            (g * Ts) * (α * Tp2 + (δ * Ts2));

    let a2 = Tp2 * (α + 1.) +
            Ts2 * (δ + 1.) +
            (Ts * Tp) / (Qs * Qmp) +
            (g * Ts) * ((α * Tp / Qmp) + (δ * Ts / Qms));

    let a1 = Ts * (δ + 1.) / Qms +
            Tp * (α + 1.) / Qmp +
            (g * Ts) * (α + δ);

    BassFnData {
        num: vec![0., b3, 0., b1, psi],
        den: vec![a4, a3, a2, a1, psi]
    }
}

/*
pub fn EfficencySmalls(params: &Parameters) -> f64 {
    let Qts = params.Qts.v();
    let Qes = params.Qes.v();
    let c = params.c.v();
    let Vb = params.Vb.v();
    let Vas = params.Vas.v();
    let Fs = params.Fs.v();

    let kηq = Qts / Qes;

    let kηg = 4 * (PI.powi(2)) / c.powi(3) * (Vas / Vb) * (Fs.powi(3) / (f3 ** 3.)) * (1. / Qts);
    let kη = kηq * kηg;

    let η0 = kη * (f3 ** 3) * Vb;
    η0
}
*/

#[allow(dead_code)]
pub fn EfficiencyAdams(params: &Parameters) -> f64 {
    let c = params.c.v();
    let ρ0 = params.ρ0.v();
    let Re = params.Re.v();
    let Sd = params.Sd.v();
    let Bl = params.Bl.v();
    let Mas = params.Mas.v();

    let η0 = Bl.powi(2) * ρ0 / (Sd.powi(2) * Mas.powi(2) * 2. * PI * c * Re);
    η0
}
