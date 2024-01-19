use crate::{backend::AnalysisResults, models::*};
use itertools::Itertools;
use std::collections::HashMap;

use crate::backend::{linear_analysis, AnalysisError};

mod combinations;
mod io;
mod loadcases;
mod members;
mod nodes;

#[derive(Debug)]
pub enum ViewModelError {
    InvalidCombinationId(usize),
    InvalidNodeId(usize),
    InvalidMemberId(usize),
    InvalidMaterialId(usize),
    InvalidSectionId(usize),
    InvalidLoadcaseId(usize),

    EmptyName,
    NameInUse,
    InvalidDimension,

    NodeAlreadyExists(usize),
    NodeInUse,
    NegativeSpring(Direction),

    NonDistinctNodes,
    MemberAlreadyExists(usize),

    NonPositiveValue,

    MaterialInUse,
    SectionInUse,

    UnstableStructure,

    FailedToWrite,
    FailedToRead,
    FailedToSerialize,
    FailedToDeserialize,
}

pub struct ViewModel {
    current_file: String,
    unsaved_changes: bool,
    nodes_list: Vec<Node>,
    members_list: Vec<Member>,
    materials_list: HashMap<usize, Material>,
    sections_list: HashMap<usize, Section>,

    loadcases_list: HashMap<usize, Loadcase>,
    loadcase_current: usize,

    combinations_list: HashMap<usize, Combination>,

    undo_history: Vec<(String, String, String)>,
    redo_history: Vec<(String, String, String)>,

    settings: Settings,
}

impl Default for ViewModel {
    fn default() -> Self {
        Self::new()
    }
}

impl ViewModel {
    pub fn new() -> ViewModel {
        let nodes: Vec<Node> = vec![];
        let members: Vec<Member> = vec![];
        let materials: HashMap<usize, Material> = HashMap::new();
        let sections: HashMap<usize, Section> = HashMap::new();
        let loadcases: HashMap<usize, Loadcase> = HashMap::new();

        let settings = Settings {
            dark_theme: false,
            locale: "pt-BR".to_string(),
            grid_spacing: [100, 100],
            units: UnitSettings {
                length: UnitLength::Centimeter,
                force: UnitForce::KiloNewton,
                angle: UnitAngle::Degree,
                temperature: UnitTemperature::Celsius,
                moment: (UnitForce::KiloNewton, UnitLength::Centimeter),
                load: (UnitForce::KiloNewton, UnitLength::Centimeter),
                displacement: UnitLength::Centimeter,
                rotation: UnitAngle::Degree,
                spring: (UnitForce::KiloNewton, UnitLength::Centimeter),
                torsion_spring: (
                    UnitForce::KiloNewton,
                    UnitLength::Centimeter,
                    UnitAngle::Degree,
                ),
                elasticity: UnitStress::MegaPascal,
                thermal: UnitTemperature::Celsius,
                inertia: UnitLength::Centimeter,
                area: UnitLength::Centimeter,
                dimension: UnitLength::Centimeter,
            },
            unit_precision: UnitPrecision {
                length: (2, false),
                force: (2, false),
                angle: (1, false),
                temperature: (1, false),
                moment: (2, false),
                load: (2, false),
                displacement: (3, true),
                rotation: (3, true),
                spring: (2, false),
                torsion_spring: (2, false),
                elasticity: (1, true),
                thermal: (2, true),
                inertia: (3, true),
                area: (2, false),
                dimension: (1, false),
            },
        };

        let mut vm = ViewModel {
            current_file: "".to_string(),
            unsaved_changes: false,
            nodes_list: nodes,
            members_list: members,
            materials_list: materials,
            sections_list: sections,
            loadcases_list: loadcases,
            loadcase_current: 0,
            combinations_list: HashMap::new(),

            undo_history: vec![],
            redo_history: vec![],

            settings,
        };

        let settings = vm.load_settings();
        match settings {
            Ok(value) => vm.settings = value,
            Err(_) => {
                let _ = vm.save_settings();
            }
        }

        let _ = vm.new_file();

        vm
    }

