
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
    pub precision: usize,
    children: RefCell<Vec<Param>>,
    parents: RefCell<Vec<Param>>,
}

impl ParamPrivate {
    fn update(&self, params: &Parameters) {
        if let Some(update_fn) = self.update_fn {
            self.set(update_fn(params))
        }
    }

    pub fn v(&self) -> f64 {
        self.value.get()
    }

    pub fn to_percent(&self) -> f64 {
        (self.v() - self.min) / (self.max - self.min)
    }

    pub fn set_percent(&self, percent: f64) {
        self.set(self.min + percent*(self.max - self.min))
    }

    fn set(&self, new_value: f64) {
        self.value.set(new_value)
    }
}

pub type Param = Rc<ParamPrivate>;

#[allow(non_snake_case)]
pub struct Parameters {

    pub driver: [Param; 23],
    pub passive: [Param; 12],
    pub enclosure: [Param; 8],
    pub constant: [Param; 3],

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
    
}

fn param_simple(name: &str, unit: &str, value: f64, min: f64, max: f64, precision: usize) -> Param {

    make_param(name, unit, value, min, max, precision, None)
}

fn param(name: &str, unit: &str, value: f64, min: f64, max: f64, precision: usize,
        update: fn(&Parameters) -> f64) -> Param {

    make_param(name, unit, value, min, max, precision, Some(update))
}

