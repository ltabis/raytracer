/// A color struct containing rgba values.
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64
}

impl Color {
    
    pub fn black() -> Color {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0
        }
    }

    pub fn clone(to_clone: &Color) -> Color {
        Color {
            r: to_clone.r,
            g: to_clone.g,
            b: to_clone.b,
            a: to_clone.a
        }
    }
    pub fn clamp(&self) -> Color {
        Color {
            r: self.r.min(1.0).max(0.0),
            g: self.g.min(1.0).max(0.0),
            b: self.b.min(1.0).max(0.0),
            a: self.a.min(1.0).max(0.0)
        }
    }
}
