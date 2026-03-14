use std::f64::consts::TAU;

pub const SCREEN_WIDTH: usize = 80;
pub const SCREEN_HEIGHT: usize = 24;

pub const R1: f64 = 1.0;
pub const R2: f64 = 2.0;
pub const K2: f64 = 5.0;

pub const THETA_STEP: f64 = 0.07;
pub const PHI_STEP: f64 = 0.02;

pub const LUMINANCE_CHARS: &[u8] = b".,-~:;=!*#$@";

const THETA_STEPS: usize = (TAU / THETA_STEP) as usize;
const PHI_STEPS: usize = (TAU / PHI_STEP) as usize;

pub fn compute_k1(screen_width: usize) -> f64 {
    screen_width as f64 * K2 / (8.0 * (R1 + R2))
}

#[derive(Clone, Copy)]
pub struct Trig {
    pub sin_t: f64, pub cos_t: f64,
    pub sin_p: f64, pub cos_p: f64,
    pub sin_a: f64, pub cos_a: f64,
    pub sin_b: f64, pub cos_b: f64,
}

pub fn compute_sample(t: &Trig) -> (f64, f64, f64, f64) {
    let circle_x = R2 + R1 * t.cos_t;
    let circle_y = R1 * t.sin_t;

    let x = circle_x * (t.cos_b * t.cos_p + t.sin_a * t.sin_b * t.sin_p)
        - circle_y * t.cos_a * t.sin_b;
    let y = circle_x * (t.sin_b * t.cos_p - t.sin_a * t.cos_b * t.sin_p)
        + circle_y * t.cos_a * t.cos_b;
    let z = K2 + t.cos_a * circle_x * t.sin_p + circle_y * t.sin_a;

    let luminance = t.cos_p * t.cos_t * t.sin_b
        - t.cos_a * t.cos_t * t.sin_p
        - t.sin_a * t.sin_t
        + t.cos_b * (t.cos_a * t.sin_t - t.cos_t * t.sin_a * t.sin_p);

    (x, y, z, luminance)
}

pub fn luminance_to_char(luminance: f64) -> u8 {
    let idx = (luminance * 8.0).max(0.0).min((LUMINANCE_CHARS.len() - 1) as f64) as usize;
    LUMINANCE_CHARS[idx]
}

pub fn render_frame(
    a: f64, b: f64, k1: f64,
    output: &mut [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT],
    zbuffer: &mut [[f64; SCREEN_WIDTH]; SCREEN_HEIGHT],
) {
    for row in output.iter_mut() { row.fill(b' '); }
    for row in zbuffer.iter_mut() { row.fill(0.0); }

    let (sin_a, cos_a) = a.sin_cos();
    let (sin_b, cos_b) = b.sin_cos();

    for ti in 0..THETA_STEPS {
        let theta = ti as f64 * THETA_STEP;
        let (sin_t, cos_t) = theta.sin_cos();

        for pi in 0..PHI_STEPS {
            let phi = pi as f64 * PHI_STEP;
            let (sin_p, cos_p) = phi.sin_cos();

            let t = Trig { sin_t, cos_t, sin_p, cos_p, sin_a, cos_a, sin_b, cos_b };
            let (x, y, z, luminance) = compute_sample(&t);
            let ooz = 1.0 / z;

            let xp = (SCREEN_WIDTH as f64 / 2.0 + k1 * ooz * x) as isize;
            let yp = (SCREEN_HEIGHT as f64 / 2.0 - k1 * ooz * y / 2.0) as isize;

            if luminance > 0.0
                && xp >= 0
                && (xp as usize) < SCREEN_WIDTH
                && yp >= 0
                && (yp as usize) < SCREEN_HEIGHT
            {
                let xp = xp as usize;
                let yp = yp as usize;

                if ooz > zbuffer[yp][xp] {
                    zbuffer[yp][xp] = ooz;
                    output[yp][xp] = luminance_to_char(luminance);
                }
            }
        }
    }
}

pub fn write_frame(output: &[[u8; SCREEN_WIDTH]; SCREEN_HEIGHT], buf: &mut String) {
    buf.clear();
    for row in output {
        // All bytes are ASCII literals from LUMINANCE_CHARS or b' '
        buf.push_str(unsafe { std::str::from_utf8_unchecked(row) });
        buf.push('\n');
    }
}