    pub fn save_state(&mut self) -> Result<(), ViewModelError> {
        let nodes_current = serde_json::to_string(&self.nodes_list);
        let nodes_current = match nodes_current {
            Ok(value) => value,
            Err(_) => return Err(ViewModelError::FailedToSerialize),
        };

        let members_current = serde_json::to_string(&self.members_list);
        let members_current = match members_current {
            Ok(value) => value,
            Err(_) => return Err(ViewModelError::FailedToSerialize),
        };

        let loadcases_current = serde_json::to_string(&self.loadcases_list);
        let loadcases_current = match loadcases_current {
            Ok(value) => value,
            Err(_) => return Err(ViewModelError::FailedToSerialize),
        };

        self.unsaved_changes = true;

        self.undo_history
            .push((nodes_current, members_current, loadcases_current));
        self.redo_history.clear();

        Ok(())
    }

    pub fn undo_state(&mut self) -> Result<(), ViewModelError> {
        let previous = self.undo_history.pop();
        let previous = match previous {
            Some(value) => value,
            None => return Ok(()),
        };

        let nodes_current = serde_json::to_string(&self.nodes_list);
        let nodes_current = match nodes_current {
            Ok(value) => value,
            Err(_) => return Err(ViewModelError::FailedToSerialize),
        };

        let members_current = serde_json::to_string(&self.members_list);
        let members_current = match members_current {
            Ok(value) => value,
            Err(_) => return Err(ViewModelError::FailedToSerialize),
        };

        let loadcases_current = serde_json::to_string(&self.loadcases_list);
        let loadcases_current = match loadcases_current {
            Ok(value) => value,
            Err(_) => return Err(ViewModelError::FailedToSerialize),
        };

        let nodes_previous: Result<Vec<Node>, _> = serde_json::from_str(&previous.0);
        let nodes_previous = match nodes_previous {
            Ok(value) => value,
            Err(_) => return Err(ViewModelError::FailedToDeserialize),
        };

        let members_previous: Result<Vec<Member>, _> = serde_json::from_str(&previous.1);
        let members_previous = match members_previous {
            Ok(value) => value,
            Err(_) => return Err(ViewModelError::FailedToDeserialize),
        };

        let loadcases_previous: Result<HashMap<usize, Loadcase>, _> =
            serde_json::from_str(&previous.2);
        let loadcases_previous = match loadcases_previous {
            Ok(value) => value,
            Err(_) => return Err(ViewModelError::FailedToDeserialize),
        };

        self.nodes_list = nodes_previous;
        self.members_list = members_previous;
        self.loadcases_list = loadcases_previous;

        self.unsaved_changes = true;

        self.redo_history
            .push((nodes_current, members_current, loadcases_current));

        Ok(())
    }

    pub fn redo_state(&mut self) -> Result<(), ViewModelError> {
        let next = self.redo_history.pop();
        let next = match next {
            Some(value) => value,
            None => return Ok(()),
        };

        let nodes_current = serde_json::to_string(&self.nodes_list);
        let nodes_current = match nodes_current {
            Ok(value) => value,
            Err(_) => return Err(ViewModelError::FailedToSerialize),
        };

        let members_current = serde_json::to_string(&self.members_list);
        let members_current = match members_current {
            Ok(value) => value,
            Err(_) => return Err(ViewModelError::FailedToSerialize),
        };

        let loadcases_current = serde_json::to_string(&self.loadcases_list);
        let loadcases_current = match loadcases_current {
            Ok(value) => value,
            Err(_) => return Err(ViewModelError::FailedToSerialize),
        };

        let nodes_next: Result<Vec<Node>, _> = serde_json::from_str(&next.0);
        let nodes_next = match nodes_next {
            Ok(value) => value,
            Err(_) => return Err(ViewModelError::FailedToDeserialize),
        };

        let members_next: Result<Vec<Member>, _> = serde_json::from_str(&next.1);
        let members_next = match members_next {
            Ok(value) => value,
            Err(_) => return Err(ViewModelError::FailedToDeserialize),
        };

        let loadcases_next: Result<HashMap<usize, Loadcase>, _> = serde_json::from_str(&next.2);
        let loadcases_next = match loadcases_next {
            Ok(value) => value,
            Err(_) => return Err(ViewModelError::FailedToDeserialize),
        };

        self.nodes_list = nodes_next;
        self.members_list = members_next;
        self.loadcases_list = loadcases_next;

        self.unsaved_changes = true;

        self.undo_history
            .push((nodes_current, members_current, loadcases_current));

        Ok(())
    }

