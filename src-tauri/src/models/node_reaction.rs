#[derive(serde::Serialize)]
pub struct NodeReaction {
    pub id: usize,
    pub rx: f64,
    pub ry: f64,
    pub mz: f64,
}
