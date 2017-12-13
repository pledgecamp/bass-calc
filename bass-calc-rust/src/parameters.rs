
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::f64::consts::PI;

use dim::si::*;

const PI2: f64 = 2.0 * PI;

/// Parameter that depends on one or more children
pub struct ParamPrivate {
    pub name: String,
    pub unit: String,
    value: Cell<f64>,
    pub min: f64,
    pub max: f64,
    pub update_fn: Option<fn(&Parameters) -> f64>,
    children: RefCell<Vec<Param>>,
    parents: RefCell<Vec<Param>>,
}

impl ParamPrivate {
    fn update(&self, params: &Parameters) {
        if let Some(update_fn) = self.update_fn {
            self.set(update_fn(params))
        }
    }

    fn v(&self) -> f64 {
        self.value.get()
    }

    fn set(&self, new_value: f64) {
        self.value.set(new_value)
    }
}

pub type Param = Rc<ParamPrivate>;

#[allow(non_snake_case)]
pub struct Parameters {
    // Environmental parameters
    ρ0: Param,
    c: Param,
    t: Param,

    // Driver low level parameters
    Xmax: Param,
    Vd: Param,
    Sd: Param,
    Bl: Param,
    Re: Param,
    Mmd: Param,
    Mms: Param,
    Mas: Param,
    Rms: Param,
    Ras: Param,
    Cms: Param,
    Cas: Param,
    Vas: Param,

    Rg: Param,

    // Driver mid level parameters
    Ts: Param,
    ωs: Param,
    Fs: Param,
    Qes: Param,
    Qms: Param,
    Qts: Param,
    Qs: Param,
    Cab: Param,
    Vb: Param,

    // Passive radiator low level parameters
    Vap: Param,
    Cmp: Param,
    Cap: Param,
    Rmp: Param,
    Rap: Param,
    Mmp: Param,
    Map: Param,
    Sp: Param,

    // Passive radiator mid level parameters
    Qmp: Param,
    ωp: Param,
    Fp: Param,
    Tp: Param,

    // Enclosure parameters
    ωb: Param,
    Fb: Param,
    Tb: Param,

    α: Param,
    δ: Param,
    y: Param,
    h: Param,
    η0: Param,
}

impl Parameters {
    pub fn driver_params(&self) -> [&Param; 23] {
        [&self.Xmax, &self.Vd, &self.Sd, &self.Bl, &self.Re, &self.Mmd, &self.Mms,
         &self.Mas, &self.Rms, &self.Ras, &self.Cms, &self.Cas, &self.Vas, &self.Rg,
         &self.Ts, &self.ωs, &self.Fs, &self.Qes, &self.Qms, &self.Qts, &self.Qs,
         &self.Cab, &self.Vb]
    }

    pub fn passive_params(&self) -> [&Param; 12] {
        [&self.Vap, &self.Cmp, &self.Cap, &self.Rmp, &self.Rap, &self.Mmp, &self.Map,
         &self.Sp, &self.Qmp, &self.ωp, &self.Fp, &self.Tp]
    }

    pub fn enclosure_params(&self) -> [&Param; 8] {
        [&self.ωb, &self.Fb, &self.Tb, &self.α, &self.δ, &self.y, &self.h, &self.η0]
    }

    pub fn constant_params(&self) -> [&Param; 3] {
        [&self.ρ0, &self.c, &self.t]
    }
}

fn param_simple(name: &str, unit: &str, value: f64, min: f64, max: f64) -> Param {
    make_param(name, unit, value, min, max, None)
}

fn param(name: &str, unit: &str, value: f64, min: f64, max: f64, update: fn(&Parameters) -> f64) -> Param {
    make_param(name, unit, value, min, max, Some(update))
}

fn make_param(name: &str, unit: &str, value: f64, min: f64, max: f64,
         update: Option<fn(&Parameters) -> f64>) -> Param {

    Rc::new(ParamPrivate {
        name: name.to_string(),
        unit: unit.to_string(),
        value: Cell::new(value),
        min: min,
        max: max,
        update_fn: update,
        children: RefCell::new(vec![]),
        parents: RefCell::new(vec![]),
    })
}

