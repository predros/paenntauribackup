#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MemberResult {
    pub id: usize,

    pub dx: Vec<f64>,
    pub dy: Vec<f64>,
    pub rz: Vec<f64>,

    pub normal: Vec<f64>,
    pub shear: Vec<f64>,
    pub moment: Vec<f64>,

    pub vert_shear: (f64, f64),
    pub vert_normal: (f64, f64),
    pub max_moment: (f64, f64),
    pub min_moment: (f64, f64),
}
