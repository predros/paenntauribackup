use super::ViewModel;
use super::ViewModelError;
use crate::models::UnitType;
use crate::models::{Direction, Node, NodeDTO};

impl ViewModel {
    pub fn node_delete(&mut self, id: usize, state_save: bool) -> Result<usize, ViewModelError> {
        let index = self.nodes_list.iter().position(|x| x.id() == id);

        let index = match index {
            Some(value) => value,
            None => return Err(ViewModelError::InvalidNodeId(id)),
        };

        for member in self.members_list.iter() {
            if member.node_start() == id || member.node_end() == id {
                return Err(ViewModelError::NodeInUse);
            }
        }

        if state_save {
            let _ = self.state_save();
        }

        self.nodes_list.remove(index);

        for (_, case) in self.loadcases_list.iter_mut() {
            let _ = case.remove_node(id);
        }

        Ok(0)
    }

    pub fn node_get(&self, id: usize) -> Result<&Node, ViewModelError> {
        for node in self.nodes_list.iter() {
            if node.id() == id {
                return Ok(node);
            }
        }
        Err(ViewModelError::InvalidMemberId(id))
    }

    pub fn node_get_dtos(&self) -> Result<Vec<NodeDTO>, ViewModelError> {
        let mut result: Vec<NodeDTO> = vec![];

        let case = self.loadcases_list.get(&self.loadcase_current);
        let case = match case {
            Some(value) => value,
            None => return Err(ViewModelError::InvalidLoadcaseId(self.loadcase_current)),
        };

        for node in self.nodes_list.iter() {
            let id = node.id();
            let nodal = case.get_nodal(id);
            let nodal = match nodal {
                Some(value) => value,
                None => return Err(ViewModelError::InvalidNodeId(id)),
            };

            result.push(NodeDTO {
                id,
                x: node.x(),
                y: node.y(),
                hinged: node.hinged(),
                supports: [
                    node.support(Direction::X),
                    node.support(Direction::Y),
                    node.support(Direction::Z),
                ],
                support_angle: self.unit_to(node.support_angle(), UnitType::Angle),
                springs: [
                    self.unit_to(node.spring(Direction::X), UnitType::Spring),
                    self.unit_to(node.spring(Direction::Y), UnitType::Spring),
                    self.unit_to(node.spring(Direction::Z), UnitType::TorsionSpring),
                ],
                prescribed_displacements: [
                    self.unit_to(
                        node.prescribed_displacement(Direction::X),
                        UnitType::Displacement,
                    ),
                    self.unit_to(
                        node.prescribed_displacement(Direction::Y),
                        UnitType::Displacement,
                    ),
                    self.unit_to(
                        node.prescribed_displacement(Direction::Z),
                        UnitType::Rotation,
                    ),
                ],
                fx: self.unit_to(nodal.fx, UnitType::Force),
                fy: self.unit_to(nodal.fy, UnitType::Force),
                mz: self.unit_to(nodal.mz, UnitType::Moment),
                force_angle: self.unit_to(nodal.angle, UnitType::Angle),
            });
        }
        Ok(result)
    }

    pub fn node_get_mut(&mut self, id: usize) -> Result<&mut Node, ViewModelError> {
        for node in self.nodes_list.iter_mut() {
            if node.id() == id {
                return Ok(node);
            }
        }
        Err(ViewModelError::InvalidMemberId(id))
    }