fn set_children(param_ref: &mut Param, children: Vec<Param>) {
    
    for mut child in children.into_iter() {

        param_ref.children.borrow_mut().push(child.clone());
        child.parents.borrow_mut().push(param_ref.clone());
        //Rc::get_mut(param).unwrap().parents.push(param.clone())
    }
}

fn vd_update(P: &Parameters) -> f64 {
    P.Sd.v() * P.Xmax.v()
}

fn mms_update(P: &Parameters) -> f64 {
    let Sd = P.Sd.v();
    P.Mmd.v() + (2.0 * ((8.0 * P.ρ0.v()) / (3.0 * PI2 * ( Sd / PI ).sqrt()))) * Sd.powi(2)
}

fn mas_update(P: &Parameters) -> f64 {
    P.Mms.v() / P.Sd.v().powi(2)
}

fn ras_update(P: &Parameters) -> f64 {
    P.Rms.v() / P.Sd.v().powi(2)
}

fn cas_update(P: &Parameters) -> f64 {
    P.Cms.v() * P.Sd.v().powi(2)
}

fn vas_update(P: &Parameters) -> f64 {
    P.ρ0.v() * P.c.v().powi(2) * P.Cas.v()
}

fn ts_update(P: &Parameters) -> f64 {
    1.0 / P.ωs.v()
}

fn ωs_update(P: &Parameters) -> f64 {
    P.Fs.v() * PI2
}

fn fs_update(P: &Parameters) -> f64 {
    1.0 / ( PI2 * (P.Mas.v() * P.Cas.v()).sqrt())
}

fn qes_update(P: &Parameters) -> f64 {
    (P.ωs.v() * P.Re.v() * P.Mas.v() * P.Sd.v().powi(2)) / P.Bl.v().powi(2)
}

fn qms_update(P: &Parameters) -> f64 {
    1.0 / (P.Bl.v() * P.Cas.v() * P.Ras.v())
}

fn qts_update(P: &Parameters) -> f64 {
    (P.Qes.v() * P.Qms.v()) / (P.Qes.v() + P.Qms.v())
}

fn qs_update(P: &Parameters) -> f64 {
    P.Qts.v()
}

fn vb_update(P: &Parameters) -> f64 {
    P.ρ0.v() * P.c.v().powi(2) * P.Cab.v()
}

fn vap_update(P: &Parameters) -> f64 {
    P.ρ0.v() * P.c.v().powi(2) * P.Cap.v()
}

fn cap_update(P: &Parameters) -> f64 {
    P.Cmp.v() * P.Sp.v().powi(2)
}

fn rap_update(P: &Parameters) -> f64 {
    P.Rmp.v() / P.Sp.v().powi(2)
}

fn map_update(P: &Parameters) -> f64 {
    P.Mmp.v() / P.Sp.v().powi(2)
}

fn qmp_update(P: &Parameters) -> f64 {
    1.0 / (P.ωp.v() * P.Cap.v() * P.Rap.v())
}

fn fp_update(P: &Parameters) -> f64 {
    1.0 / ( PI2 * (P.Map.v() * P.Cap.v()).sqrt())
}

fn tp_update(P: &Parameters) -> f64 {
    1.0 / P.ωp.v()
}

fn ωp_update(P: &Parameters) -> f64 {
    P.Fp.v() * PI2
}

fn fb_update(P: &Parameters) -> f64 {
    ((1.0 + (P.Cab.v() / P.Cap.v())) / (PI2 * P.Cab.v() * P.Map.v())).sqrt()
}

fn tb_update(P: &Parameters) -> f64 {
    1.0 / P.ωp.v()
}

fn ωb_update(P: &Parameters) -> f64 {
    P.Fp.v() * PI2
}

fn α_update(P: &Parameters) -> f64 {
    P.Cas.v() / P.Cab.v()
}

fn δ_update(P: &Parameters) -> f64 {
    P.Cap.v() / P.Cab.v()
}

