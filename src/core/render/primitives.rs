pub enum RenderPrimitive {
    Point { position: [f64; 3], size: f64 },
    Sphere { center: [f64; 3], radius: f64 },
    Line { from: [f64; 3], to: [f64; 3] },
    Vector { origin: [f64; 3], direction: [f64; 3], scale: f64 },
}
