use itertools::Itertools;

use crate::models::{Loadcase, LoadcaseDTO, UnitType};

use super::{ViewModel, ViewModelError};

impl ViewModel {
    pub fn loadcase_get_current(&self) -> usize {
        self.loadcase_current
    }

    pub fn loadcase_get_dtos(&self) -> Vec<LoadcaseDTO> {
        let mut result: Vec<LoadcaseDTO> = vec![];

        for (id, case) in self.loadcases_list.iter() {
            result.push(LoadcaseDTO {
                id: (*id),
                name: case.name(),
            });
        }
        result
    }

    pub fn loadcase_set_current(&mut self, id: usize) -> Result<usize, ViewModelError> {
        if self.loadcases_list.contains_key(&id) {
            self.loadcase_current = id;
            return Ok(0);
        }

        Err(ViewModelError::InvalidLoadcaseId(id))
    }

    pub fn loadcase_new(&mut self, name: String) -> Result<usize, ViewModelError> {
        if name.trim().is_empty() {
            return Err(ViewModelError::EmptyName);
        }

        for (_, case) in self.loadcases_list.iter() {
            if case.name() == name {
                return Err(ViewModelError::NameInUse);
            }
        }

        let loadcase = Loadcase::new(&name, &self.nodes_list, &self.members_list);

        let keys = self.materials_list.keys().sorted();
        let mut id: usize = 0;
        for key in keys {
            if *key == id {
                id += 1;
            } else {
                break;
            }
        }

        self.loadcases_list.insert(id, loadcase);

        for (_, combination) in self.combinations_list.iter_mut() {
            let _ = combination.add_loadcase(id);
        }

        Ok(id)
    }

    pub fn loadcase_update(
        &mut self,
        id: usize,
        new_name: String,
    ) -> Result<usize, ViewModelError> {
        for (_, case) in self.loadcases_list.iter() {
            if case.name() == new_name {
                return Err(ViewModelError::NameInUse);
            }
        }

        let loadcase = self.loadcases_list.get_mut(&id);
        let loadcase = match loadcase {
            Some(value) => value,
            None => return Err(ViewModelError::InvalidLoadcaseId(id)),
        };

        let _ = loadcase.set_name(new_name);

        Ok(0)
    }

    pub fn loadcase_delete(&mut self, id: usize) -> Result<usize, ViewModelError> {
        match self.loadcases_list.remove(&id) {
            Some(_) => {}
            None => return Err(ViewModelError::InvalidLoadcaseId(id)),
        }

        for (_, combination) in self.combinations_list.iter_mut() {
            let _ = combination.remove_loadcase(id);
        }

        Ok(0)
    }

    pub fn node_apply_forces(
        &mut self,
        id: usize,
        fx: f64,
        fy: f64,
        mz: f64,
        angle: f64,
        save_state: bool,
    ) -> Result<usize, ViewModelError> {
        if !self.loadcases_list.contains_key(&self.loadcase_current) {
            return Err(ViewModelError::InvalidLoadcaseId(self.loadcase_current));
        }

        if save_state {
            let _ = self.save_state();
        }

        let fx = self.unit_from(fx, UnitType::Force);
        let fy = self.unit_from(fy, UnitType::Force);
        let mz = self.unit_from(mz, UnitType::Moment);
        let angle = self.unit_from(angle, UnitType::Angle);

        let loadcase = self.loadcases_list.get_mut(&self.loadcase_current).unwrap();
        match loadcase.set_nodal(id, fx, fy, mz, angle) {
            Ok(_) => Ok(0),
            Err(_) => Err(ViewModelError::InvalidNodeId(id)),
        }
    }

    pub fn member_apply_loads(
        &mut self,
        id: usize,
        load: (f64, f64, f64, f64),
        is_global: bool,
        save_state: bool,
    ) -> Result<usize, ViewModelError> {
        if !self.loadcases_list.contains_key(&self.loadcase_current) {
            return Err(ViewModelError::InvalidLoadcaseId(self.loadcase_current));
        }

        let qx0 = self.unit_from(load.0, UnitType::Load);
        let qy0 = self.unit_from(load.1, UnitType::Load);
        let qx1 = self.unit_from(load.2, UnitType::Load);
        let qy1 = self.unit_from(load.3, UnitType::Load);

        if save_state {
            let _ = self.save_state();
        }

        let loadcase = self.loadcases_list.get_mut(&self.loadcase_current).unwrap();
        match loadcase.set_load(id, qx0, qy0, qx1, qy1, is_global) {
            Ok(_) => Ok(0),
            Err(_) => Err(ViewModelError::InvalidMemberId(id)),
        }
    }

    pub fn member_apply_temperatures(
        &mut self,
        id: usize,
        t_sup: f64,
        t_inf: f64,
        save_state: bool,
    ) -> Result<usize, ViewModelError> {
        if !self.loadcases_list.contains_key(&self.loadcase_current) {
            return Err(ViewModelError::InvalidLoadcaseId(self.loadcase_current));
        }

        let t_sup = self.unit_from(t_sup, UnitType::Temperature);
        let t_inf = self.unit_from(t_inf, UnitType::Temperature);

        if save_state {
            let _ = self.save_state();
        }

        let loadcase = self.loadcases_list.get_mut(&self.loadcase_current).unwrap();

        match loadcase.set_temperature(id, t_sup, t_inf) {
            Ok(_) => Ok(0),
            Err(_) => Err(ViewModelError::InvalidMemberId(id)),
        }
    }
}
