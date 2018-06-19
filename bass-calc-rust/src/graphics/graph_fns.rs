//! Equations from "Complete Response Function and System Parameters for a Loudspeaker with Passive Radiator"
//! by Douglas H. Hurlburt

use parameters::Parameters;
use num_complex::Complex64;
type C64 = Complex64;

pub struct BassFnData {
    num: Vec<f64>,
    den: Vec<f64>
}

fn poly_calc(x: f64, scale: f64, exp: f64) -> C64 {
    let i = C64::new(0.0, x);
    scale * i.powf(exp)
}

// Calculate a single point on the graph represented by `data` at frequency `w`
pub fn bass_fn_point(data: &BassFnData, w: f64) -> f64 {
    let num: C64 = data.num.iter().enumerate().fold(C64::new(0.0, 0.0), |s, (i, v)| s + poly_calc(v.clone(), w, i.clone() as f64));
    let den: C64 = data.den.iter().enumerate().fold(C64::new(0.0, 0.0), |s, (i, v)| s + poly_calc(v.clone(), w, i.clone() as f64));
    let n = num / den;
    n.norm_sqr().sqrt()
}

pub fn Radiator(params: &Parameters) -> BassFnData {
    let g =  0.2; // τb / Ts 0.2 is a good guesstimate
    let g25 = 0.66874; // g ^ 0.25
    let α = params.α.v();
    let δ = params.δ.v();
    let psi = α + params.δ.v() + 1.0;
    let y = params.y.v();
    let y2 = y.sqrt();
    let Qmp = params.Qmp.v();
    let Qs = params.Qs.v();

    let T0 = params.Ts.v() / (y2 * g25); // 8a
    let a1 = (y2 / g25) * 
        ((1.0 / Qmp) + (1.0 / (y * Qs)) + (g * ((α / y) + (y * δ))));

    let a2 = (1.0 / psi.sqrt()) * (((α + 1.0) / y) +
                            (y * (δ + 1.0)) +
                            (1.0 / (Qmp * Qs)) +
                            (g *((α / Qmp) +
                            (y * (δ / Qs)))));

    let a3 = (y2 / psi.powf(0.75)) *
        (((δ + 1.0) / Qs) + ((α + 1.0) / (y * Qmp)) + (g * (α + δ)));
    
    
    
    let b1 = y2 / (Qmp * g25);
    let b2 = y / psi.sqrt();

    let num = [1.0, b1, b2, 0., 0. ];
    let den = [1.0, a1, a2, a3, 1.0];

    //sys = signal.TransferFunction(num, den)
    //print(sys)
    //signal.bode(sys)
    // println!("{} {} {}", a1, a2, a3);
    BassFnData {
        num: vec![1.0, b1, b2, 0., 0.],
        den: vec![1.0, a1, a2, a3, 1.0]
    }
}