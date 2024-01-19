// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{collections::HashMap, sync::Mutex};

use models::{
    CombinationDTO, LoadcaseDTO, MaterialDTO, MemberDTO, MemberResult, NodeDTO, NodeReaction, SectionDTO,
    SectionType, Settings,
};
use tauri::{Manager, State, Window};
use viewmodel::{ViewModel, ViewModelError};

pub mod backend;
pub mod models;
pub mod viewmodel;

#[tauri::command]
async fn close_splashscreen(window: Window) {
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

fn main() {
    tauri::Builder::default()
        .manage(Mutex::from(ViewModel::new()))
        .invoke_handler(tauri::generate_handler![
            close_splashscreen,
            get_settings,
            get_current_file,
            unsaved_changes,
            new_file,
            open_file,
            save_file,
            run_analysis_linear,
            get_node_dtos,
            new_node,
            get_member_dtos,
            new_member,
            get_material_dtos,
            get_section_dtos,
            get_loadcase_dtos,
            get_loadcase_current,
            get_combination_dtos,
            set_loadcase_current,
            apply_supports,
            apply_nodal_forces,
            apply_member_loads,
            apply_member_temperatures,
            apply_hinges,
            apply_material,
            apply_section,
            apply_material_and_section,
            new_material,
            update_material,
            delete_material,
            new_section,
            update_section,
            delete_section,
            undo,
            redo,
            delete_selected,
            save_settings,
        ])
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application!");
}

#[tauri::command]
async fn get_settings(state: State<'_, Mutex<ViewModel>>) -> Result<Settings, (String, usize)> {
    let vm = state.lock().unwrap();
    Ok(vm.settings_get().unwrap())
}

#[tauri::command]
async fn get_current_file(state: State<'_, Mutex<ViewModel>>) -> Result<String, (String, usize)> {
    let vm = state.lock().unwrap();
    Ok(vm.get_current_file().unwrap())
}

#[tauri::command]
async fn unsaved_changes(state: State<'_, Mutex<ViewModel>>) -> Result<bool, (String, usize)> {
    let vm = state.lock().unwrap();
    Ok(vm.unsaved_changes().unwrap())
}

#[tauri::command]
async fn new_file(state: State<'_, Mutex<ViewModel>>) -> Result<(usize, usize), (String, usize)> {
    let mut vm = state.lock().unwrap();
    let result = vm.new_file();

    match result {
        Ok(_) => Ok(vm.history_len()),
        Err(error) => Err(vm.error_to_string(error)),
    }
}

#[tauri::command]
async fn open_file(
    path: String,
    state: State<'_, Mutex<ViewModel>>,
) -> Result<(usize, usize), (String, usize)> {
    let mut vm = state.lock().unwrap();
    let result = vm.open_file(&path);

    match result {
        Ok(_) => Ok(vm.history_len()),
        Err(error) => Err(vm.error_to_string(error)),
    }
}

#[tauri::command]
async fn save_file(
    path: String,
    state: State<'_, Mutex<ViewModel>>,
) -> Result<(), (String, usize)> {
    let mut vm = state.lock().unwrap();
    let result = vm.save_file(&path);

    match result {
        Ok(_) => Ok(()),
        Err(error) => Err(vm.error_to_string(error)),
    }
}

type MemberResults = HashMap<usize, Vec<MemberResult>>;
type NodeReactions = HashMap<usize, Vec<NodeReaction>>;

#[tauri::command]
async fn run_analysis_linear(
    state: State<'_, Mutex<ViewModel>>,
) -> Result<(MemberResults, NodeReactions, MemberResults, NodeReactions), (String, usize)> {
    let vm = state.lock().unwrap();
    let result = vm.run_linear_analysis();
    match result {
        Ok(value) => Ok((value.loadcase_results, value.loadcase_reactions, value.combination_results, value.combination_reactions)),
        Err(error) => Err(vm.error_to_string(error)),
    }
}

#[tauri::command]
async fn get_node_dtos(
    state: State<'_, Mutex<ViewModel>>,
) -> Result<Vec<NodeDTO>, (String, usize)> {
    let vm = state.lock().unwrap();
    let result = vm.node_get_dtos();
    match result {
        Ok(value) => Ok(value),
        Err(error) => Err(vm.error_to_string(error)),
    }
}

#[tauri::command]
async fn get_member_dtos(
    state: State<'_, Mutex<ViewModel>>,
) -> Result<Vec<MemberDTO>, (String, usize)> {
    let vm = state.lock().unwrap();
    let result = vm.member_get_all_dtos();
    match result {
        Ok(value) => Ok(value),
        Err(error) => Err(vm.error_to_string(error)),
    }
}

#[tauri::command]
async fn get_material_dtos(state: State<'_, Mutex<ViewModel>>) -> Result<Vec<MaterialDTO>, String> {
    let vm = state.lock().unwrap();
    let result = vm.material_get_all();
    Ok(result)
}

#[tauri::command]
async fn get_section_dtos(state: State<'_, Mutex<ViewModel>>) -> Result<Vec<SectionDTO>, String> {
    let vm = state.lock().unwrap();
    let result = vm.section_get_all();
    Ok(result)
}

#[tauri::command]
async fn get_loadcase_dtos(state: State<'_, Mutex<ViewModel>>) -> Result<Vec<LoadcaseDTO>, String> {
    let vm = state.lock().unwrap();
    let result = vm.loadcase_get_dtos();

    Ok(result)
}

#[tauri::command]
async fn get_loadcase_current(state: State<'_, Mutex<ViewModel>>) -> Result<usize, String> {
    let vm = state.lock().unwrap();
    let result = vm.loadcase_get_current();

    Ok(result)
}

#[tauri::command]
async fn get_combination_dtos(state: State<'_, Mutex<ViewModel>>) -> Result<Vec<CombinationDTO>, String> {
    let vm = state.lock().unwrap();
    let result = vm.combination_get_dtos();

    Ok(result)
}

#[tauri::command]
async fn new_node(
    x: f64,
    y: f64,
    state: State<'_, Mutex<ViewModel>>,
) -> Result<(usize, usize), (String, usize)> {
    let mut vm = state.lock().unwrap();
    let result = vm.node_new(x, y, true);
    match result {
        Ok(_) => Ok(vm.history_len()),
        Err(error) => Err(vm.error_to_string(error)),
    }
}

#[tauri::command]
async fn new_member(
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
    material_id: usize,
    section_id: usize,
    state: State<'_, Mutex<ViewModel>>,
) -> Result<(usize, usize), (String, usize)> {
    let mut vm = state.lock().unwrap();
    let result: Result<usize, ViewModelError> =
        vm.member_new((x0, y0, x1, y1), material_id, section_id, true);
    match result {
        Ok(_) => Ok(vm.history_len()),
        Err(error) => Err(vm.error_to_string(error)),
    }
}

#[tauri::command]
async fn set_loadcase_current(
    id: usize,
    state: State<'_, Mutex<ViewModel>>,
) -> Result<(), (String, usize)> {
    let mut vm = state.lock().unwrap();
    let result = vm.loadcase_set_current(id);
    match result {
        Ok(_) => Ok(()),
        Err(error) => Err(vm.error_to_string(error)),
    }
}

#[tauri::command]
async fn apply_supports(
    ids: Vec<usize>,
    supports: (bool, bool, bool, f64),
    springs: (f64, f64, f64),
    displacements: (f64, f64, f64),
    state: State<'_, Mutex<ViewModel>>,
) -> Result<(usize, usize), (String, usize)> {
    let mut vm = state.lock().unwrap();

    let _ = vm.save_state();

    for id in ids.iter() {
        let spring = vm.node_set_springs(*id, springs.0, springs.1, springs.2, false);
        match spring {
            Ok(_) => {}
            Err(error) => return Err(vm.error_to_string(error)),
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
    Ok(vm.history_len())
}

#[tauri::command]
async fn apply_nodal_forces(
    ids: Vec<usize>,
    fx: f64,
    fy: f64,
    mz: f64,
    angle: f64,
    state: State<'_, Mutex<ViewModel>>,
) -> Result<(usize, usize), (String, usize)> {
    let mut vm = state.lock().unwrap();

    let _ = vm.save_state();
    for id in ids.iter() {
        let result = vm.node_apply_forces(*id, fx, fy, mz, angle, false);
        match result {
            Ok(_) => {}
            Err(error) => return Err(vm.error_to_string(error)),
        };
    }
    Ok(vm.history_len())
}

#[tauri::command]
async fn apply_member_loads(
    ids: Vec<usize>,
    qx0: f64,
    qy0: f64,
    qx1: f64,
    qy1: f64,
    is_global: bool,
    state: State<'_, Mutex<ViewModel>>,
) -> Result<(usize, usize), (String, usize)> {
    let mut vm = state.lock().unwrap();

    let _ = vm.save_state();

    for id in ids.iter() {
        let result = vm.member_apply_loads(*id, (qx0, qy0, qx1, qy1), is_global, false);
        match result {
            Ok(_) => {}
            Err(error) => return Err(vm.error_to_string(error)),
        };
    }
    Ok(vm.history_len())
}

#[tauri::command]
async fn apply_member_temperatures(
    ids: Vec<usize>,
    t_sup: f64,
    t_inf: f64,
    state: State<'_, Mutex<ViewModel>>,
) -> Result<(usize, usize), (String, usize)> {
    let mut vm = state.lock().unwrap();

    let _ = vm.save_state();

    for id in ids.iter() {
        let result = vm.member_apply_temperatures(*id, t_sup, t_inf, false);
        match result {
            Ok(_) => {}
            Err(error) => return Err(vm.error_to_string(error)),
        };
    }
    Ok(vm.history_len())
}

#[tauri::command]
async fn apply_hinges(
    node_ids: Vec<usize>,
    member_ids: Vec<usize>,
    on_nodes: bool,
    on_member_starts: bool,
    on_member_ends: bool,
    state: State<'_, Mutex<ViewModel>>,
) -> Result<(usize, usize), (String, usize)> {
    let mut vm = state.lock().unwrap();

    let _ = vm.save_state();

    for id in node_ids.iter() {
        let result = vm.node_set_hinged(*id, on_nodes, false);
        match result {
            Ok(_) => {}
            Err(error) => return Err(vm.error_to_string(error)),
        };
    }

    for id in member_ids.iter() {
        let result = vm.member_set_hinges(*id, on_member_starts, on_member_ends, false);
        match result {
            Ok(_) => {}
            Err(error) => return Err(vm.error_to_string(error)),
        };
    }

    Ok(vm.history_len())
}

#[tauri::command]
async fn apply_material(
    ids: Vec<usize>,
    material_id: usize,
    state: State<'_, Mutex<ViewModel>>,
) -> Result<(usize, usize), (String, usize)> {
    let mut vm = state.lock().unwrap();

    let _ = vm.save_state();

    for id in ids.iter() {
        let result = vm.member_set_material(*id, material_id, false);
        match result {
            Ok(_) => {}
            Err(error) => return Err(vm.error_to_string(error)),
        };
    }

    Ok(vm.history_len())
}

#[tauri::command]
async fn apply_section(
    ids: Vec<usize>,
    section_id: usize,
    state: State<'_, Mutex<ViewModel>>,
) -> Result<(usize, usize), (String, usize)> {
    let mut vm = state.lock().unwrap();

    let _ = vm.save_state();

    for id in ids.iter() {
        let result = vm.member_set_section(*id, section_id, false);
        match result {
            Ok(_) => {}
            Err(error) => return Err(vm.error_to_string(error)),
        };
    }

    Ok(vm.history_len())
}

#[tauri::command]
async fn apply_material_and_section(
    ids: Vec<usize>,
    material_id: usize,
    section_id: usize,
    state: State<'_, Mutex<ViewModel>>,
) -> Result<(usize, usize), (String, usize)> {
    let mut vm = state.lock().unwrap();

    let _ = vm.save_state();

    for id in ids.iter() {
        let result_0 = vm.member_set_material(*id, material_id, false);
        match result_0 {
            Ok(_) => {}
            Err(error) => return Err(vm.error_to_string(error)),
        };

        let result_1 = vm.member_set_section(*id, section_id, false);
        match result_1 {
            Ok(_) => {}
            Err(error) => return Err(vm.error_to_string(error)),
        };
    }

    Ok(vm.history_len())
}

#[tauri::command]
async fn new_material(
    name: String,
    elasticity: f64,
    thermal: f64,
    state: State<'_, Mutex<ViewModel>>,
) -> Result<(), (String, usize)> {
    let mut vm = state.lock().unwrap();
    let result = vm.material_new(&name, elasticity, thermal);
    match result {
        Ok(_) => Ok(()),
        Err(error) => Err(vm.error_to_string(error)),
    }
}

#[tauri::command]
async fn update_material(
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
        Err(error) => Err(vm.error_to_string(error)),
    }
}

#[tauri::command]
async fn delete_material(
    id: usize,
    state: State<'_, Mutex<ViewModel>>,
) -> Result<(), (String, usize)> {
    let mut vm = state.lock().unwrap();
    let result = vm.material_delete(id);
    match result {
        Ok(_) => Ok(()),
        Err(error) => Err(vm.error_to_string(error)),
    }
}

#[tauri::command]
async fn new_section(
    name: String,
    section_type: SectionType,
    params: Vec<f64>,
    state: State<'_, Mutex<ViewModel>>,
) -> Result<(), (String, usize)> {
    let mut vm = state.lock().unwrap();
    let result = vm.section_new(&name, section_type, params);
    match result {
        Ok(_) => Ok(()),
        Err(error) => Err(vm.error_to_string(error)),
    }
}

#[tauri::command]
async fn update_section(
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
        Err(error) => Err(vm.error_to_string(error)),
    }
}

#[tauri::command]
async fn delete_section(
    id: usize,
    state: State<'_, Mutex<ViewModel>>,
) -> Result<(), (String, usize)> {
    let mut vm = state.lock().unwrap();
    let result = vm.section_delete(id);
    match result {
        Ok(_) => Ok(()),
        Err(error) => Err(vm.error_to_string(error)),
    }
}

#[tauri::command]
async fn undo(state: State<'_, Mutex<ViewModel>>) -> Result<(usize, usize), (String, usize)> {
    let mut vm = state.lock().unwrap();
    let result = vm.undo_state();

    match result {
        Ok(_) => Ok(vm.history_len()),
        Err(error) => Err(vm.error_to_string(error)),
    }
}

#[tauri::command]
async fn redo(state: State<'_, Mutex<ViewModel>>) -> Result<(usize, usize), (String, usize)> {
    let mut vm = state.lock().unwrap();
    let result = vm.redo_state();

    match result {
        Ok(_) => Ok(vm.history_len()),
        Err(error) => Err(vm.error_to_string(error)),
    }
}

#[tauri::command]
async fn delete_selected(
    node_ids: Vec<usize>,
    member_ids: Vec<usize>,
    state: State<'_, Mutex<ViewModel>>,
) -> Result<(usize, usize), (String, usize)> {
    let mut vm = state.lock().unwrap();

    let _ = vm.save_state();

    for id in member_ids.iter() {
        let _ = vm.member_delete(*id, false);
    }

    for id in node_ids.iter() {
        let _ = vm.node_delete(*id, false);
    }

    Ok(vm.history_len())
}

#[tauri::command]
async fn save_settings(
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
