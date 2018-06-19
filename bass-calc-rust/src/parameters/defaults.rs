
use parameters::params::*;
use graphics::BassGraph;
use find_folder;
use std::path::PathBuf;
use std::collections::HashMap;
use csv::StringRecord;

const FILE_RECORD_LEN: usize = 6;

fn update_parameter(params: &Parameters, record: StringRecord) {
    if record.len() != FILE_RECORD_LEN {
        println!("Invalid file record len on line {}", record.position().unwrap().line());
    } else {
        let name = record.get(0).unwrap().to_string();
        if let Some(param) = params.get(&name) {
            
                let val_str = record.get(1).unwrap().to_string();
                if let Ok(val) = val_str.trim().parse::<f64>() {
                    param.set(val);
                } else {
                    println!("Could not parse default value {} for {}", val_str, name);
                }
                
                let prec_str = record.get(4).unwrap().to_string();
                if let Ok(prec) = prec_str.trim().parse::<usize>() {
                    param.set_precision(prec);
                } else {
                    println!("Could not parse precision {} for {}", val_str, name);
                }

        } else {
            println!("Unknown parameter {}", &name);
        }
    }
}

pub fn load_file(params: Parameters, path: &PathBuf) -> Parameters {
    use csv::{ReaderBuilder, ErrorKind};

    if let Ok(mut reader) = ReaderBuilder::new().comment(Some(b'#'))
                                    .from_path(path) {

        for result in reader.records() {
            match result {
                Ok(record) => update_parameter(&params, record),
                Err(err) => {
                    match err.kind() {
                        ErrorKind::UnequalLengths {pos: Some(p), .. } => {
                            println!("Error! Missing data on line {}", p.line())
                        },
                        _ => println!("Unknown error parsing parameter file")
                    };
                }
            }
        }
    }
    params
}

pub fn file_defaults() -> Parameters {

    let mut P = builtin_defaults();
    
    if let Ok(resources) = find_folder::Search::KidsThenParents(3, 5).for_folder("resources") {

        let defaults_path = resources.join("presets/defaults.bass");

        P = load_file(P, &defaults_path);
    }

    P
}

pub fn builtin_defaults() -> Parameters {

        // Environmental parameters
    let ρ0 = param_simple("ρ0", "kg / m^3", 1.1839, 1.0, 1.4, 4);
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

    let p_arr = vec![Xmax.clone(), Vd.clone(), Sd.clone(), Bl.clone(), Re.clone(), Mmd.clone(), Mms.clone(),
                 Mas.clone(), Rms.clone(), Ras.clone(), Cms.clone(), Cas.clone(), Vas.clone(), Rg.clone(),
                 Ts.clone(), ωs.clone(), Fs.clone(), Qes.clone(), Qms.clone(), Qts.clone(), Qs.clone(),
                 Cab.clone(), Vb.clone(),
                 Vap.clone(), Cmp.clone(), Cap.clone(), Rmp.clone(), Rap.clone(), Mmp.clone(),
                 Map.clone(), Sp.clone(), Qmp.clone(), ωp.clone(), Fp.clone(), Tp.clone(),
                 ωb.clone(), Fb.clone(), Tb.clone(),
                 α.clone(), δ.clone(), y.clone(), h.clone(), η0.clone(),
                 ρ0.clone(), c.clone(), t.clone()];

    let p_tuples = p_arr.into_iter().map(|p| (p.name.clone(), p)).collect::<Vec<(String, Param)>>();

    let p_map: HashMap<String, Param> = p_tuples.iter().cloned().collect();

    let mut P = Parameters {
        param_map: p_map,

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
        ρ0, c, t,

        // Driver low level parameters
        Xmax, Vd, Sd, Bl, Re, Mmd, Mms,
        Mas, Rms, Ras, Cms, Cas, Vas, Rg,

        // Driver mid level parameters
        Ts, ωs, Fs, Qes, Qms, Qts, Qs,
        Cab, Vb,

        // Passive radiator low level parameters
        Vap, Cmp, Cap, Rmp, Rap, Mmp, Map, Sp,

        // Passive radiator mid level parameters
        Qmp, ωp, Fp, Tp,

        // Enclosure parameters
        ωb, Fb, Tb, α, δ, y, h, η0,
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
