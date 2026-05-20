use macroquad::prelude::Color;

// ── geometry ──────────────────────────────────────────────────────────────────

/// Returns `true` when `p` lies inside the triangle `v1`–`v2`–`v3`.
pub(crate) fn is_point_in_triangle(p: (f32, f32), v1: (f32, f32), v2: (f32, f32), v3: (f32, f32)) -> bool {
    let d1 = sign(p, v1, v2);
    let d2 = sign(p, v2, v3);
    let d3 = sign(p, v3, v1);
    !((d1 < 0.0 || d2 < 0.0 || d3 < 0.0) && (d1 > 0.0 || d2 > 0.0 || d3 > 0.0))
}

fn sign(p1: (f32, f32), p2: (f32, f32), p3: (f32, f32)) -> f32 {
    (p1.0 - p3.0) * (p2.1 - p3.1) - (p2.0 - p3.0) * (p1.1 - p3.1)
}

// ── color ─────────────────────────────────────────────────────────────────────

/// Linear color gradient from `c1` to `c2` in `steps` steps.
pub(crate) fn gradient(c1: Color, c2: Color, steps: usize) -> Vec<Color> {
    if steps <= 1 {
        return vec![c2];
    }
    let n = steps as f32 - 1.0;
    (0..steps)
        .map(|i| {
            let t = i as f32 / n;
            Color::new(
                c1.r + (c2.r - c1.r) * t,
                c1.g + (c2.g - c1.g) * t,
                c1.b + (c2.b - c1.b) * t,
                c1.a + (c2.a - c1.a) * t,
            )
        })
        .collect()
}

// ── time ──────────────────────────────────────────────────────────────────────

pub(crate) fn current_millis() -> u128 {
    std::time::UNIX_EPOCH.elapsed().unwrap().as_millis()
}

/// Microsecond-precision wall-clock time.  Used for high-FPS frame pacing where
/// 1 ms granularity would overshoot the target frame budget.
pub(crate) fn current_micros() -> u128 {
    std::time::UNIX_EPOCH.elapsed().unwrap().as_micros()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_in_triangle_detects_inside_and_outside() {
        let v1 = (0.0, 0.0);
        let v2 = (2.0, 0.0);
        let v3 = (0.0, 2.0);
        assert!(is_point_in_triangle((0.5, 0.5), v1, v2, v3));
        assert!(!is_point_in_triangle((2.0, 2.0), v1, v2, v3));
    }

    #[test]
    fn gradient_returns_expected_endpoints() {
        let a = Color::new(0.0, 0.0, 0.0, 1.0);
        let b = Color::new(1.0, 0.5, 0.25, 1.0);
        let g = gradient(a, b, 3);
        assert_eq!(g.len(), 3);
        assert_eq!(g.first().unwrap().r, a.r);
        assert_eq!(g.last().unwrap().r, b.r);

        let one = gradient(a, b, 1);
        assert_eq!(one.len(), 1);
        assert_eq!(one[0].r, b.r);
    }

    #[test]
    fn current_millis_is_monotonic() {
        let t1 = current_millis();
        let t2 = current_millis();
        assert!(t2 >= t1);
    }
}
