
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::f64::consts::PI;
use std::collections::HashMap;

const PI2: f64 = 2.0 * PI;

/// Parameter that depends on one or more children
pub struct ParamPrivate {
    pub name: String,
    pub unit: String,
    value: Cell<f64>,
    pub min: f64,
    pub max: f64,
    pub update_fn: Option<fn(&Parameters) -> f64>,
    precision: Cell<usize>,
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
    /*
    pub fn q<N>(&self) -> Quantity {
        self.unit
    }
    */

    pub fn to_percent(&self) -> f64 {
        (self.v() - self.min) / (self.max - self.min)
    }

    pub fn set_percent(&self, percent: f64) {
        self.set(self.min + percent*(self.max - self.min))
    }

    pub fn set(&self, new_value: f64) {
        self.value.set(new_value)
    }

    pub fn precision(&self) -> usize {
        self.precision.get()
    }

    pub fn set_precision(&self, precision: usize) {
        self.precision.set(precision)
    }
}

pub type Param = Rc<ParamPrivate>;

#[allow(non_snake_case)]
pub struct Parameters {
    pub param_map: HashMap<String, Param>,

    pub driver: [Param; 23],
    pub passive: [Param; 12],
    pub enclosure: [Param; 8],
    pub constant: [Param; 3],

    // Environmental parameters
    pub ρ0: Param,
    pub c: Param,
    pub t: Param,

    // Driver low level parameters
    pub Xmax: Param,
    pub Vd: Param,
    pub Sd: Param,
    pub Bl: Param,
    pub Re: Param,
    pub Mmd: Param,
    pub Mms: Param,
    pub Mas: Param,
    pub Rms: Param,
    pub Ras: Param,
    pub Cms: Param,
    pub Cas: Param,
    pub Vas: Param,

    pub Rg: Param,

    // Driver mid level parameters
    pub Ts: Param,
    pub ωs: Param,
    pub Fs: Param,
    pub Qes: Param,
    pub Qms: Param,
    pub Qts: Param,
    pub Qs: Param,
    pub Cab: Param,
    pub Vb: Param,

    // Passive radiator low level parameters
    pub Vap: Param,
    pub Cmp: Param,
    pub Cap: Param,
    pub Rmp: Param,
    pub Rap: Param,
    pub Mmp: Param,
    pub Map: Param,
    pub Sp: Param,

    // Passive radiator mid level parameters
    pub Qmp: Param,
    pub ωp: Param,
    pub Fp: Param,
    pub Tp: Param,

    // Enclosure parameters
    pub ωb: Param,
    pub Fb: Param,
    pub Tb: Param,

    pub α: Param,
    pub δ: Param,
    pub y: Param,
    pub h: Param,
    pub η0: Param,
}

impl Parameters {
    
    pub fn get(&self, name: &str) -> Option<Param> {

        if let Some(param) = self.param_map.get(name) {
            Some(param.clone())
        } else {
            None
        }
    }

}

pub fn param_simple(name: &str, unit: &str, value: f64, min: f64, max: f64, precision: usize) -> Param {

    make_param(name, unit, value, min, max, precision, None)
}

pub fn param(name: &str, unit: &str, value: f64, min: f64, max: f64, precision: usize,
        update: fn(&Parameters) -> f64) -> Param {

    make_param(name, unit, value, min, max, precision, Some(update))
}

pub fn make_param(name: &str, unit: &str, value: f64, min: f64, max: f64, precision: usize,
         update: Option<fn(&Parameters) -> f64>) -> Param {

    Rc::new(ParamPrivate {
        name: name.to_string(),
        unit: unit.to_string(),
        value: Cell::new(value),
        min,
        max,
        precision: Cell::new(precision),
        update_fn: update,
        children: RefCell::new(vec![]),
        parents: RefCell::new(vec![]),
    })
}

pub fn set_children(param_ref: &mut Param, children: Vec<Param>) {
    
    for mut child in children.into_iter() {

        param_ref.children.borrow_mut().push(child.clone());
        child.parents.borrow_mut().push(param_ref.clone());
        //Rc::get_mut(param).unwrap().parents.push(param.clone())
    }
}

// 10000 * cm^2 * mm = L
pub fn vd_update(P: &Parameters) -> f64 {
    10000.0 * P.Sd.v() * P.Xmax.v()
}

// g + 1000 * ((kg / m^3) / sqrt(cm^2)) * cm^4 = g
pub fn mms_update(P: &Parameters) -> f64 {
    let Sd = P.Sd.v();
    P.Mmd.v() + 1000.0 * (2.0 * ((8.0 * P.ρ0.v()) / (3.0 * PI2 * ( Sd / PI ).sqrt()))) * Sd.powi(2)
}

// g / cm^4 = g / cm^4
pub fn mas_update(P: &Parameters) -> f64 {
    P.Mms.v() / P.Sd.v().powi(2)
}

// (N * s / m) / (100000000 * cm^4) = (Pa * s) / m^3
pub fn ras_update(P: &Parameters) -> f64 {
    P.Rms.v() / (100000000.0 * P.Sd.v().powi(2))
}