    pub fn node_new(&mut self, x: f64, y: f64, state_save: bool) -> Result<usize, ViewModelError> {
        for node in self.nodes_list.iter() {
            if node.x() == x && node.y() == y {
                return Err(ViewModelError::NodeAlreadyExists(node.id()));
            }
        }

        if state_save {
            let _ = self.state_save();
        }

        self.nodes_list.sort_by_key(|a| a.id());

        let mut id: usize = 0;
        for node in self.nodes_list.iter() {
            if node.id() == id {
                id += 1;
            } else {
                break;
            }
        }

        self.nodes_list.push(Node::new(id, x, y));

        for (_, case) in self.loadcases_list.iter_mut() {
            _ = case.add_node(id);
        }

        for index in 0..self.members_list.len() {
            let member = self.members_list[index];
            let material = member.material();
            let section = member.section();

            let start = self.node_get(member.node_start());
            let start = match start {
                Ok(value) => value,
                Err(_) => continue,
            };

            let end = self.node_get(member.node_end());
            let end = match end {
                Ok(value) => value,
                Err(_) => continue,
            };

            let x0 = start.x();
            let y0 = start.y();
            let x1 = end.x();
            let y1 = end.y();

            let is_in_line = if x0 == x1 {
                let y_max = y0.max(y1);
                let y_min = y0.min(y1);

                x == x0 && y > y_min && y < y_max
            } else {
                let x_max = x0.max(x1);
                let x_min = x0.min(x1);

                let slope = (y1 - y0) / (x1 - x0);
                let eq = y - y0 - slope * (x - x0);

                eq.abs() < 1e-6 && x > x_min && x < x_max
            };

            let mut member = self.members_list[index];
            if is_in_line {
                let _ = self.member_new((x, y, x1, y1), material, section, false);
                let _ = member.set_node_end(id);
            }
        }

        Ok(id)
    }

    pub fn node_set_pos(
        &mut self,
        id: usize,
        x: f64,
        y: f64,
        state_save: bool,
    ) -> Result<usize, ViewModelError> {
        match self.node_get(id) {
            Ok(_) => {}
            Err(_) => return Err(ViewModelError::InvalidNodeId(id)),
        }

        if state_save {
            let _ = self.state_save();
        }

        let node = self.node_get_mut(id).unwrap();
        node.set_pos(x, y);

        Ok(0)
    }

    pub fn node_set_hinged(
        &mut self,
        id: usize,
        hinge: bool,
        state_save: bool,
    ) -> Result<usize, ViewModelError> {
        match self.node_get(id) {
            Ok(_) => {}
            Err(_) => return Err(ViewModelError::InvalidNodeId(id)),
        }

        if state_save {
            let _ = self.state_save();
        }

        let node = self.node_get_mut(id).unwrap();
        node.set_hinged(hinge);

        Ok(0)
    }

    pub fn node_set_prescribed_displacements(
        &mut self,
        id: usize,
        x: f64,
        y: f64,
        z: f64,
        state_save: bool,
    ) -> Result<usize, ViewModelError> {
        match self.node_get(id) {
            Ok(_) => {}
            Err(_) => return Err(ViewModelError::InvalidNodeId(id)),
        }

        let x = self.unit_from(x, UnitType::Displacement);
        let y = self.unit_from(y, UnitType::Displacement);
        let z = self.unit_from(z, UnitType::Rotation);

        if state_save {
            let _ = self.state_save();
        }

        let node = self.node_get_mut(id).unwrap();
        node.set_prescribed_displacements(x, y, z);

        Ok(0)
    }

    pub fn node_set_springs(
        &mut self,
        id: usize,
        x: f64,
        y: f64,
        z: f64,
        state_save: bool,
    ) -> Result<usize, ViewModelError> {
        match self.node_get(id) {
            Ok(_) => {}
            Err(_) => return Err(ViewModelError::InvalidNodeId(id)),
        }

        let x = self.unit_from(x, UnitType::Spring);
        let y = self.unit_from(y, UnitType::Spring);
        let z = self.unit_from(z, UnitType::TorsionSpring);

        if state_save {
            let _ = self.state_save();
        }

        let node = self.node_get_mut(id).unwrap();
        let result = node.set_springs(x, y, z);
        match result {
            Ok(_) => Ok(0),
            Err(value) => Err(ViewModelError::NegativeSpring(value)),
        }
    }

    pub fn node_set_supports(
        &mut self,
        id: usize,
        x: bool,
        y: bool,
        z: bool,
        angle: f64,
        state_save: bool,
    ) -> Result<usize, ViewModelError> {
        match self.node_get(id) {
            Ok(_) => {}
            Err(_) => return Err(ViewModelError::InvalidNodeId(id)),
        }

        if state_save {
            let _ = self.state_save();
        }

        let node = self.node_get_mut(id).unwrap();
        node.set_supports(x, y, z, angle);

        Ok(0)
    }
}
