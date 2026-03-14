use vibe_coded_torus::*;

fn render_at(a: f64, b: f64) -> [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT] {
    let k1 = compute_k1(SCREEN_WIDTH);
    let mut output = [[b' '; SCREEN_WIDTH]; SCREEN_HEIGHT];
    let mut zbuffer = [[0.0_f64; SCREEN_WIDTH]; SCREEN_HEIGHT];
    render_frame(a, b, k1, &mut output, &mut zbuffer);
    output
}

fn render_with_zbuffer(a: f64, b: f64) -> ([[u8; SCREEN_WIDTH]; SCREEN_HEIGHT], [[f64; SCREEN_WIDTH]; SCREEN_HEIGHT]) {
    let k1 = compute_k1(SCREEN_WIDTH);
    let mut output = [[b' '; SCREEN_WIDTH]; SCREEN_HEIGHT];
    let mut zbuffer = [[0.0_f64; SCREEN_WIDTH]; SCREEN_HEIGHT];
    render_frame(a, b, k1, &mut output, &mut zbuffer);
    (output, zbuffer)
}

#[test]
fn frame_contains_torus_pixels() {
    let output = render_at(0.0, 0.0);
    let non_space = output.iter().flatten().filter(|&&c| c != b' ').count();
    assert!(non_space > 0, "frame should contain torus pixels");
}

#[test]
fn zbuffer_consistent_with_output() {
    let (output, zbuffer) = render_with_zbuffer(0.0, 0.0);
    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            if zbuffer[y][x] > 0.0 {
                assert_ne!(output[y][x], b' ', "zbuffer set at ({x},{y}) but output is space");
            }
        }
    }
}

#[test]
fn output_is_valid_ascii() {
    let output = render_at(1.0, 0.5);
    for row in &output {
        assert!(std::str::from_utf8(row).is_ok(), "output contains invalid UTF-8");
    }
}

#[test]
fn write_frame_dimensions() {
    let output = [[b'X'; SCREEN_WIDTH]; SCREEN_HEIGHT];
    let mut buf = String::new();
    write_frame(&output, &mut buf);
    let lines: Vec<&str> = buf.lines().collect();
    assert_eq!(lines.len(), SCREEN_HEIGHT);
    for line in &lines {
        assert_eq!(line.len(), SCREEN_WIDTH);
    }
}

#[test]
fn different_angles_produce_different_frames() {
    let out1 = render_at(0.0, 0.0);
    let out2 = render_at(1.0, 0.5);
    assert_ne!(out1, out2, "different rotation angles should produce different frames");
}
