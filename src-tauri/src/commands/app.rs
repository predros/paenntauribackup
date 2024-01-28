use crate::viewmodel::ViewModel;
use std::sync::Mutex;
use tauri::{Manager, State, Window};

#[tauri::command]
pub async fn app_splashcreen_close(window: Window) {
    window
        .get_window("splashscreen")
        .expect("No window labeled 'splashscreen' found")
        .close()
        .unwrap();
    window
        .get_window("main")
        .expect("No window labeled 'main' found")
        .show()
        .unwrap();
}

#[tauri::command]
pub async fn app_redo(
    state: State<'_, Mutex<ViewModel>>,
) -> Result<(usize, usize), (String, usize)> {
    let mut vm = state.lock().unwrap();
    let result = vm.state_redo();

    match result {
        Ok(_) => Ok(vm.state_history_length()),
        Err(error) => Err(vm.error_get_dto(error)),
    }
}

#[tauri::command]
pub async fn app_undo(
    state: State<'_, Mutex<ViewModel>>,
) -> Result<(usize, usize), (String, usize)> {
    let mut vm = state.lock().unwrap();
    let result = vm.state_undo();

    match result {
        Ok(_) => Ok(vm.state_history_length()),
        Err(error) => Err(vm.error_get_dto(error)),
    }
}
