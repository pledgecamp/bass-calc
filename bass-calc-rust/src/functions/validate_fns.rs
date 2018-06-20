//! Equations from "Complete Response Function and System Parameters for a Loudspeaker with Passive Radiator"
//! by Douglas H. Hurlburt

use parameters::Parameters;
use functions::graph_fns::BassFnData;

use uom::si::time::second;
use uom::si::f64::Time;

fn Ts(params: &Parameters) -> Time {
    Time::new::<second>(params.Ts.v())
}

fn Tp(params: &Parameters) -> Time {
    Time::new::<second>(params.Tp.v())
}

fn ValidateRadiatorAlt(params: &Parameters) -> BassFnData {
    let g =  0.2; // τb / Ts 0.2 is a good guesstimate
    let g25 = 0.66874030497; // g ^ 0.25
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
        let d1 = ValidateRadiatorAlt(&params);
        let d2 = RadiatorAlt(&params);
        assert!(bass_data_cmp(&d1, &d2));
    }
}