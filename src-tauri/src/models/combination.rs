use super::Loadcase;
use std::collections::HashMap;

pub enum CombinationError {
    LoadcaseAlreadyExists(usize),
    InvalidLoadcase(usize),
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CombinationDTO {
    pub id: usize,
    pub name: String,
    pub load_factors: HashMap<usize, f64>,
}

#[derive(serde::Serialize, serde::Deserialize)]
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

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn add_loadcase(&mut self, id: usize) -> Result<(), CombinationError> {
        if let std::collections::hash_map::Entry::Vacant(e) = self.load_factors.entry(id) {
            e.insert(0.0);
            Ok(())
        } else {
            Err(CombinationError::LoadcaseAlreadyExists(id))
        }
    }

    pub fn remove_loadcase(&mut self, id: usize) -> Result<(), CombinationError> {
        match self.load_factors.remove(&id) {
            Some(_) => Ok(()),
            None => Err(CombinationError::InvalidLoadcase(id)),
        }
    }

    pub fn get_factor(&self, id: usize) -> Option<f64> {
        self.load_factors.get(&id).copied()
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
            None => Err(CombinationError::InvalidLoadcase(id)),
        }
    }
}
