use std::collections::HashMap;

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MemberDTO {
    pub id: usize,

    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,

    pub length: f64,
    pub angle: f64,

    pub hinges: HashMap<String, bool>,

    pub material: usize,
    pub section: usize,

    pub qx0: f64,
    pub qy0: f64,
    pub qx1: f64,
    pub qy1: f64,
    pub is_global: bool,

    pub t_sup: f64,
    pub t_inf: f64,
}

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Member {
    id: usize,
    node_start: usize,
    node_end: usize,
    material: usize,
    section: usize,
    hinges: [bool; 2],
}

impl Member {
    pub fn new(
        id: usize,
        node_start: usize,
        node_end: usize,
        material: usize,
        section: usize,
    ) -> Result<Member, usize> {
        if node_start == node_end {
            return Err(0);
        }

        Ok(Member {
            id,
            node_start,
            node_end,
            material,
            section,
            hinges: [false, false],
        })
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn node_start(&self) -> usize {
        self.node_start
    }

    pub fn node_end(&self) -> usize {
        self.node_end
    }

    pub fn material(&self) -> usize {
        self.material
    }

    pub fn section(&self) -> usize {
        self.section
    }

    pub fn hinge_start(&self) -> bool {
        self.hinges[0]
    }

    pub fn hinge_end(&self) -> bool {
        self.hinges[1]
    }

    pub fn set_node_start(&mut self, node: usize) -> Result<(), usize> {
        if self.node_start == node || self.node_end == node {
            return Err(0);
        }
        self.node_start = node;
        Ok(())
    }

    pub fn set_node_end(&mut self, node: usize) -> Result<(), usize> {
        if self.node_start == node || self.node_end == node {
            return Err(0);
        }
        self.node_start = node;
        Ok(())
    }

    pub fn set_material(&mut self, material: usize) {
        self.material = material;
    }

    pub fn set_section(&mut self, section: usize) {
        self.section = section;
    }

    pub fn set_hinges(&mut self, hinge_start: bool, hinge_end: bool) {
        self.hinges[0] = hinge_start;
        self.hinges[1] = hinge_end;
    }
}