    pub fn history_len(&self) -> (usize, usize) {
        (self.undo_history.len(), self.redo_history.len())
    }

    pub fn settings_get(&self) -> Result<Settings, ViewModelError> {
        Ok(self.settings.clone())
    }

    pub fn settings_set(&mut self, settings: Settings) -> Result<(), ViewModelError> {
        self.settings = settings.clone();

        self.save_settings()
    }

    pub fn run_linear_analysis(&self) -> Result<AnalysisResults, ViewModelError> {
        let result = linear_analysis(
            &self.nodes_list,
            &self.members_list,
            &self.materials_list,
            &self.sections_list,
            &self.loadcases_list,
            &self.combinations_list,
        );
        let mut result = match result {
            Ok(value) => value,
            Err(AnalysisError::InvalidLoadcaseId(id)) => {
                return Err(ViewModelError::InvalidLoadcaseId(id))
            }
            Err(AnalysisError::InvalidMaterialId(id)) => {
                return Err(ViewModelError::InvalidMaterialId(id))
            }
            Err(AnalysisError::InvalidMemberId(id)) => {
                return Err(ViewModelError::InvalidMemberId(id))
            }
            Err(AnalysisError::InvalidNodeId(id)) => return Err(ViewModelError::InvalidNodeId(id)),
            Err(AnalysisError::InvalidSectionId(id)) => {
                return Err(ViewModelError::InvalidSectionId(id))
            }
            Err(AnalysisError::MemberNotInLoadcase(_, id)) => {
                return Err(ViewModelError::InvalidMemberId(id))
            }
            Err(AnalysisError::NodeNotInLoadcase(_, id)) => {
                return Err(ViewModelError::InvalidNodeId(id))
            }
            Err(AnalysisError::UnstableStructure) => return Err(ViewModelError::UnstableStructure),
        };

        for (_, case) in result.loadcase_results.iter_mut() {
            for member in case.iter_mut() {
                member.max_moment.1 = self.unit_to(member.max_moment.1, UnitType::Moment);
                member.min_moment.1 = self.unit_to(member.min_moment.1, UnitType::Moment);
                member.vert_normal.1 = self.unit_to(member.vert_normal.1, UnitType::Force);
                member.vert_shear.1 = self.unit_to(member.vert_shear.1, UnitType::Force);

                member.moment = member
                    .moment
                    .iter()
                    .map(|x| self.unit_to(*x, UnitType::Moment))
                    .collect();
                member.shear = member
                    .shear
                    .iter()
                    .map(|x| self.unit_to(*x, UnitType::Force))
                    .collect();
                member.normal = member
                    .normal
                    .iter()
                    .map(|x| self.unit_to(*x, UnitType::Force))
                    .collect();
            }
        }

        for (_, case) in result.combination_results.iter_mut() {
            for member in case.iter_mut() {
                member.max_moment.1 = self.unit_to(member.max_moment.1, UnitType::Moment);
                member.min_moment.1 = self.unit_to(member.min_moment.1, UnitType::Moment);
                member.vert_normal.1 = self.unit_to(member.vert_normal.1, UnitType::Force);
                member.vert_shear.1 = self.unit_to(member.vert_shear.1, UnitType::Force);

                member.moment = member
                    .moment
                    .iter()
                    .map(|x| self.unit_to(*x, UnitType::Moment))
                    .collect();
                member.shear = member
                    .shear
                    .iter()
                    .map(|x| self.unit_to(*x, UnitType::Force))
                    .collect();
                member.normal = member
                    .normal
                    .iter()
                    .map(|x| self.unit_to(*x, UnitType::Force))
                    .collect();
            }
        }

        for (_, case) in result.loadcase_reactions.iter_mut() {
            for node in case.iter_mut() {
                node.rx = self.unit_to(node.rx, UnitType::Force);
                node.ry = self.unit_to(node.ry, UnitType::Force);
                node.mz = self.unit_to(node.mz, UnitType::Moment);
            }
        }

        for (_, case) in result.combination_reactions.iter_mut() {
            for node in case.iter_mut() {
                node.rx = self.unit_to(node.rx, UnitType::Force);
                node.ry = self.unit_to(node.ry, UnitType::Force);
                node.mz = self.unit_to(node.mz, UnitType::Moment);
            }
        }

        Ok(result)
    }