fn make_param(name: &str, unit: &str, value: f64, min: f64, max: f64, precision: usize,
         update: Option<fn(&Parameters) -> f64>) -> Param {

    Rc::new(ParamPrivate {
        name: name.to_string(),
        unit: unit.to_string(),
        value: Cell::new(value),
        min,
        max,
        precision,
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

// 10000 * cm^2 * mm = L
fn vd_update(P: &Parameters) -> f64 {
    10000.0 * P.Sd.v() * P.Xmax.v()
}

// g + 1000 * ((kg / m^3) / sqrt(cm^2)) * cm^4 = g
fn mms_update(P: &Parameters) -> f64 {
    let Sd = P.Sd.v();
    P.Mmd.v() + 1000.0 * (2.0 * ((8.0 * P.ρ0.v()) / (3.0 * PI2 * ( Sd / PI ).sqrt()))) * Sd.powi(2)
}

// g / cm^4 = g / cm^4
fn mas_update(P: &Parameters) -> f64 {
    P.Mms.v() / P.Sd.v().powi(2)
}

// (N * s / m) / (100000000 * cm^4) = (Pa * s) / m^3
fn ras_update(P: &Parameters) -> f64 {
    P.Rms.v() / (100000000.0 * P.Sd.v().powi(2))
}

// 100000000 * (1 m / N) * cm^4 = m^5 / N
fn cas_update(P: &Parameters) -> f64 {
    100000000.0 * P.Cms.v() * P.Sd.v().powi(2)
}

// (kg / m^3) * (m/s)^2 * (m^5 / N) / 1000 = L
fn vas_update(P: &Parameters) -> f64 {
    P.ρ0.v() * P.c.v().powi(2) * P.Cas.v() / 1000.0
}

// 1 / Hz = s
fn ts_update(P: &Parameters) -> f64 {
    1.0 / P.ωs.v()
}

// Hz = Hz
fn ωs_update(P: &Parameters) -> f64 {
    P.Fs.v() * PI2
}

// 1 / sqrt((g/cm^4) * (m^5 / N) / 100000) = Hz
fn fs_update(P: &Parameters) -> f64 {
    1.0 / ( PI2 * (P.Mas.v() * P.Cas.v() / 100000.0).sqrt())
}

// 1000 * (Hz * Ohm * (g/cm^4) * cm^4) / (tesla * m)^2 = 1
fn qes_update(P: &Parameters) -> f64 {
    1000.0 * (P.ωs.v() * P.Re.v() * P.Mas.v() * P.Sd.v().powi(2)) / P.Bl.v().powi(2)
}

// 1 / (Hz * (m^5 / N) * (Pa * s) / m^3) = 1
fn qms_update(P: &Parameters) -> f64 {
    1.0 / (P.ωs.v() * P.Cas.v() * P.Ras.v())
}

fn qts_update(P: &Parameters) -> f64 {
    (P.Qes.v() * P.Qms.v()) / (P.Qes.v() + P.Qms.v())
}

fn qs_update(P: &Parameters) -> f64 {
    P.Qts.v()
}

// (kg/m^3) * (m/s)^2 * (m^5 / N) / 1000 = L
fn vb_update(P: &Parameters) -> f64 {
    P.ρ0.v() * P.c.v().powi(2) * P.Cab.v() / 1000.0
}

// (kg/m^3) * (m/s)^2 * (m^5 / N) / 1000 = L
fn vap_update(P: &Parameters) -> f64 {
    P.ρ0.v() * P.c.v().powi(2) * P.Cap.v()
}

// 100000000 * (m/N) * cm^4 = m^5 / N
fn cap_update(P: &Parameters) -> f64 {
    100000000.0 * P.Cmp.v() * P.Sp.v().powi(2)
}

// ((N * s / m) / cm^4) / 100000000 = (Pa * s) / m^3
fn rap_update(P: &Parameters) -> f64 {
    (P.Rmp.v() / P.Sp.v().powi(2)) * 100000000.0
}

// kg / cm^4 = kg / cm^4
fn map_update(P: &Parameters) -> f64 {
    P.Mmp.v() / P.Sp.v().powi(2)
}

// 1 / (Hz * (m^5 / N) * (Pa * s)/m^3) = 1
fn qmp_update(P: &Parameters) -> f64 {
    1.0 / (P.ωp.v() * P.Cap.v() * P.Rap.v())
}

// 10000 / sqrt((kg/cm^4) * (m^5/N)) = Hz
fn fp_update(P: &Parameters) -> f64 {
    10000.0 / ( PI2 * (P.Map.v() * P.Cap.v()).sqrt())
}

// 1 / Hz = s
fn tp_update(P: &Parameters) -> f64 {
    1.0 / P.ωp.v()
}

fn ωp_update(P: &Parameters) -> f64 {
    P.Fp.v() * PI2
}

// 10000 * sqrt(1 / (m^5/N * kg/cm^4)) = Hz
fn fb_update(P: &Parameters) -> f64 {
    10000.0 * ((1.0 + (P.Cab.v() / P.Cap.v())) / (PI2 * P.Cab.v() * P.Map.v())).sqrt()
}

// 1 / Hz = s
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

// (1000 / (m/s)^3) * (Hz^3 * L) = 1
fn η0_update(P: &Parameters) -> f64 {
    ((4.0 * PI.powi(2)) / P.c.v().powi(3)) * (P.Fs.v().powi(3) * P.Vas.v() / P.Qes.v())
}

pub fn default_parameters() -> Parameters {

        // Environmental parameters
    let ρ0 = param_simple("p0", "kg / m^3", 1.1839, 1.0, 1.4, 4);
    let c = param_simple("c", "m/s", 345.0, 340.0, 350.0, 1);
    let t = param_simple("t", "s", 1.0, 0.9, 1.1, 1);

    // Driver low level parameters
    let Xmax = param_simple("Xmax", "mm", 3.0, 0.0, 100.0, 1);
    let Vd = param("Vd", "Liter", 0.1, 0.1, 100.0, 1, vd_update);
    let Sd = param_simple("Sd", "cm ^ 2", 10.0, 1.0, 1000.0, 1);
    let Bl = param_simple("Bl", "tesla m", 1.0, 0.1, 20.0, 1);
    let Re = param_simple("Re", "ohm", 4.0, 0.1, 1000.0, 1);
    let Mmd = param_simple("Mmd", "g", 10.0, 1.0, 1000.0, 1);
    let Mms = param("Mms", "g", 10.0, 1.0, 1000.0, 1, mms_update);
    let Mas = param("Mas", "g / cm^4", 10.0, 1.0, 1000.0, 1, mas_update);
    let Rms = param_simple("Rms", "N * s / m", 4.0, 0.0, 1000.0, 1);
    let Ras = param("Ras", "(Pa * s) / m^3", 1.0, 0.0, 1000.0, 1, ras_update);
    let Cms = param_simple("Cms", "m / N", 1.0, 0.1, 1000.0, 1);
    let Cas = param("Cas", "m^5 / N", 1.0, 0.0, 100.0, 1, cas_update);
    let Vas = param("Vas", "liter", 1.0, 0.0, 100.0, 1, vas_update);

    let Rg = param_simple("Rg", "", 0.0, 0.0, 1000.0, 1);

    // Driver mid level parameters
    let Ts = param("Ts", "s", 0.02, 0.0002, 0.2, 4, ts_update);
    let ωs = param("ωs", "Hz", 50.0, 5.0, 5000.0, 1, ωs_update);
    let Fs = param("Fs", "Hz", 314.1, 31.4, 31415.9, 1, fs_update);
    let Qes = param("Qes", "", 0.5, 0.0, 30.0, 1, qes_update);
    let Qms = param("Qms", "", 0.5, 0.0, 30.0, 1, qms_update);
    let Qts = param("Qts", "", 0.5, 0.0, 30.0, 1, qts_update);
    let Qs = param("Qs", "", 0.5, 0.0, 30.0, 1, qs_update);
    let Cab = param_simple("Cab", "m^5 / N", 1.0, 0.0, 100.0, 1);
    let Vb = param("Vb", "liter", 0.1, 0.0, 100.0, 1, vb_update);

    // Passive radiator low level parameters
    let Vap = param("Vap", "liter", 1.0, 0.0, 100.0, 1, vap_update);
    let Cmp = param_simple("Cmp", "m / N", 1.0, 0.0, 1000.0, 1);
    let Cap = param("Cap", "m^5 / N", 1.0, 0.0, 100.0, 1, cap_update);
    let Rmp = param_simple("Rmp", "N * s / m", 4.0, 0.0, 1000.0, 1);
    let Rap = param("Rap", "ohm", 1.0, 0.0, 1000.0, 1, rap_update);
    let Mmp = param_simple("Mmp", "kg",  1.0, 0.001, 100.0, 3);
    let Map = param("Map", "kg / cm^2", 1.0, 0.0, 1000.0, 1, map_update);
    let Sp = param_simple("Sp", "cm^2", 10.0, 0.0, 1000.0, 1);

    // Passive radiator mid level parameters
    let Qmp = param("Qmp", "", 0.5, 0.0, 30.0, 1, qmp_update);
    let ωp = param("ωp", "Hz", 20.0, 0.0, 1000.0, 1, ωp_update);
    let Fp = param("Fp", "Hz", 120.0, 0.0, 6282.0, 1, fp_update);
    let Tp = param("Tp", "s", 0.05, 0.0, 0.1, 1, tp_update);

    // Enclosure parameters
    let ωb = param("ωb", "Hz", 20.0, 0.0, 1000.0, 1, ωb_update);
    let Fb = param("Fb", "Hz", 120.0, 0.0, 6282.0, 1, fb_update);
    let Tb = param("Tb", "s", 0.05, 0.0, 0.1, 1, tb_update);

    let α = param("α", "", 3.0, 0.0, 100.0, 1, α_update);
    let δ = param("δ", "", 7.0, 0.0, 100.0, 1, δ_update);
    let y = param("y", "", 0.5, 0.0, 100.0, 1, y_update);
    let h = param("h", "", 0.5, 0.0, 100.0, 1, h_update);
    let η0 = param("η0", "", 0.4, 0.0, 100.0, 1, η0_update);

    let mut P = Parameters {

        driver: [Xmax.clone(), Vd.clone(), Sd.clone(), Bl.clone(), Re.clone(), Mmd.clone(), Mms.clone(),
                 Mas.clone(), Rms.clone(), Ras.clone(), Cms.clone(), Cas.clone(), Vas.clone(), Rg.clone(),
                 Ts.clone(), ωs.clone(), Fs.clone(), Qes.clone(), Qms.clone(), Qts.clone(), Qs.clone(),
                 Cab.clone(), Vb.clone()],
        passive: [Vap.clone(), Cmp.clone(), Cap.clone(), Rmp.clone(), Rap.clone(), Mmp.clone(),
                  Map.clone(), Sp.clone(), Qmp.clone(), ωp.clone(), Fp.clone(), Tp.clone()],
        enclosure: [ωb.clone(), Fb.clone(), Tb.clone(),
                    α.clone(), δ.clone(), y.clone(), h.clone(), η0.clone()],
        constant: [ρ0.clone(), c.clone(), t.clone()],

        // Environmental parameters
        ρ0: ρ0, c: c, t: t,

        // Driver low level parameters
        Xmax: Xmax, Vd: Vd, Sd: Sd, Bl: Bl, Re: Re, Mmd: Mmd, Mms: Mms,
        Mas: Mas, Rms: Rms, Ras: Ras, Cms: Cms, Cas: Cas, Vas: Vas, Rg: Rg,

        // Driver mid level parameters
        Ts: Ts, ωs: ωs, Fs: Fs, Qes: Qes, Qms: Qms, Qts: Qts, Qs: Qs,
        Cab: Cab, Vb: Vb,

        // Passive radiator low level parameters
        Vap: Vap, Cmp: Cmp, Cap: Cap, Rmp: Rmp, Rap: Rap, Mmp: Mmp, Map: Map, Sp: Sp,

        // Passive radiator mid level parameters
        Qmp: Qmp, ωp: ωp, Fp: Fp, Tp: Tp,

        // Enclosure parameters
        ωb: ωb, Fb: Fb, Tb: Tb, α: α, δ: δ, y: y, h: h, η0: η0,
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
