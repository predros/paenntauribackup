use std::collections::HashMap;

use super::{ViewModel, ViewModelError};
use crate::models::{Member, MemberDTO, UnitType};

impl ViewModel {
    pub fn member_delete(&mut self, id: usize, state_save: bool) -> Result<usize, ViewModelError> {
        let index = self.members_list.iter().position(|x| x.id() == id);

        match index {
            Some(value) => {
                if state_save {
                    let _ = self.state_save();
                }

                self.members_list.remove(value);

                for (_, case) in self.loadcases_list.iter_mut() {
                    let _ = case.remove_member(id);
                }

                Ok(0)
            }
            None => Err(ViewModelError::InvalidMemberId(id)),
        }
    }

    pub fn member_get(&self, id: usize) -> Result<&Member, ViewModelError> {
        for member in self.members_list.iter() {
            if member.id() == id {
                return Ok(member);
            }
        }
        Err(ViewModelError::InvalidMemberId(id))
    }

    pub fn member_get_all_dtos(&self) -> Result<Vec<MemberDTO>, ViewModelError> {
        let mut result: Vec<MemberDTO> = vec![];

        let case = self.loadcases_list.get(&self.loadcase_current);
        let case = match case {
            Some(value) => value,
            None => return Err(ViewModelError::InvalidLoadcaseId(self.loadcase_current)),
        };

        for member in self.members_list.iter() {
            let id = member.id();
            let load = case.get_load(id);
            let load = match load {
                Some(value) => value,
                None => return Err(ViewModelError::InvalidMemberId(id)),
            };

            let node_start = self.node_get(member.node_start());
            let node_start = match node_start {
                Ok(value) => value,
                Err(_) => return Err(ViewModelError::InvalidNodeId(member.node_start())),
            };

            let node_end = self.node_get(member.node_end());
            let node_end = match node_end {
                Ok(value) => value,
                Err(_) => return Err(ViewModelError::InvalidNodeId(member.node_start())),
            };

            let mut hinges: HashMap<String, bool> = HashMap::new();
            hinges.insert("start".to_string(), member.hinge_start());
            hinges.insert("end".to_string(), member.hinge_end());

            let delta_x = node_end.x() - node_start.x();
            let delta_y = node_end.y() - node_start.y();

            let mut angle = delta_y.atan2(delta_x).to_degrees();
            if angle < 0.0 {
                angle += 360.0;
            }

            result.push(MemberDTO {
                id,
                x0: node_start.x(),
                y0: node_start.y(),
                x1: node_end.x(),
                y1: node_end.y(),
                length: (delta_x * delta_x + delta_y * delta_y).sqrt(),
                angle,
                hinges,

                material: member.material(),
                section: member.section(),

                qx0: self.unit_to(load.qx0, UnitType::Load),
                qy0: self.unit_to(load.qy0, UnitType::Load),
                qx1: self.unit_to(load.qx1, UnitType::Load),
                qy1: self.unit_to(load.qy1, UnitType::Load),
                is_global: load.is_global,

                t_sup: self.unit_to(load.t_sup, UnitType::Temperature),
                t_inf: self.unit_to(load.t_inf, UnitType::Temperature),
            });
        }
        Ok(result)
    }

    pub fn member_get_mut(&mut self, id: usize) -> Result<&mut Member, ViewModelError> {
        for member in self.members_list.iter_mut() {
            if member.id() == id {
                return Ok(member);
            }
        }
        Err(ViewModelError::InvalidMemberId(id))
    }

    pub fn member_new(
        &mut self,
        coords: (f64, f64, f64, f64),
        material_id: usize,
        section_id: usize,
        state_save: bool,
    ) -> Result<usize, ViewModelError> {
        let (x0, y0, x1, y1) = coords;

        if x0 == x1 && y0 == y1 {
            return Err(ViewModelError::NonDistinctNodes);
        }

        let material = self.materials_list.contains_key(&material_id);
        if !material {
            return Err(ViewModelError::InvalidMaterialId(material_id));
        }

        let section = self.sections_list.contains_key(&section_id);
        if !section {
            return Err(ViewModelError::InvalidSectionId(section_id));
        }

        if state_save {
            let _ = self.state_save();
        }

        let start_id = self.node_new(x0, y0, false);
        let start_id = match start_id {
            Ok(new_id) => new_id,
            Err(ViewModelError::NodeAlreadyExists(id)) => id,
            _ => panic!(), // There is no other implemented error type in that function
        };

        let end_id = self.node_new(x1, y1, false);
        let end_id = match end_id {
            Ok(new_id) => new_id,
            Err(ViewModelError::NodeAlreadyExists(id)) => id,
            _ => panic!(), // There is no other implemented error type in that function
        };

        self.members_list.sort_by_key(|a| a.id());

        let mut id: usize = 0;
        for member in self.members_list.iter() {
            if member.id() == id {
                id += 1;
            } else {
                break;
            }
        }

        let member_new = Member::new(id, start_id, end_id, material_id, section_id).unwrap();

        self.members_list.push(member_new);

        for (_, case) in self.loadcases_list.iter_mut() {
            _ = case.add_member(id);
        }

        Ok(id)
    }

    pub fn member_set_hinges(
        &mut self,
        id: usize,
        start: bool,
        end: bool,
        state_save: bool,
    ) -> Result<usize, ViewModelError> {
        match self.member_get(id) {
            Ok(_) => {}
            Err(_) => return Err(ViewModelError::InvalidMemberId(id)),
        }

        if state_save {
            let _ = self.state_save();
        }

        let member = self.member_get_mut(id).unwrap();
        member.set_hinges(start, end);

        Ok(0)
    }

    pub fn member_set_material(
        &mut self,
        id: usize,
        material_id: usize,
        state_save: bool,
    ) -> Result<usize, ViewModelError> {
        match self.member_get(id) {
            Ok(_) => {}
            Err(_) => return Err(ViewModelError::InvalidMemberId(id)),
        }

        if state_save {
            let _ = self.state_save();
        }

        let member = self.member_get_mut(id).unwrap();
        member.set_material(material_id);
        Ok(0)
    }

    pub fn member_set_section(
        &mut self,
        id: usize,
        section_id: usize,
        state_save: bool,
    ) -> Result<usize, ViewModelError> {
        match self.member_get(id) {
            Ok(_) => {}
            Err(_) => return Err(ViewModelError::InvalidMemberId(id)),
        }

        if state_save {
            let _ = self.state_save();
        }

        let member = self.member_get_mut(id).unwrap();
        member.set_section(section_id);

        Ok(0)
    }
}
