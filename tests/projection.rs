use vibe_coded_torus::*;

#[test]
fn k1_formula() {
    let k1 = compute_k1(80);
    // 80 * 5 / (8 * 3) = 400 / 24
    assert!((k1 - 400.0 / 24.0).abs() < 1e-10);
}

#[test]
fn point_no_rotation_on_outer_equator() {
    let t = Trig {
        cos_t: 1.0, sin_t: 0.0,
        cos_p: 1.0, sin_p: 0.0,
        sin_a: 0.0, cos_a: 1.0,
        sin_b: 0.0, cos_b: 1.0,
    };
    let (x, y, z, _) = compute_sample(&t);
    assert!((x - 3.0).abs() < 1e-10);
    assert!(y.abs() < 1e-10);
    assert!((z - K2).abs() < 1e-10);
}

#[test]
fn z_always_positive_across_all_angles() {
    let angles = [0.0, 1.0, 2.0, std::f64::consts::PI];
    let theta_steps = (std::f64::consts::TAU / THETA_STEP) as usize;
    let phi_steps = (std::f64::consts::TAU / PHI_STEP) as usize;

    for ti in 0..theta_steps {
        let theta = ti as f64 * THETA_STEP;
        let (sin_t, cos_t) = theta.sin_cos();
        for pi in 0..phi_steps {
            let phi = pi as f64 * PHI_STEP;
            let (sin_p, cos_p) = phi.sin_cos();
            for &a in &angles {
                let (sin_a, cos_a) = a.sin_cos();
                for &b in &angles {
                    let (sin_b, cos_b) = b.sin_cos();
                    let t = Trig { sin_t, cos_t, sin_p, cos_p, sin_a, cos_a, sin_b, cos_b };
                    let (_, _, z, _) = compute_sample(&t);
                    assert!(z > 0.0, "z={z} at theta={theta}, phi={phi}, a={a}, b={b}");
                }
            }
        }
    }
}
