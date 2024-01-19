use super::{ViewModel, ViewModelError};
use crate::models::{Combination, CombinationDTO};
use itertools::Itertools;
use std::collections::HashMap;

impl ViewModel {
    pub fn combination_get_dtos(&self) -> Vec<CombinationDTO> {
        let mut result: Vec<CombinationDTO> = vec![];

        for (id, combination) in self.combinations_list.iter() {
            result.push(CombinationDTO {
                id: *id,
                name: combination.name(),
            });
        }
        result
    }

    pub fn combination_new(&mut self, name: String) -> Result<usize, ViewModelError> {
        if name.trim().is_empty() {
            return Err(ViewModelError::EmptyName);
        }

        for (_, case) in self.combinations_list.iter() {
            if case.name() == name {
                return Err(ViewModelError::NameInUse);
            }
        }

        let combination = Combination::new(&name, &self.loadcases_list);

        let keys = self.materials_list.keys().sorted();
        let mut id: usize = 0;
        for key in keys {
            if *key == id {
                id += 1;
            } else {
                break;
            }
        }

        self.combinations_list.insert(id, combination);

        Ok(id)
    }

    pub fn combination_update(
        &mut self,
        id: usize,
        new_name: String,
    ) -> Result<usize, ViewModelError> {
        if new_name.trim().is_empty() {
            return Err(ViewModelError::EmptyName);
        }

        for (_, case) in self.combinations_list.iter() {
            if case.name() == new_name {
                return Err(ViewModelError::NameInUse);
            }
        }

        let combination = match self.combinations_list.get_mut(&id) {
            Some(value) => value,
            None => return Err(ViewModelError::InvalidCombinationId(id)),
        };

        let _ = combination.set_name(&new_name);

        Ok(0)
    }

    pub fn combination_get_factors(
        &self,
        id: usize,
    ) -> Result<HashMap<usize, f64>, ViewModelError> {
        let combination = match self.combinations_list.get(&id) {
            Some(value) => value,
            None => return Err(ViewModelError::InvalidCombinationId(id)),
        };

        Ok(combination.get_all_factors())
    }

    pub fn combination_set_factors(
        &mut self,
        combination_id: usize,
        factors: &HashMap<usize, f64>,
    ) -> Result<usize, ViewModelError> {
        let combination = match self.combinations_list.get_mut(&combination_id) {
            Some(value) => value,
            None => return Err(ViewModelError::InvalidCombinationId(combination_id)),
        };

        for (id, factor) in factors.iter() {
            let _ = combination.set_factor(*id, *factor);
        }

        Ok(0)
    }

    pub fn combination_set_specific_factor(
        &mut self,
        combination_id: usize,
        loadcase_id: usize,
        factor: f64,
    ) -> Result<usize, ViewModelError> {
        let combination = match self.combinations_list.get_mut(&combination_id) {
            Some(value) => value,
            None => return Err(ViewModelError::InvalidCombinationId(combination_id)),
        };

        match combination.set_factor(loadcase_id, factor) {
            Ok(_) => Ok(0),
            Err(_) => Err(ViewModelError::InvalidLoadcaseId(loadcase_id)),
        }
    }
}
