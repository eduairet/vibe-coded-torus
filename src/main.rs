use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use vibe_coded_torus::*;

struct CursorGuard;

impl CursorGuard {
    fn new(out: &mut impl Write) -> Self {
        let _ = write!(out, "\x1B[2J\x1B[?25l");
        let _ = out.flush();
        Self
    }
}

impl Drop for CursorGuard {
    fn drop(&mut self) {
        let stdout = io::stdout();
        let mut out = stdout.lock();
        let _ = write!(out, "\x1B[?25h");
        let _ = out.flush();
    }
}

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("failed to set Ctrl+C handler");

    let k1 = compute_k1(SCREEN_WIDTH);

    let mut a: f64 = 0.0;
    let mut b: f64 = 0.0;

    let stdout = io::stdout();
    let mut out = stdout.lock();

    let _guard = CursorGuard::new(&mut out);

    let mut output = [[b' '; SCREEN_WIDTH]; SCREEN_HEIGHT];
    let mut zbuffer = [[0.0_f64; SCREEN_WIDTH]; SCREEN_HEIGHT];
    let mut frame_buf = String::with_capacity((SCREEN_WIDTH + 1) * SCREEN_HEIGHT);

    while running.load(Ordering::SeqCst) {
        render_frame(a, b, k1, &mut output, &mut zbuffer);

        write_frame(&output, &mut frame_buf);
        write!(out, "\x1B[H{}", frame_buf).unwrap();
        out.flush().unwrap();

        a += 0.07;
        b += 0.03;

        thread::sleep(Duration::from_millis(33));
    }
}