    pub fn material_get(&self, id: usize) -> Result<&Material, ViewModelError> {
        let material = self.materials_list.get(&id);
        match material {
            Some(value) => Ok(value),
            None => Err(ViewModelError::InvalidMaterialId(id)),
        }
    }

    pub fn material_get_all(&self) -> Vec<MaterialDTO> {
        let mut result: Vec<MaterialDTO> = vec![];

        for (id, value) in self.materials_list.iter() {
            result.push(MaterialDTO {
                id: *id,
                name: value.name(),
                elasticity: self.unit_to(value.elasticity(), UnitType::Elasticity),
                thermal: self.unit_to(value.thermal(), UnitType::Thermal),
            });
        }
        result
    }

    pub fn section_get(&self, id: usize) -> Result<&Section, ViewModelError> {
        let section = self.sections_list.get(&id);
        match section {
            Some(value) => Ok(value),
            None => Err(ViewModelError::InvalidSectionId(id)),
        }
    }

    pub fn section_get_all(&self) -> Vec<SectionDTO> {
        let mut result: Vec<SectionDTO> = vec![];

        for (id, value) in self.sections_list.iter() {
            let params = value.params();

            let params = if value.section_type() == SectionType::Generic {
                vec![
                    self.unit_from(params[0], UnitType::Inertia),
                    self.unit_from(params[1], UnitType::Area),
                    self.unit_from(params[2], UnitType::Dimension),
                    self.unit_from(params[3], UnitType::Dimension),
                ]
            } else {
                params
                    .iter()
                    .map(|x| self.unit_from(*x, UnitType::Dimension))
                    .collect()
            };

            result.push(SectionDTO {
                id: *id,
                name: value.name(),
                section_type: value.section_type(),
                inertia: self.unit_to(value.inertia(), UnitType::Inertia),
                area: self.unit_to(value.area(), UnitType::Area),
                y_sup: self.unit_to(value.y_sup(), UnitType::Dimension),
                y_inf: self.unit_to(value.y_inf(), UnitType::Dimension),
                params,
            })
        }
        result
    }

    pub fn material_new(
        &mut self,
        name: &str,
        elasticity: f64,
        thermal: f64,
    ) -> Result<usize, ViewModelError> {
        for (_, value) in self.materials_list.iter() {
            if value.name() == name {
                return Err(ViewModelError::NameInUse);
            }
        }

        let elasticity = self.unit_from(elasticity, UnitType::Elasticity);
        let thermal = self.unit_from(thermal, UnitType::Thermal);

        let material = Material::new(name, elasticity, thermal);
        let material = match material {
            Ok(value) => value,
            Err(MaterialError::EmptyName) => return Err(ViewModelError::EmptyName),
            Err(_) => return Err(ViewModelError::NonPositiveValue),
        };

        let keys = self.materials_list.keys().sorted();
        let mut id: usize = 0;
        for key in keys {
            if *key == id {
                id += 1;
            } else {
                break;
            }
        }
        self.materials_list.insert(id, material);
        Ok(id)
    }

