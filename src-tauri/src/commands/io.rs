use crate::models::Settings;
use crate::viewmodel::ViewModel;
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
pub async fn file_get_current(
    state: State<'_, Mutex<ViewModel>>,
) -> Result<String, (String, usize)> {
    let vm = state.lock().unwrap();
    Ok(vm.file_get_current().unwrap())
}

#[tauri::command]
pub async fn file_new(
    state: State<'_, Mutex<ViewModel>>,
) -> Result<(usize, usize), (String, usize)> {
    let mut vm = state.lock().unwrap();
    let result = vm.file_new();

    match result {
        Ok(_) => Ok(vm.state_history_length()),
        Err(error) => Err(vm.error_get_dto(error)),
    }
}

#[tauri::command]
pub async fn file_open(
    path: String,
    state: State<'_, Mutex<ViewModel>>,
) -> Result<(usize, usize), (String, usize)> {
    let mut vm = state.lock().unwrap();
    let result = vm.file_open(&path);

    match result {
        Ok(_) => Ok(vm.state_history_length()),
        Err(error) => Err(vm.error_get_dto(error)),
    }
}

#[tauri::command]
pub async fn file_save(
    path: String,
    state: State<'_, Mutex<ViewModel>>,
) -> Result<(), (String, usize)> {
    let mut vm = state.lock().unwrap();
    let result = vm.file_save(&path);

    match result {
        Ok(_) => Ok(()),
        Err(error) => Err(vm.error_get_dto(error)),
    }
}

#[tauri::command]
pub async fn file_unsaved_changes(
    state: State<'_, Mutex<ViewModel>>,
) -> Result<bool, (String, usize)> {
    let vm = state.lock().unwrap();
    Ok(vm.file_unsaved_changes().unwrap())
}

#[tauri::command]
pub async fn settings_get(state: State<'_, Mutex<ViewModel>>) -> Result<Settings, (String, usize)> {
    let vm = state.lock().unwrap();
    Ok(vm.settings_get().unwrap())
}

#[tauri::command]
pub async fn settings_save(
    settings: Settings,
    state: State<'_, Mutex<ViewModel>>,
) -> Result<(), (String, usize)> {
    let mut vm = state.lock().unwrap();

    println!(
        "{}, {}, {}",
        settings.dark_theme, settings.grid_spacing[0], settings.grid_spacing[1]
    );
    println!("{}", settings.unit_precision.length.0);

    let _ = vm.settings_set(settings);

    Ok(())
}
