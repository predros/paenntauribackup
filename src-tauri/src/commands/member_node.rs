use crate::models::{MemberDTO, NodeDTO};
use crate::viewmodel::ViewModel;
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
pub async fn member_get_dtos(
    state: State<'_, Mutex<ViewModel>>,
) -> Result<Vec<MemberDTO>, (String, usize)> {
    let vm = state.lock().unwrap();
    let result = vm.member_get_all_dtos();
    match result {
        Ok(value) => Ok(value),
        Err(error) => Err(vm.error_get_dto(error)),
    }
}

#[tauri::command]
pub async fn member_new(
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
    material_id: usize,
    section_id: usize,
    state: State<'_, Mutex<ViewModel>>,
) -> Result<(usize, usize), (String, usize)> {
    let mut vm = state.lock().unwrap();
    let result = vm.member_new((x0, y0, x1, y1), material_id, section_id, true);
    match result {
        Ok(_) => Ok(vm.state_history_length()),
        Err(error) => Err(vm.error_get_dto(error)),
    }
}

#[tauri::command]
pub async fn node_get_dtos(
    state: State<'_, Mutex<ViewModel>>,
) -> Result<Vec<NodeDTO>, (String, usize)> {
    let vm = state.lock().unwrap();
    let result = vm.node_get_dtos();
    match result {
        Ok(value) => Ok(value),
        Err(error) => Err(vm.error_get_dto(error)),
    }
}

#[tauri::command]
pub async fn node_new(
    x: f64,
    y: f64,
    state: State<'_, Mutex<ViewModel>>,
) -> Result<(usize, usize), (String, usize)> {
    let mut vm = state.lock().unwrap();
    let result = vm.node_new(x, y, true);
    match result {
        Ok(_) => Ok(vm.state_history_length()),
        Err(error) => Err(vm.error_get_dto(error)),
    }
}
