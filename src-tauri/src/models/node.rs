use super::Direction;

#[derive(serde::Serialize)]
pub struct NodeDTO {
    pub id: usize,
    pub x: f64,
    pub y: f64,
    pub hinged: bool,
    pub supports: [bool; 3],
    pub support_angle: f64,
    pub springs: [f64; 3],
    pub prescribed_displacements: [f64; 3],
    pub fx: f64,
    pub fy: f64,
    pub mz: f64,
    pub force_angle: f64,
}

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Node {
    id: usize,
    x: f64,
    y: f64,
    hinged: bool,
    supports: [bool; 3],
    support_angle: f64,
    springs: [f64; 3],
    prescribed_displacements: [f64; 3],
}

impl Node {
    pub fn new(id: usize, x: f64, y: f64) -> Node {
        Node {
            id,
            x,
            y,
            hinged: false,
            supports: [false, false, false],
            support_angle: 0.0,
            springs: [0.0, 0.0, 0.0],
            prescribed_displacements: [0.0, 0.0, 0.0],
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn pos(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    pub fn hinged(&self) -> bool {
        self.hinged
    }

    pub fn support(&self, direction: Direction) -> bool {
        self.supports[(direction as usize) - 1]
    }

    pub fn support_angle(&self) -> f64 {
        self.support_angle
    }

    pub fn spring(&self, direction: Direction) -> f64 {
        self.springs[(direction as usize) - 1]
    }

    pub fn prescribed_displacement(&self, direction: Direction) -> f64 {
        self.prescribed_displacements[(direction as usize) - 1]
    }

    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    pub fn set_pos(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    pub fn set_hinged(&mut self, hinged: bool) {
        self.hinged = hinged;
    }

    pub fn set_supports(&mut self, x: bool, y: bool, z: bool, angle: f64) {
        self.supports = [x, y, z];
        self.support_angle = angle;
    }

    pub fn set_springs(&mut self, x: f64, y: f64, z: f64) -> Result<(), Direction> {
        if x < 0.0 {
            return Err(Direction::X);
        }

        if y < 0.0 {
            return Err(Direction::Y);
        }

        if z < 0.0 {
            return Err(Direction::Z);
        }

        self.springs = [x, y, z];
        Ok(())
    }

    pub fn set_prescribed_displacements(&mut self, x: f64, y: f64, z: f64) {
        self.prescribed_displacements = [x, y, z];
    }
}
