use crate::models::{MemberResult, NodeReaction};
use crate::viewmodel::ViewModel;
use std::{collections::HashMap, sync::Mutex};
use tauri::State;

type MemberResults = HashMap<usize, Vec<MemberResult>>;
type NodeReactions = HashMap<usize, Vec<NodeReaction>>;
#[tauri::command]
pub async fn analysis_run_linear(
    state: State<'_, Mutex<ViewModel>>,
) -> Result<(MemberResults, NodeReactions, MemberResults, NodeReactions), (String, usize)> {
    let vm = state.lock().unwrap();
    let result = vm.analysis_run_linear();
    match result {
        Ok(value) => Ok((
            value.loadcase_results,
            value.loadcase_reactions,
            value.combination_results,
            value.combination_reactions,
        )),
        Err(error) => Err(vm.error_get_dto(error)),
    }
}
