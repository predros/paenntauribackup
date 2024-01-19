use super::Loadcase;
use std::collections::HashMap;

pub enum CombinationError {
    LoadcaseAlreadyExists(usize),
    InvalidLoadcase(usize),
}

#[derive(serde::Serialize)]
pub struct CombinationDTO {
    pub id: usize,
    pub name: String,
}

pub struct Combination {
    name: String,
    load_factors: HashMap<usize, f64>,
}

impl Combination {
    pub fn new(name: &str, loadcases_list: &HashMap<usize, Loadcase>) -> Combination {
        let mut result = Combination {
            name: name.to_string(),
            load_factors: HashMap::new(),
        };

        for (id, _) in loadcases_list.iter() {
            result.load_factors.insert(*id, 0.0);
        }

        result
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: &str) -> () {
        self.name = name.to_string();
    }

    pub fn add_loadcase(&mut self, id: usize) -> Result<(), CombinationError> {
        if self.load_factors.contains_key(&id) {
            return Err(CombinationError::LoadcaseAlreadyExists(id));
        } else {
            self.load_factors.insert(id, 0.0);
            return Ok(());
        }
    }

    pub fn remove_loadcase(&mut self, id: usize) -> Result<(), CombinationError> {
        match self.load_factors.remove(&id) {
            Some(_) => Ok(()),
            None => Err(CombinationError::InvalidLoadcase(id)),
        }
    }

    pub fn get_factor(&self, id: usize) -> Option<f64> {
        match self.load_factors.get(&id) {
            Some(value) => Some(*value),
            None => None,
        }
    }

    pub fn get_all_factors(&self) -> HashMap<usize, f64> {
        self.load_factors.clone()
    }

    pub fn set_factor(&mut self, id: usize, factor: f64) -> Result<(), CombinationError> {
        let original_factor = self.load_factors.get_mut(&id);
        match original_factor {
            Some(value) => {
                *value = factor;
                Ok(())
            }
            None => return Err(CombinationError::InvalidLoadcase(id)),
        }
    }
}
