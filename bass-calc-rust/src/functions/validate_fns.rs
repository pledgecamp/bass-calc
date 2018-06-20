//! Equations from "Complete Response Function and System Parameters for a Loudspeaker with Passive Radiator"
//! by Douglas H. Hurlburt

use parameters::Parameters;
use functions::graph_fns::BassFnData;

use uom::si::time::second;
use uom::si::f64::Time;

#[allow(dead_code)]
fn Ts(params: &Parameters) -> Time {
    Time::new::<second>(params.Ts.v())
}

#[allow(dead_code)]
fn Tp(params: &Parameters) -> Time {
    Time::new::<second>(params.Tp.v())
}

/// Another formulation of 
#[allow(dead_code)]
pub fn ValidateRadiatorTest(params: &Parameters) -> BassFnData {
    let g =  0.2; // τb / Ts 0.2 is a good guesstimate
    let α = params.α.v();
    let δ = params.δ.v();
    let psi = α + δ + 1.0;
    let psi25 = psi.powf(0.25);
    let y = params.Ts.v() / params.Tp.v();//params.y.v();
    let y2 = y.sqrt();
    let Qmp = params.Qmp.v();
    let Qs = params.Qs.v();

    let a1 = (y2 / psi25) * 
        ((1.0 / Qmp) + (1.0 / (y * Qs)) + (g * ((α / y) + (y * δ))));

    let a2 = (1.0 / psi.sqrt()) * (((α + 1.0) / y) +
                            (y * (δ + 1.0)) +
                            (1.0 / (Qmp * Qs)) +
                            (g * ((α / Qmp) +
                            (y * (δ / Qs)))));

    let a3 = (y2 / psi.powf(0.75)) *
        (((δ + 1.0) / Qs) + ((α + 1.0) / (y * Qmp)) + (g * (α + δ)));
    
    let b1 = y2 / (Qmp * psi25);
    let b2 = y / psi.sqrt();

    let T0 = params.Ts.v() / (y2 * psi25); // 8a
    println!("a: {}, d: {}", α, δ);
    let coef4 = T0.powi(4);

    BassFnData {
        num: vec![coef4, b1*T0.powi(3), b2*T0.powi(2), 0., 0.],
        den: vec![coef4, a1*T0.powi(3), a2*T0.powi(2), a3*T0, 1.0]
    }
}

#[allow(dead_code)]
fn ValidateRadiator(params: &Parameters) -> BassFnData {
    let g =  0.2; // τb / Ts 0.2 is a good guesstimate
    let α = params.α.v();
    let δ = params.δ.v();
    let psi = α + δ + 1.0;
    let Ts = Ts(params);
    let Ts2 = Ts * Ts;
    let Tp = Tp(params);
    let Tp2 = Tp * Tp;
    let Qmp = params.Qmp.v();
    let Qs = params.Qs.v();

    let b4 = Ts2 * Tp2;
    let b3 = Ts2 * (Tp / Qmp);
    let b2 = Ts2;

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
        num: vec![b4.value, b3.value, b2.value, 0., 0.],
        den: vec![a4.value, a3.value, a2.value, a1.value, psi]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use functions::graph_fns::*;
    use parameters::builtin_defaults;

    fn bass_data_cmp(a: &BassFnData, b: &BassFnData) -> bool {
        return a.num.iter().eq(b.num.iter()) &&
            a.den.iter().eq(b.den.iter())
    }

    #[test]
    fn radiator_alt() {
        let params = builtin_defaults();
        let d1 = ValidateRadiator(&params);
        let d2 = Radiator(&params);
        let d3 = ValidateRadiatorTest(&params);
        println!("{:?}\n{:?}\n", d1.num, d3.num);
        println!("{:?}\n{:?}", d1.den, d3.den);
        assert!(bass_data_cmp(&d1, &d2));
        assert!(bass_data_cmp(&d1, &d3));
    }
}