    pub fn material_update(
        &mut self,
        id: usize,
        name: &str,
        elasticity: f64,
        thermal: f64,
    ) -> Result<usize, ViewModelError> {
        for (key, value) in self.materials_list.iter() {
            if value.name() == name && *key != id {
                return Err(ViewModelError::NameInUse);
            }
        }

        let elasticity = self.unit_from(elasticity, UnitType::Elasticity);
        let thermal = self.unit_from(thermal, UnitType::Thermal);

        let material = self.materials_list.get_mut(&id);
        let material = match material {
            Some(value) => value,
            None => return Err(ViewModelError::InvalidMaterialId(id)),
        };

        let _ = material.set_name(name);
        let _ = material.set_elasticity(elasticity);
        let _ = material.set_thermal(thermal);

        Ok(0)
    }

    pub fn material_delete(&mut self, id: usize) -> Result<usize, ViewModelError> {
        for member in self.members_list.iter() {
            if member.material() == id {
                return Err(ViewModelError::MaterialInUse);
            }
        }

        match self.materials_list.remove(&id) {
            Some(_) => Ok(0),
            None => Err(ViewModelError::InvalidMaterialId(id)),
        }
    }

    pub fn section_new(
        &mut self,
        name: &str,
        section_type: SectionType,
        params: Vec<f64>,
    ) -> Result<usize, ViewModelError> {
        for (_, value) in self.sections_list.iter() {
            if value.name() == name {
                return Err(ViewModelError::NameInUse);
            }
        }

        let params = if section_type == SectionType::Generic {
            vec![
                self.unit_from(params[0], UnitType::Inertia),
                self.unit_from(params[1], UnitType::Area),
                self.unit_from(params[2], UnitType::Dimension),
                self.unit_from(params[3], UnitType::Dimension),
            ]
        } else {
            params
                .iter()
                .map(|x| self.unit_from(*x, UnitType::Dimension))
                .collect()
        };

        let section = match section_type {
            SectionType::AsymmetricW => return Ok(0),
            SectionType::SymmetricW => return Ok(0),
            SectionType::Circle => Section::new_circle(name, params[0], params[1]),
            SectionType::Generic => {
                Section::new_generic(name, params[0], params[1], params[2], params[3])
            }
            SectionType::Rectangle => {
                Section::new_rect(name, params[0], params[1], params[2], params[3])
            }
        };

        let section = match section {
            Ok(value) => value,
            Err(SectionError::EmptyName) => return Err(ViewModelError::EmptyName),
            Err(SectionError::LargerInnerDimension) => {
                return Err(ViewModelError::InvalidDimension)
            }
            Err(SectionError::NegativeArea) => return Err(ViewModelError::NonPositiveValue),
            Err(SectionError::NegativeDimension) => return Err(ViewModelError::NonPositiveValue),
            Err(SectionError::NegativeInertia) => return Err(ViewModelError::NonPositiveValue),
        };

        let keys = self.sections_list.keys().sorted();
        let mut id: usize = 0;
        for key in keys {
            if *key == id {
                id += 1;
            } else {
                break;
            }
        }
        self.sections_list.insert(id, section);
        Ok(id)
    }

