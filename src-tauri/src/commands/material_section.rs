use crate::models::{MaterialDTO, SectionDTO, SectionType};
use crate::viewmodel::ViewModel;
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
pub async fn material_delete(
    id: usize,
    state: State<'_, Mutex<ViewModel>>,
) -> Result<(), (String, usize)> {
    let mut vm = state.lock().unwrap();
    let result = vm.material_delete(id);
    match result {
        Ok(_) => Ok(()),
        Err(error) => Err(vm.error_get_dto(error)),
    }
}

#[tauri::command]
pub async fn material_get_dtos(
    state: State<'_, Mutex<ViewModel>>,
) -> Result<Vec<MaterialDTO>, (String, usize)> {
    let vm = state.lock().unwrap();
    let result = vm.material_get_all();
    Ok(result)
}

#[tauri::command]
pub async fn material_new(
    name: String,
    elasticity: f64,
    thermal: f64,
    state: State<'_, Mutex<ViewModel>>,
) -> Result<(), (String, usize)> {
    let mut vm = state.lock().unwrap();
    let result = vm.material_new(&name, elasticity, thermal);
    match result {
        Ok(_) => Ok(()),
        Err(error) => Err(vm.error_get_dto(error)),
    }
}

#[tauri::command]
pub async fn material_update(
    id: usize,
    name: String,
    elasticity: f64,
    thermal: f64,
    state: State<'_, Mutex<ViewModel>>,
) -> Result<(), (String, usize)> {
    let mut vm = state.lock().unwrap();

    let result = vm.material_update(id, &name, elasticity, thermal);
    match result {
        Ok(_) => Ok(()),
        Err(error) => Err(vm.error_get_dto(error)),
    }
}

#[tauri::command]
pub async fn section_delete(
    id: usize,
    state: State<'_, Mutex<ViewModel>>,
) -> Result<(), (String, usize)> {
    let mut vm = state.lock().unwrap();
    let result = vm.section_delete(id);
    match result {
        Ok(_) => Ok(()),
        Err(error) => Err(vm.error_get_dto(error)),
    }
}

#[tauri::command]
pub async fn section_get_dtos(
    state: State<'_, Mutex<ViewModel>>,
) -> Result<Vec<SectionDTO>, (String, usize)> {
    let vm = state.lock().unwrap();
    let result = vm.section_get_all();
    Ok(result)
}

#[tauri::command]
pub async fn section_new(
    name: String,
    section_type: SectionType,
    params: Vec<f64>,
    state: State<'_, Mutex<ViewModel>>,
) -> Result<(), (String, usize)> {
    let mut vm = state.lock().unwrap();
    let result = vm.section_new(&name, section_type, params);
    match result {
        Ok(_) => Ok(()),
        Err(error) => Err(vm.error_get_dto(error)),
    }
}

#[tauri::command]
pub async fn section_update(
    id: usize,
    name: String,
    section_type: SectionType,
    params: Vec<f64>,
    state: State<'_, Mutex<ViewModel>>,
) -> Result<(), (String, usize)> {
    let mut vm = state.lock().unwrap();

    let result = vm.section_update(id, &name, section_type, params);
    match result {
        Ok(_) => Ok(()),
        Err(error) => Err(vm.error_get_dto(error)),
    }
}
