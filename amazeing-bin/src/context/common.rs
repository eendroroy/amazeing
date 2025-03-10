pub(crate) fn display_size(display_size: &str) -> (f32, f32, f32, f32) {
    // margin, padding, cell_width, cell_height
    match display_size {
        "xs" | "x-small" => (5., 1., 5., 5.),
        "s" | "small" => (10., 1., 10., 10.),
        "m" | "medium" => (15., 2., 15., 15.),
        "l" | "large" => (25., 3., 20., 20.),
        "xl" | "x-large" => (30., 4., 30., 30.),
        _ => (15., 2., 15., 15.),
    }
}
