use vibe_coded_torus::*;

#[test]
fn luminance_within_theoretical_bounds() {
    let sqrt2 = 2.0_f64.sqrt();
    let (sin_a, cos_a) = 0.5_f64.sin_cos();
    let (sin_b, cos_b) = 0.3_f64.sin_cos();
    let theta_steps = (std::f64::consts::TAU / THETA_STEP) as usize;
    let phi_steps = (std::f64::consts::TAU / PHI_STEP) as usize;

    for ti in 0..theta_steps {
        let theta = ti as f64 * THETA_STEP;
        let (sin_t, cos_t) = theta.sin_cos();
        for pi in 0..phi_steps {
            let phi = pi as f64 * PHI_STEP;
            let (sin_p, cos_p) = phi.sin_cos();
            let t = Trig { sin_t, cos_t, sin_p, cos_p, sin_a, cos_a, sin_b, cos_b };
            let (_, _, _, l) = compute_sample(&t);
            assert!(
                l >= -sqrt2 - 1e-10 && l <= sqrt2 + 1e-10,
                "luminance {l} out of range"
            );
        }
    }
}

#[test]
fn char_mapping_dimmest() {
    assert_eq!(luminance_to_char(0.0), b'.');
    assert_eq!(luminance_to_char(0.1), b'.');
}

#[test]
fn char_mapping_brightest_clamps() {
    assert_eq!(luminance_to_char(100.0), b'@');
}

#[test]
fn char_mapping_mid_range() {
    assert_eq!(luminance_to_char(1.0), LUMINANCE_CHARS[8]);
}
