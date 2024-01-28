use crate::models::CombinationDTO;
use crate::viewmodel::ViewModel;
use std::collections::HashMap;
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
pub async fn combination_apply_factors(
    factors: HashMap<usize, HashMap<usize, f64>>,
    state: State<'_, Mutex<ViewModel>>,
) -> Result<usize, (String, usize)> {
    let mut vm = state.lock().unwrap();

    for (comb_id, factors) in factors.iter() {
        let _ = vm.combination_set_factors(*comb_id, factors);
    }

    Ok(0)
}

#[tauri::command]
pub async fn combination_delete(
    id: usize,
    state: State<'_, Mutex<ViewModel>>,
) -> Result<usize, (String, usize)> {
    let mut vm = state.lock().unwrap();
    match vm.combination_delete(id) {
        Ok(_) => Ok(0),
        Err(error) => Err(vm.error_get_dto(error)),
    }
}

#[tauri::command]
pub async fn combination_get_dtos(
    state: State<'_, Mutex<ViewModel>>,
) -> Result<Vec<CombinationDTO>, (String, usize)> {
    let vm = state.lock().unwrap();
    let result = vm.combination_get_dtos();

    Ok(result)
}

#[tauri::command]
pub async fn combination_new(
    name: String,
    state: State<'_, Mutex<ViewModel>>,
) -> Result<usize, (String, usize)> {
    let mut vm = state.lock().unwrap();
    match vm.combination_new(name) {
        Ok(_) => Ok(0),
        Err(error) => Err(vm.error_get_dto(error)),
    }
}

#[tauri::command]
pub async fn combination_update(
    id: usize,
    name: String,
    state: State<'_, Mutex<ViewModel>>,
) -> Result<usize, (String, usize)> {
    let mut vm = state.lock().unwrap();
    match vm.combination_update(id, name) {
        Ok(_) => Ok(0),
        Err(error) => Err(vm.error_get_dto(error)),
    }
}
