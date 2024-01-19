use std::{collections::HashMap, fs::canonicalize, fs::File, io::Write, path::Path};

use crate::models::{Loadcase, Material, Member, Node, Section, Settings};

use super::{Combination, ViewModel, ViewModelError};

#[derive(serde::Serialize, serde::Deserialize)]
struct IOResult {
    pub nodes: Vec<Node>,
    pub members: Vec<Member>,
    pub materials: HashMap<usize, Material>,
    pub sections: HashMap<usize, Section>,
    pub loadcases: HashMap<usize, Loadcase>,
    pub loadcase_current: usize,
}

impl ViewModel {
    pub fn get_current_file(&self) -> Result<String, ViewModelError> {
        Ok(self.current_file.clone())
    }

    pub fn unsaved_changes(&self) -> Result<bool, ViewModelError> {
        Ok(self.unsaved_changes)
    }

    pub fn save_file(&mut self, file_path: &String) -> Result<(), ViewModelError> {
        let file = File::create(file_path);

        let mut file = match file {
            Ok(value) => value,
            Err(_) => return Err(ViewModelError::FailedToWrite),
        };

        let nodes_json = serde_json::to_string(&self.nodes_list);
        let nodes_json = match nodes_json {
            Ok(value) => value,
            Err(_) => return Err(ViewModelError::FailedToSerialize),
        };

        let members_json: Result<String, serde_json::Error> =
            serde_json::to_string(&self.members_list);
        let members_json = match members_json {
            Ok(value) => value,
            Err(_) => return Err(ViewModelError::FailedToSerialize),
        };

        let materials_json = serde_json::to_string(&self.materials_list);
        let materials_json = match materials_json {
            Ok(value) => value,
            Err(_) => return Err(ViewModelError::FailedToSerialize),
        };

        let sections_json = serde_json::to_string(&self.sections_list);
        let sections_json = match sections_json {
            Ok(value) => value,
            Err(_) => return Err(ViewModelError::FailedToSerialize),
        };

        let loadcases_json = serde_json::to_string(&self.loadcases_list);
        let loadcases_json = match loadcases_json {
            Ok(value) => value,
            Err(_) => return Err(ViewModelError::FailedToSerialize),
        };

        let loadcase_current = self.loadcase_current.to_string();

        let result = format!(
            "{{\n
            \"nodes\": {},
            \"members\": {},
            \"materials\": {},
            \"sections\": {},
            \"loadcases\": {},
            \"loadcase_current\": {}
            }}",
            nodes_json,
            members_json,
            materials_json,
            sections_json,
            loadcases_json,
            loadcase_current
        );

        let write_result = file.write(result.as_bytes());
        match write_result {
            Ok(_) => {}
            Err(_) => return Err(ViewModelError::FailedToWrite),
        }

        let absolute = canonicalize(Path::new(&file_path)).unwrap();
        self.current_file = absolute.to_str().unwrap().to_string();
        self.unsaved_changes = false;

        Ok(())
    }

    pub fn open_file(&mut self, file_path: &String) -> Result<(), ViewModelError> {
        let file = std::fs::read_to_string(file_path);
        let file = match file {
            Ok(value) => value,
            Err(_) => return Err(ViewModelError::FailedToRead),
        };

        let result: Result<IOResult, _> = serde_json::from_str(&file);
        let result = match result {
            Ok(value) => value,
            Err(_) => return Err(ViewModelError::FailedToSerialize),
        };

        self.nodes_list = result.nodes;
        self.members_list = result.members;
        self.materials_list = result.materials;
        self.sections_list = result.sections;
        self.loadcases_list = result.loadcases;
        self.loadcase_current = result.loadcase_current;

        let absolute = canonicalize(Path::new(&file_path)).unwrap();
        self.current_file = absolute.to_str().unwrap().to_string();
        self.unsaved_changes = false;

        self.redo_history.clear();
        self.undo_history.clear();

        Ok(())
    }

    pub fn new_file(&mut self) -> Result<(), ViewModelError> {
        self.nodes_list.clear();
        self.members_list.clear();
        self.materials_list.clear();
        self.sections_list.clear();
        self.loadcases_list.clear();

        self.materials_list
            .insert(0, Material::new("Steel", 20000.0, 12e-6).unwrap());
        self.materials_list
            .insert(1, Material::new("Concrete", 2400.0, 1e-5).unwrap());
        self.sections_list
            .insert(0, Section::new_circle("D20", 20.0, 0.0).unwrap());
        self.loadcases_list.insert(
            0,
            Loadcase::new("Caso 01", &self.nodes_list, &self.members_list),
        );

        self.loadcases_list.insert(
            1,
            Loadcase::new("Caso 02", &self.nodes_list, &self.members_list),
        );
        self.loadcase_current = 0;

        self.combinations_list
            .insert(0, Combination::new("Comb. 01", &self.loadcases_list));

        let _ = self.combination_set_specific_factor(0, 0, 1.0);
        let _ = self.combination_set_specific_factor(0, 1, 1.0);

        self.current_file = "".to_string();
        self.unsaved_changes = false;

        self.redo_history.clear();
        self.undo_history.clear();

        Ok(())
    }

    pub fn load_settings(&mut self) -> Result<Settings, ViewModelError> {
        let file = std::fs::read_to_string("./paennsettings");
        let file = match file {
            Ok(value) => value,
            Err(_) => return Err(ViewModelError::FailedToRead),
        };

        let result: Result<Settings, _> = serde_json::from_str(&file);
        match result {
            Ok(value) => Ok(value),
            Err(_) => Err(ViewModelError::FailedToSerialize),
        }
    }

    pub fn save_settings(&self) -> Result<(), ViewModelError> {
        let file = File::create("./paennsettings");
        let mut file = match file {
            Ok(value) => value,
            Err(_) => return Err(ViewModelError::FailedToWrite),
        };

        let result = serde_json::to_string(&self.settings);
        let result = match result {
            Ok(value) => value,
            Err(_) => return Err(ViewModelError::FailedToSerialize),
        };

        let write_result = file.write(result.as_bytes());
        match write_result {
            Ok(_) => Ok(()),
            Err(_) => Err(ViewModelError::FailedToWrite),
        }
    }
}
