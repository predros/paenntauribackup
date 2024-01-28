use crate::models::LoadcaseDTO;
use crate::viewmodel::ViewModel;
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
pub async fn loadcase_delete(
    id: usize,
    state: State<'_, Mutex<ViewModel>>,
) -> Result<usize, (String, usize)> {
    let mut vm = state.lock().unwrap();
    match vm.loadcase_delete(id) {
        Ok(_) => Ok(0),
        Err(error) => Err(vm.error_get_dto(error)),
    }
}

#[tauri::command]
pub async fn loadcase_get_current(
    state: State<'_, Mutex<ViewModel>>,
) -> Result<usize, (String, usize)> {
    let vm = state.lock().unwrap();
    let result = vm.loadcase_get_current();

    Ok(result)
}

#[tauri::command]
pub async fn loadcase_get_dtos(
    state: State<'_, Mutex<ViewModel>>,
) -> Result<Vec<LoadcaseDTO>, (String, usize)> {
    let vm = state.lock().unwrap();
    let result = vm.loadcase_get_dtos();

    Ok(result)
}

#[tauri::command]
pub async fn loadcase_new(
    name: String,
    state: State<'_, Mutex<ViewModel>>,
) -> Result<usize, (String, usize)> {
    let mut vm = state.lock().unwrap();
    match vm.loadcase_new(name) {
        Ok(_) => Ok(0),
        Err(error) => Err(vm.error_get_dto(error)),
    }
}

#[tauri::command]
pub async fn loadcase_set_current(
    id: usize,
    state: State<'_, Mutex<ViewModel>>,
) -> Result<(), (String, usize)> {
    let mut vm = state.lock().unwrap();
    let result = vm.loadcase_set_current(id);
    match result {
        Ok(_) => Ok(()),
        Err(error) => Err(vm.error_get_dto(error)),
    }
}

#[tauri::command]
pub async fn loadcase_update(
    id: usize,
    name: String,
    state: State<'_, Mutex<ViewModel>>,
) -> Result<usize, (String, usize)> {
    let mut vm = state.lock().unwrap();
    match vm.loadcase_update(id, name) {
        Ok(_) => Ok(0),
        Err(error) => Err(vm.error_get_dto(error)),
    }
}