    pub fn section_update(
        &mut self,
        id: usize,
        name: &str,
        section_type: SectionType,
        params: Vec<f64>,
    ) -> Result<usize, ViewModelError> {
        for (key, value) in self.sections_list.iter() {
            if value.name() == name && *key != id {
                return Err(ViewModelError::NameInUse);
            }
        }

        let params = if section_type == SectionType::Generic {
            vec![
                self.unit_from(params[0], UnitType::Inertia),
                self.unit_from(params[1], UnitType::Area),
                self.unit_from(params[2], UnitType::Dimension),
                self.unit_from(params[3], UnitType::Dimension),
            ]
        } else {
            params
                .iter()
                .map(|x| self.unit_from(*x, UnitType::Dimension))
                .collect()
        };

        let section = self.sections_list.get_mut(&id);
        let section = match section {
            Some(value) => value,
            None => return Err(ViewModelError::InvalidSectionId(id)),
        };

        match section_type {
            SectionType::AsymmetricW => return Ok(0),
            SectionType::SymmetricW => return Ok(0),
            SectionType::Circle => {
                let _ = section.set_circle(params[0], params[1]);
            }
            SectionType::Generic => {
                let _ = section.set_generic(params[0], params[1], params[2], params[3]);
            }
            SectionType::Rectangle => {
                let _ = section.set_rect(params[0], params[1], params[2], params[3]);
            }
        }

        Ok(0)
    }

    pub fn section_delete(&mut self, id: usize) -> Result<usize, ViewModelError> {
        for member in self.members_list.iter() {
            if member.section() == id {
                return Err(ViewModelError::SectionInUse);
            }
        }

        match self.sections_list.remove(&id) {
            Some(_) => Ok(0),
            None => Err(ViewModelError::InvalidSectionId(id)),
        }
    }

    pub fn error_to_string(&self, error: ViewModelError) -> (String, usize) {
        match error {
            ViewModelError::EmptyName => ("alerts.emptyName".to_string(), 0),
            ViewModelError::InvalidCombinationId(id) => {
                ("alerts.invalidCombinationId".to_string(), id)
            }
            ViewModelError::InvalidLoadcaseId(id) => ("alerts.invalidLoadcaseId".to_string(), id),
            ViewModelError::InvalidMaterialId(id) => ("alerts.invalidMaterialId".to_string(), id),
            ViewModelError::InvalidMemberId(id) => ("alerts.invalidMemberId".to_string(), id),
            ViewModelError::InvalidNodeId(id) => ("alerts.invalidNodeId".to_string(), id),
            ViewModelError::InvalidSectionId(id) => ("alerts.invalidSectionId".to_string(), id),
            ViewModelError::MaterialInUse => ("alerts.materialInUse".to_string(), 0),
            ViewModelError::MemberAlreadyExists(id) => {
                ("alerts.memberAlreadyExists".to_string(), id)
            }
            ViewModelError::NameInUse => ("alerts.nameInUse".to_string(), 0),
            ViewModelError::NegativeSpring(_) => ("alerts.negativeSpring".to_string(), 0),
            ViewModelError::NodeAlreadyExists(id) => ("alerts.nodeAlreadyExists".to_string(), id),
            ViewModelError::NodeInUse => ("alert.nodeInUse".to_string(), 0),
            ViewModelError::NonDistinctNodes => ("alert.distinctNodes".to_string(), 0),
            ViewModelError::NonPositiveValue => ("alert.nonPositiveValue".to_string(), 0),
            ViewModelError::SectionInUse => ("alert.sectionInUse".to_string(), 0),
            ViewModelError::UnstableStructure => ("alert.unstableStructure".to_string(), 0),
            ViewModelError::FailedToRead => ("alert.failedToRead".to_string(), 0),
            ViewModelError::FailedToWrite => ("alert.failedToWrite".to_string(), 0),
            ViewModelError::FailedToSerialize => ("alert.failedToSerialize".to_string(), 0),
            ViewModelError::FailedToDeserialize => ("alert.failedToDeserialize".to_string(), 0),
            ViewModelError::InvalidDimension => ("alert.invalidDimension".to_string(), 0),
        }
    }

    pub fn unit_to(&self, value: f64, unit_type: UnitType) -> f64 {
        self.settings.units.convert_to(value, unit_type)
    }

    pub fn unit_from(&self, value: f64, unit_type: UnitType) -> f64 {
        self.settings.units.convert_from(value, unit_type)
    }
}