// 100000000 * (1 m / N) * cm^4 = m^5 / N
pub fn cas_update(P: &Parameters) -> f64 {
    100000000.0 * P.Cms.v() * P.Sd.v().powi(2)
}

// (kg / m^3) * (m/s)^2 * (m^5 / N) / 1000 = L
pub fn vas_update(P: &Parameters) -> f64 {
    P.ρ0.v() * P.c.v().powi(2) * P.Cas.v() / 1000.0
}

// 1 / Hz = s
pub fn ts_update(P: &Parameters) -> f64 {
    1.0 / P.ωs.v()
}

// Hz = Hz
pub fn ωs_update(P: &Parameters) -> f64 {
    P.Fs.v() * PI2
}

// 1 / sqrt((g/cm^4) * (m^5 / N) / 100000) = Hz
pub fn fs_update(P: &Parameters) -> f64 {
    1.0 / ( PI2 * (P.Mas.v() * P.Cas.v() / 100000.0).sqrt())
}

// 1000 * (Hz * Ohm * (g/cm^4) * cm^4) / (tesla * m)^2 = 1
pub fn qes_update(P: &Parameters) -> f64 {
    1000.0 * (P.ωs.v() * P.Re.v() * P.Mas.v() * P.Sd.v().powi(2)) / P.Bl.v().powi(2)
}

// 1 / (Hz * (m^5 / N) * (Pa * s) / m^3) = 1
pub fn qms_update(P: &Parameters) -> f64 {
    1.0 / (P.ωs.v() * P.Cas.v() * P.Ras.v())
}

pub fn qts_update(P: &Parameters) -> f64 {
    (P.Qes.v() * P.Qms.v()) / (P.Qes.v() + P.Qms.v())
}

pub fn qs_update(P: &Parameters) -> f64 {
    P.Qts.v()
}

// (kg/m^3) * (m/s)^2 * (m^5 / N) / 1000 = L
pub fn vb_update(P: &Parameters) -> f64 {
    P.ρ0.v() * P.c.v().powi(2) * P.Cab.v() / 1000.0
}

// (kg/m^3) * (m/s)^2 * (m^5 / N) / 1000 = L
pub fn vap_update(P: &Parameters) -> f64 {
    P.ρ0.v() * P.c.v().powi(2) * P.Cap.v()
}

// 100000000 * (m/N) * cm^4 = m^5 / N
pub fn cap_update(P: &Parameters) -> f64 {
    100000000.0 * P.Cmp.v() * P.Sp.v().powi(2)
}

// ((N * s / m) / cm^4) / 100000000 = (Pa * s) / m^3
pub fn rap_update(P: &Parameters) -> f64 {
    (P.Rmp.v() / P.Sp.v().powi(2)) * 100000000.0
}

// kg / cm^4 = kg / cm^4
pub fn map_update(P: &Parameters) -> f64 {
    P.Mmp.v() / P.Sp.v().powi(2)
}

// 1 / (Hz * (m^5 / N) * (Pa * s)/m^3) = 1
pub fn qmp_update(P: &Parameters) -> f64 {
    1.0 / (P.ωp.v() * P.Cap.v() * P.Rap.v())
}

// 10000 / sqrt((kg/cm^4) * (m^5/N)) = Hz
pub fn fp_update(P: &Parameters) -> f64 {
    10000.0 / ( PI2 * (P.Map.v() * P.Cap.v()).sqrt())
}

// 1 / Hz = s
pub fn tp_update(P: &Parameters) -> f64 {
    1.0 / P.ωp.v()
}

pub fn ωp_update(P: &Parameters) -> f64 {
    P.Fp.v() * PI2
}

// 10000 * sqrt(1 / (m^5/N * kg/cm^4)) = Hz
pub fn fb_update(P: &Parameters) -> f64 {
    10000.0 * ((1.0 + (P.Cab.v() / P.Cap.v())) / (PI2 * P.Cab.v() * P.Map.v())).sqrt()
}

// 1 / Hz = s
pub fn tb_update(P: &Parameters) -> f64 {
    1.0 / P.ωp.v()
}

pub fn ωb_update(P: &Parameters) -> f64 {
    P.Fp.v() * PI2
}

pub fn α_update(P: &Parameters) -> f64 {
    P.Cas.v() / P.Cab.v()
}

pub fn δ_update(P: &Parameters) -> f64 {
    P.Cap.v() / P.Cab.v()
}

pub fn y_update(P: &Parameters) -> f64 {
    P.Fb.v() / P.Fs.v()
}

pub fn h_update(P: &Parameters) -> f64 {
    P.Fb.v() / P.Fp.v()
}

// (1000 / (m/s)^3) * (Hz^3 * L) = 1
pub fn η0_update(P: &Parameters) -> f64 {
    ((4.0 * PI.powi(2)) / P.c.v().powi(3)) * (P.Fs.v().powi(3) * P.Vas.v() / P.Qes.v())
}
