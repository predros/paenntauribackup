// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use viewmodel::ViewModel;

pub mod analysis;
pub mod commands;
pub mod models;
pub mod viewmodel;

use commands::*;

fn main() {
    tauri::Builder::default()
        .manage(Mutex::from(ViewModel::new()))
        .invoke_handler(tauri::generate_handler![
            analysis_run_linear,
            app_redo,
            app_splashcreen_close,
            app_undo,
            combination_apply_factors,
            combination_delete,
            combination_get_dtos,
            combination_new,
            combination_update,
            file_get_current,
            file_new,
            file_open,
            file_save,
            file_unsaved_changes,
            loadcase_delete,
            loadcase_get_current,
            loadcase_get_dtos,
            loadcase_new,
            loadcase_set_current,
            loadcase_update,
            material_delete,
            material_get_dtos,
            material_new,
            material_update,
            member_get_dtos,
            member_new,
            node_get_dtos,
            node_new,
            section_delete,
            section_get_dtos,
            section_new,
            section_update,
            selected_apply_hinges,
            selected_apply_material,
            selected_apply_material_and_section,
            selected_apply_member_loads,
            selected_apply_member_temperatures,
            selected_apply_nodal_forces,
            selected_apply_section,
            selected_apply_supports,
            selected_delete,
            settings_get,
            settings_save,
        ])
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application!");
}
