use std::collections::HashMap;

use crate::models::{Member, MemberLoads, Node};
use nalgebra::{SMatrix, SVector};

type Vector6f = SVector<f64, 6>;
type Matrix6x6 = SMatrix<f64, 6, 6>;

#[derive(Debug)]
pub enum AnalysisError {
    UnstableStructure,
    InvalidNodeId(usize),
    InvalidMemberId(usize),
    InvalidMaterialId(usize),
    InvalidSectionId(usize),
    InvalidLoadcaseId(usize),
    NodeNotInLoadcase(usize, usize),
    MemberNotInLoadcase(usize, usize),
}

type NodeProperties = (
    HashMap<usize, usize>,
    HashMap<usize, usize>,
    HashMap<usize, (f64, f64)>,
);

pub struct MemberProperties {
    pub id: usize,
    pub start_index: usize,
    pub end_index: usize,

    pub elasticity: f64,
    pub thermal: f64,
    pub inertia: f64,
    pub area: f64,
    pub y_sup: f64,
    pub y_inf: f64,

    pub length: f64,

    pub dofs: [i32; 6],
    pub dofs_indices: [usize; 6],

    pub stiffness_matrix: Matrix6x6,
    pub rotation_matrix: Matrix6x6,
    pub node_rotation_matrix: Matrix6x6,
}

pub struct LoadProperties {
    pub id: usize,
    pub qx0: f64,
    pub delta_qx: f64,

    pub qy0: f64,
    pub delta_qy: f64,

    pub t_avg: f64,
    pub delta_t: f64,
}

pub fn node_properties_initial(nodes_list: &[Node]) -> NodeProperties {
    let mut id_to_index: HashMap<usize, usize> = HashMap::new();
    let mut members_per_node: HashMap<usize, usize> = HashMap::new();
    let mut rotations: HashMap<usize, (f64, f64)> = HashMap::new();

    for (index, node) in nodes_list.iter().enumerate() {
        let cos = node.support_angle().to_radians().cos();
        let sin = node.support_angle().to_radians().sin();

        rotations.insert(node.id(), (cos, sin));
        id_to_index.insert(node.id(), index);
        members_per_node.insert(node.id(), 0);
    }

    (id_to_index, members_per_node, rotations)
}

pub fn member_dof_indices(
    member: &Member,
    start: (usize, &Node),
    end: (usize, &Node),
    dofs_extra: &[usize],
    members_per_node: &HashMap<usize, usize>,
    dofs_extra_done: &mut [usize],
) -> [usize; 6] {
    let (start_index, start_node) = start;

    let start_extras = dofs_extra[start_index];
    let start_extras_done = &mut dofs_extra_done[start_index];
    let start_members = *members_per_node.get(&member.node_start()).unwrap();
    let start_offset: usize = dofs_extra[0..start_index].iter().sum();

    let start_hinged =
        start_members > 1 && (start_node.hinged() || start_extras == start_members - 1);
    let start_hinged = start_hinged || (start_members > 1 && member.hinge_start());
    let start_offset_extras = if start_hinged && start_extras > *start_extras_done {
        *start_extras_done += 1;
        *start_extras_done
    } else {
        0
    };

    let (end_index, end_node) = end;

    let end_extras = dofs_extra[end_index];
    let end_extras_done = &mut dofs_extra_done[end_index];
    let end_members = *members_per_node.get(&member.node_end()).unwrap();
    let end_offset: usize = dofs_extra[0..end_index].iter().sum();

    let end_hinged = end_members > 1 && (end_node.hinged() || end_extras == end_members - 1);
    let end_hinged = end_hinged || (end_members > 1 && member.hinge_end());

    let end_offset_extras = if end_hinged && end_extras > *end_extras_done {
        *end_extras_done += 1;
        *end_extras_done
    } else {
        0
    };

    [
        start_offset + 3 * start_index,
        start_offset + 3 * start_index + 1,
        start_offset + 3 * start_index + 2 + start_offset_extras,
        end_offset + 3 * end_index,
        end_offset + 3 * end_index + 1,
        end_offset + 3 * end_index + 2 + end_offset_extras,
    ]
}

pub fn member_force_vector(
    member_props: &MemberProperties,
    load_props: &LoadProperties,
) -> Vector6f {
    // Define each force vector, for the constant load, the linear load and the temperature
    // (a trapezoidal load from q0 to q1 can be divided into a constant load q0 and a triangular
    // load from 0 to q1 - q0)
    let vector_constant = Vector6f::new(
        load_props.qx0 * member_props.length / 2.0,
        load_props.qy0 * member_props.length / 2.0,
        load_props.qy0 * member_props.length.powi(2) / 12.0,
        load_props.qx0 * member_props.length / 2.0,
        load_props.qy0 * member_props.length / 2.0,
        -load_props.qy0 * member_props.length.powi(2) / 12.0,
    );
    let vector_linear = Vector6f::new(
        load_props.delta_qx * member_props.length / 6.0,
        3.0 * load_props.delta_qy * member_props.length / 20.0,
        load_props.delta_qy * member_props.length.powi(2) / 30.0,
        load_props.delta_qx * member_props.length / 3.0,
        7.0 * load_props.delta_qy * member_props.length / 20.0,
        -load_props.delta_qy * member_props.length.powi(2) / 20.0,
    );
    let vector_temperature = Vector6f::new(
        -member_props.elasticity * member_props.area * member_props.thermal * load_props.t_avg,
        0.0,
        member_props.elasticity * member_props.inertia * member_props.thermal * load_props.delta_t
            / (member_props.y_sup + member_props.y_inf),
        member_props.elasticity * member_props.area * member_props.thermal * load_props.t_avg,
        0.0,
        -member_props.elasticity * member_props.inertia * member_props.thermal * load_props.delta_t
            / (member_props.y_sup + member_props.y_inf),
    );

    // The total vector is the sum of each one
    vector_constant + vector_linear + vector_temperature
}

pub fn member_load_properties(
    member_props: &MemberProperties,
    load: &MemberLoads,
) -> LoadProperties {
    let qx0: f64;
    let qy0: f64;
    let qx1: f64;
    let qy1: f64;
    if load.is_global {
        // If the load is in global coordinates, rotate it to the local system
        let cos = member_props.rotation_matrix[(0, 0)];
        let sin = member_props.rotation_matrix[(0, 1)];
        qx0 = load.qx0 * cos + load.qy0 * sin;
        qy0 = -load.qx0 * sin + load.qy0 * cos;
        qx1 = load.qx1 * cos + load.qy1 * sin;
        qy1 = -load.qx1 * sin + load.qy1 * cos;
    } else {
        // Otherwise, just leave it as is
        qx0 = load.qx0;
        qy0 = load.qy0;
        qx1 = load.qx1;
        qy1 = load.qy1;
    }

    // Find the variation in the distributed loads
    let delta_qx = qx1 - qx0;
    let delta_qy = qy1 - qy0;

    // Find the relevant temperatures
    let t_sup = load.t_sup;
    let t_inf = load.t_inf;
    let delta_t = t_sup - t_inf;
    let t_avg = (t_sup * member_props.y_sup + t_inf * member_props.y_inf)
        / (member_props.y_sup + member_props.y_inf);

    LoadProperties {
        id: member_props.id,
        qx0,
        qy0,
        delta_qx,
        delta_qy,
        t_avg,
        delta_t,
    }
}