fn y_update(P: &Parameters) -> f64 {
    P.Fb.v() / P.Fs.v()
}

fn h_update(P: &Parameters) -> f64 {
    P.Fb.v() / P.Fp.v()
}

fn η0_update(P: &Parameters) -> f64 {
    ((4.0 * PI.powi(2)) / P.c.v().powi(3)) * (P.Fs.v().powi(3) * P.Vas.v() / P.Qes.v())
}

pub fn default_parameters() -> Parameters {

    let mut P = Parameters {

        // Environmental parameters
        ρ0: param_simple("p0", "kg / m**3", 1.1839, 1.0, 1.4),
        c: param_simple("c", "m/s", 345.0, 340.0, 350.0),
        t: param_simple("t", "s", 1.0, 1.0, 1.0),

        // Driver low level parameters
        Xmax: param_simple("Xmax", "mm", 3.0, 0.0, 100.0),
        Vd: param("Vd", "Liter", 0.1, 0.1, 100.0, vd_update),
        Sd: param_simple("Sd", "cm ** 2", 10.0, 1.0, 1000.0),
        Bl: param_simple("Bl", "tesla meter", 1.0, 0.1, 20.0),
        Re: param_simple("Re", "ohm", 4.0, 0.1, 1000.0),
        Mmd: param_simple("Mmd", "g", 10.0, 1.0, 1000.0),
        Mms: param("Mms", "g", 10.0, 1.0, 1000.0, mms_update),
        Mas: param("Mas", "g * cm**2", 10.0, 1.0, 1000.0, mas_update),
        Rms: param_simple("Rms", "N * s / m", 4.0, 0.0, 1000.0),
        Ras: param("Ras", "ohm", 1.0, 0.0, 1000.0, ras_update),
        Cms: param_simple("Cms", "meter / N", 1.0, 0.1, 1000.0),
        Cas: param("Cas", "1 meter**5 / N", 1.0, 0.0, 100.0, cas_update),
        Vas: param("Vas", "liter", 1.0, 0.0, 100.0, vas_update),

        Rg: param_simple("Rg", "", 0.0, 0.0, 1000.0),

        // Driver mid level parameters
        Ts: param("Ts", "s", 0.02, 0.0002, 0.2, ts_update),
        ωs: param("ωs", "Hz", 50.0, 5.0, 5000.0, ωs_update),
        Fs: param("Fs", "Hz", 314.159, 31.4159, 31415.93, fs_update),
        Qes: param("Qes", "", 0.5, 0.0, 30.0, qes_update),
        Qms: param("Qms", "", 0.5, 0.0, 30.0, qms_update),
        Qts: param("Qts", "", 0.5, 0.0, 30.0, qts_update),
        Qs: param("Qs", "", 0.5, 0.0, 30.0, qs_update),
        Cab: param_simple("Cab", "meter**5 / N", 1.0, 0.0, 100.0),
        Vb: param("Vb", "liter", 0.1, 0.0, 100.0, vb_update),

        // Passive radiator low level parameters
        Vap: param("Vap", "liter", 1.0, 0.0, 100.0, vap_update),
        Cmp: param_simple("Cmp", "meter / N", 1.0, 0.0, 1000.0),
        Cap: param("Cap", "meter**5 / N", 1.0, 0.0, 100.0, cap_update),
        Rmp: param_simple("Rmp", "N * s / m", 4.0, 0.0, 1000.0),
        Rap: param("Rap", "ohm", 1.0, 0.0, 1000.0, rap_update),
        Mmp: param_simple("Mmp", "kg",  1.0, 0.001, 100.0),
        Map: param("Map", "kg / cm**2", 1.0, 0.0, 1000.0, map_update),
        Sp: param_simple("Sp", "cm**2", 10.0, 0.0, 1000.0),

        // Passive radiator mid level parameters
        Qmp: param("Qmp", "", 0.5, 0.0, 30.0, qmp_update),
        ωp: param("ωp", "Hz", 20.0, 0.0, 1000.0, ωp_update),
        Fp: param("Fp", "120Hz", 120.0, 0.0, 6282.0, fp_update),
        Tp: param("Tp", "0.05s", 0.05, 0.0, 0.1, tp_update),

        // Enclosure parameters
        ωb: param("ωb", "Hz", 20.0, 0.0, 1000.0, ωb_update),
        Fb: param("Fb", "120Hz", 120.0, 0.0, 6282.0, fb_update),
        Tb: param("Tb", "0.05s", 0.05, 0.0, 0.1, tb_update),

        α: param("α", "", 3.0, 0.0, 100.0, α_update),
        δ: param("δ", "", 7.0, 0.0, 100.0, δ_update),
        y: param("y", "", 0.5, 0.0, 100.0, y_update),
        h: param("h", "", 0.5, 0.0, 100.0, h_update),
        η0: param("η0", "", 0.4, 0.0, 100.0, η0_update),
    };
    
    set_children(&mut P.Vd, vec![P.Sd.clone(), P.Xmax.clone()]);
    set_children(&mut P.Mms, vec![P.Sd.clone(), P.Mmd.clone(), P.ρ0.clone()]);
    set_children(&mut P.Mas, vec![P.Sd.clone(), P.Mms.clone()]);
    set_children(&mut P.Ras, vec![P.Sd.clone(), P.Rms.clone()]);
    set_children(&mut P.Cas, vec![P.Sd.clone(), P.Cms.clone()]);
    set_children(&mut P.Vas, vec![P.Cas.clone(), P.ρ0.clone(), P.c.clone()]);
    set_children(&mut P.Ts, vec![P.ωs.clone()]);
    set_children(&mut P.ωs, vec![P.Fs.clone()]);
    set_children(&mut P.Fs, vec![P.Mas.clone(), P.Cas.clone()]);
    set_children(&mut P.Qes, vec![P.ωs.clone(), P.Re.clone(), P.Mas.clone(), P.Sd.clone(), P.Bl.clone()]);
    set_children(&mut P.Qms, vec![P.Bl.clone(), P.Cas.clone(), P.Ras.clone()]);
    set_children(&mut P.Qts, vec![P.Qes.clone(), P.Qms.clone()]);
    set_children(&mut P.Qs, vec![P.Qts.clone()]);
    set_children(&mut P.Vb, vec![P.ρ0.clone(), P.c.clone(), P.Cab.clone()]);
    set_children(&mut P.Vap, vec![P.ρ0.clone(), P.c.clone(), P.Cap.clone()]);
    set_children(&mut P.Cap, vec![P.Cmp.clone(), P.Sp.clone()]);
    set_children(&mut P.Rap, vec![P.Rmp.clone(), P.Sp.clone()]);
    set_children(&mut P.Map, vec![P.Mmp.clone(), P.Sp.clone()]);
    set_children(&mut P.Qmp, vec![P.ωp.clone(), P.Cap.clone(), P.Rap.clone()]);
    set_children(&mut P.Fp, vec![P.Map.clone(), P.Cap.clone()]);
    set_children(&mut P.Tp, vec![P.ωp.clone()]);
    set_children(&mut P.ωp, vec![P.Fp.clone()]);
    set_children(&mut P.Fb, vec![P.Cab.clone(), P.Cap.clone(), P.Map.clone()]);
    set_children(&mut P.Tb, vec![P.ωp.clone()]);
    set_children(&mut P.ωb, vec![P.Fp.clone()]);
    set_children(&mut P.α, vec![P.Cas.clone(), P.Cab.clone()]);
    set_children(&mut P.δ, vec![P.Cap.clone(), P.Cab.clone()]);
    set_children(&mut P.y, vec![P.Fb.clone(), P.Fs.clone()]);
    set_children(&mut P.h, vec![P.Fb.clone(), P.Fp.clone()]);
    set_children(&mut P.η0, vec![P.c.clone(), P.Fs.clone(), P.Vas.clone(), P.Qes.clone()]);

    P
}
