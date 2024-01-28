use crate::viewmodel::ViewModel;
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
pub async fn selected_apply_hinges(
    node_ids: Vec<usize>,
    member_ids: Vec<usize>,
    on_nodes: bool,
    on_member_starts: bool,
    on_member_ends: bool,
    state: State<'_, Mutex<ViewModel>>,
) -> Result<(usize, usize), (String, usize)> {
    let mut vm = state.lock().unwrap();

    let _ = vm.state_save();

    for id in node_ids.iter() {
        let result = vm.node_set_hinged(*id, on_nodes, false);
        match result {
            Ok(_) => {}
            Err(error) => return Err(vm.error_get_dto(error)),
        };
    }

    for id in member_ids.iter() {
        let result = vm.member_set_hinges(*id, on_member_starts, on_member_ends, false);
        match result {
            Ok(_) => {}
            Err(error) => return Err(vm.error_get_dto(error)),
        };
    }

    Ok(vm.state_history_length())
}

#[tauri::command]
pub async fn selected_apply_material(
    ids: Vec<usize>,
    material_id: usize,
    state: State<'_, Mutex<ViewModel>>,
) -> Result<(usize, usize), (String, usize)> {
    let mut vm = state.lock().unwrap();

    let _ = vm.state_save();

    for id in ids.iter() {
        let result = vm.member_set_material(*id, material_id, false);
        match result {
            Ok(_) => {}
            Err(error) => return Err(vm.error_get_dto(error)),
        };
    }

    Ok(vm.state_history_length())
}

#[tauri::command]
pub async fn selected_apply_material_and_section(
    ids: Vec<usize>,
    material_id: usize,
    section_id: usize,
    state: State<'_, Mutex<ViewModel>>,
) -> Result<(usize, usize), (String, usize)> {
    let mut vm = state.lock().unwrap();

    let _ = vm.state_save();

    for id in ids.iter() {
        let result_0 = vm.member_set_material(*id, material_id, false);
        match result_0 {
            Ok(_) => {}
            Err(error) => return Err(vm.error_get_dto(error)),
        };

        let result_1 = vm.member_set_section(*id, section_id, false);
        match result_1 {
            Ok(_) => {}
            Err(error) => return Err(vm.error_get_dto(error)),
        };
    }

    Ok(vm.state_history_length())
}

#[tauri::command]
pub async fn selected_apply_member_loads(
    ids: Vec<usize>,
    qx0: f64,
    qy0: f64,
    qx1: f64,
    qy1: f64,
    is_global: bool,
    state: State<'_, Mutex<ViewModel>>,
) -> Result<(usize, usize), (String, usize)> {
    let mut vm = state.lock().unwrap();

    let _ = vm.state_save();

    for id in ids.iter() {
        let result = vm.loadcase_apply_load(*id, (qx0, qy0, qx1, qy1), is_global, false);
        match result {
            Ok(_) => {}
            Err(error) => return Err(vm.error_get_dto(error)),
        };
    }
    Ok(vm.state_history_length())
}

#[tauri::command]
pub async fn selected_apply_member_temperatures(
    ids: Vec<usize>,
    t_sup: f64,
    t_inf: f64,
    state: State<'_, Mutex<ViewModel>>,
) -> Result<(usize, usize), (String, usize)> {
    let mut vm = state.lock().unwrap();

    let _ = vm.state_save();

    for id in ids.iter() {
        let result = vm.loadcase_apply_temperature(*id, t_sup, t_inf, false);
        match result {
            Ok(_) => {}
            Err(error) => return Err(vm.error_get_dto(error)),
        };
    }
    Ok(vm.state_history_length())
}

#[tauri::command]
pub async fn selected_apply_nodal_forces(
    ids: Vec<usize>,
    fx: f64,
    fy: f64,
    mz: f64,
    angle: f64,
    state: State<'_, Mutex<ViewModel>>,
) -> Result<(usize, usize), (String, usize)> {
    let mut vm = state.lock().unwrap();

    let _ = vm.state_save();
    for id in ids.iter() {
        let result = vm.loadcase_apply_nodal(*id, fx, fy, mz, angle, false);
        match result {
            Ok(_) => {}
            Err(error) => return Err(vm.error_get_dto(error)),
        };
    }
    Ok(vm.state_history_length())
}

#[tauri::command]
pub async fn selected_apply_section(
    ids: Vec<usize>,
    section_id: usize,
    state: State<'_, Mutex<ViewModel>>,
) -> Result<(usize, usize), (String, usize)> {
    let mut vm = state.lock().unwrap();

    let _ = vm.state_save();

    for id in ids.iter() {
        let result = vm.member_set_section(*id, section_id, false);
        match result {
            Ok(_) => {}
            Err(error) => return Err(vm.error_get_dto(error)),
        };
    }

    Ok(vm.state_history_length())
}

#[tauri::command]
pub async fn selected_apply_supports(
    ids: Vec<usize>,
    supports: (bool, bool, bool, f64),
    springs: (f64, f64, f64),
    displacements: (f64, f64, f64),
    state: State<'_, Mutex<ViewModel>>,
) -> Result<(usize, usize), (String, usize)> {
    let mut vm = state.lock().unwrap();

    let _ = vm.state_save();

    for id in ids.iter() {
        let spring = vm.node_set_springs(*id, springs.0, springs.1, springs.2, false);
        match spring {
            Ok(_) => {}
            Err(error) => return Err(vm.error_get_dto(error)),
        };

        let _ = vm.node_set_supports(*id, supports.0, supports.1, supports.2, supports.3, false);
        let _ = vm.node_set_prescribed_displacements(
            *id,
            displacements.0,
            displacements.1,
            displacements.2,
            false,
        );
    }
    Ok(vm.state_history_length())
}

#[tauri::command]
pub async fn selected_delete(
    node_ids: Vec<usize>,
    member_ids: Vec<usize>,
    state: State<'_, Mutex<ViewModel>>,
) -> Result<(usize, usize), (String, usize)> {
    let mut vm = state.lock().unwrap();

    let _ = vm.state_save();

    for id in member_ids.iter() {
        let _ = vm.member_delete(*id, false);
    }

    for id in node_ids.iter() {
        let _ = vm.node_delete(*id, false);
    }

    Ok(vm.state_history_length())
}
