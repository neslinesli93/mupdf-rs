use crate::Quad;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct IRect {
    pub x0: i32,
    pub y0: i32,
    pub x1: i32,
    pub y1: i32,
}

impl IRect {
    pub fn new(x0: i32, y0: i32, x1: i32, y1: i32) -> Self {
        Self { x0, y0, x1, y1 }
    }

    pub fn is_empty(&self) -> bool {
        self.x0 == self.x1 || self.y0 == self.y1
    }

    pub fn contains(&self, x: i32, y: i32) -> bool {
        if self.is_empty() {
            return false;
        }
        x >= self.x0 && x < self.x1 && y >= self.y0 && y < self.y1
    }

    pub fn r#union(&mut self, other: IRect) -> &mut Self {
        let IRect { x0, y0, x1, y1 } = other;
        if self.is_empty() {
            self.x0 = x0;
            self.y0 = y0;
            self.x1 = x1;
            self.y1 = y1;
        } else {
            if x0 < self.x0 {
                self.x0 = x0;
            }
            if y0 < self.y0 {
                self.y0 = y0;
            }
            if x1 > self.x1 {
                self.x1 = x1;
            }
            if y1 > self.y1 {
                self.y1 = y1;
            }
        }
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Rect {
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,
}

impl Rect {
    pub fn new(x0: f32, y0: f32, x1: f32, y1: f32) -> Self {
        Self { x0, y0, x1, y1 }
    }

    pub fn is_empty(&self) -> bool {
        self.x0 == self.x1 || self.y0 == self.y1
    }

    pub fn contains(&self, x: f32, y: f32) -> bool {
        if self.is_empty() {
            return false;
        }
        x >= self.x0 && x < self.x1 && y >= self.y0 && y < self.y1
    }

    pub fn r#union(&mut self, other: Rect) -> &mut Self {
        let Rect { x0, y0, x1, y1 } = other;
        if self.is_empty() {
            self.x0 = x0;
            self.y0 = y0;
            self.x1 = x1;
            self.y1 = y1;
        } else {
            if x0 < self.x0 {
                self.x0 = x0;
            }
            if y0 < self.y0 {
                self.y0 = y0;
            }
            if x1 > self.x1 {
                self.x1 = x1;
            }
            if y1 > self.y1 {
                self.y1 = y1;
            }
        }
        self
    }
}

impl From<IRect> for Rect {
    fn from(it: IRect) -> Rect {
        Rect {
            x0: it.x0 as f32,
            y0: it.y0 as f32,
            x1: it.x1 as f32,
            y1: it.y1 as f32,
        }
    }
}

impl From<Quad> for Rect {
    fn from(q: Quad) -> Rect {
        let Quad {
            ul_x,
            ul_y,
            ur_x,
            ur_y,
            ll_x,
            ll_y,
            lr_x,
            lr_y,
        } = q;
        let x0 = ul_x.min(ur_x).min(ll_x).min(lr_x);
        let y0 = ul_y.min(ur_y).min(ll_y).min(lr_y);
        let x1 = ul_x.max(ur_x).max(ll_x).max(lr_x);
        let y1 = ul_y.max(ur_y).max(ll_y).max(lr_y);
        Rect { x0, y0, x1, y1 }
    }
}