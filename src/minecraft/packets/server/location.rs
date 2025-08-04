#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Location {
    x: f64,
    y: f64,
    z: f64,
    yaw: f32,
    pitch: f32,
}

impl Location {
    #[inline]
    pub fn new(x: f64, y: f64, z: f64, yaw: f32, pitch: f32) -> Self {
        Self {
            x,
            y,
            z,
            yaw,
            pitch,
        }
    }
    // get
    #[inline]
    pub fn get_x(&self) -> f64 {
        self.x
    }

    #[inline]
    pub fn get_y(&self) -> f64 {
        self.y
    }

    #[inline]
    pub fn get_z(&self) -> f64 {
        self.z
    }

    #[inline]
    pub fn get_xyz(&self) -> (f64, f64, f64) {
        (self.get_x(), self.get_y(), self.get_z())
    }

    #[inline]
    pub fn get_yaw(&self) -> f32 {
        self.yaw
    }

    #[inline]
    pub fn get_pitch(&self) -> f32 {
        self.pitch
    }

    #[inline]
    pub fn get_direction(&self) -> (f32, f32) {
        (self.get_yaw(), self.get_pitch())
    }

    // set
    #[inline]
    pub fn set_x(&mut self, v: f64) {
        self.x = v
    }

    #[inline]
    pub fn set_y(&mut self, v: f64) {
        self.y = v
    }

    #[inline]
    pub fn set_z(&mut self, v: f64) {
        self.z = v
    }

    #[inline]
    pub fn set_xyz(&mut self, x: f64, y: f64, z: f64) {
        self.set_x(x);
        self.set_y(y);
        self.set_z(z);
    }

    #[inline]
    pub fn set_yaw(&mut self, v: f32) {
        self.yaw = v
    }

    #[inline]
    pub fn set_pitch(&mut self, v: f32) {
        self.pitch = v
    }

    #[inline]
    pub fn set_direction(&mut self, yaw: f32, pitch: f32) {
        self.set_yaw(yaw);
        self.set_pitch(pitch)
    }
}
