use super::{
    member_dof_indices, member_force_vector, member_load_properties, node_properties_initial,
    AnalysisError, AnalysisResults, LoadProperties, MemberProperties,
};

use crate::models::{
    Combination, Direction, Loadcase, Material, Member, MemberResult, Node, NodeReaction, Section,
};
use nalgebra::{DMatrix, DVector, SMatrix, SVector};
use std::collections::HashMap;

type DMatrixFloat = DMatrix<f64>;
type DVectorFloat = DVector<f64>;
type Matrix6x6 = SMatrix<f64, 6, 6>;
type Vector6f = SVector<f64, 6>;

pub fn linear_analysis(
    nodes_list: &[Node],
    members_list: &[Member],
    materials_list: &HashMap<usize, Material>,
    sections_list: &HashMap<usize, Section>,
    loadcases_list: &HashMap<usize, Loadcase>,
    combinations_list: &HashMap<usize, Combination>,
) -> Result<AnalysisResults, AnalysisError> {
    // Result declarations
    let mut results_total: HashMap<usize, Vec<MemberResult>> = HashMap::new();
    let mut reactions_total: HashMap<usize, Vec<NodeReaction>> = HashMap::new();

    // Node properties
    let (nodes_indices, mut nodes_members, nodes_rotations) = node_properties_initial(nodes_list);
    let mut dofs_extra: Vec<usize> = vec![0; nodes_list.len()];

    // Member properties
    let mut member_props: HashMap<usize, MemberProperties> = HashMap::new();

    // Populate the members per node list and get any extra DOFs by internal hinges
    for member in members_list.iter() {
        // The only time we need to check if the member node ids are valid
        let start_index = match nodes_indices.get(&member.node_start()) {
            Some(value) => *value,
            None => return Err(AnalysisError::InvalidNodeId(member.node_start())),
        };
        let start_node = &nodes_list[start_index];
        let start_members = nodes_members.get_mut(&member.node_start()).unwrap();

        *start_members += 1;
        if member.hinge_start() && !start_node.hinged() && *start_members > 1 {
            dofs_extra[start_index] += 1;
        }

        let end_index = match nodes_indices.get(&member.node_end()) {
            Some(value) => *value,
            None => return Err(AnalysisError::InvalidNodeId(member.node_end())),
        };
        let end_node = &nodes_list[end_index];
        let end_members = nodes_members.get_mut(&member.node_end()).unwrap();

        *end_members += 1;
        if member.hinge_end() && !end_node.hinged() && *end_members > 1 {
            dofs_extra[end_index] += 1;
        }
    }

    // Populates the DOFs list, along with the springs and prescribed displacements list
    let mut n_dof = 0;
    let mut dofs: Vec<i32> = vec![];
    let mut springs: Vec<f64> = vec![];
    let mut prescribed_displacements: Vec<f64> = vec![];

    let mut dofs_extra_iter = dofs_extra.iter_mut();
    for node in nodes_list.iter() {
        // Check if the node itself is hinged or if every member connected to it is hinged
        let current_members = *nodes_members.get(&node.id()).unwrap();
        let current_extra = dofs_extra_iter.next().unwrap();
        if (node.hinged() && current_members > 0)
            || (*current_extra >= current_members && current_members > 1)
        {
            *current_extra = current_members - 1;
        }
        // Rotate the prescribed displacements from local nodal coordinates to global coordinates
        let rotation = nodes_rotations.get(&node.id()).unwrap();

        // Populate the lists
        if node.support(Direction::X) {
            let displacement_global = node.prescribed_displacement(Direction::X) * rotation.0
                - node.prescribed_displacement(Direction::Y) * rotation.1;

            dofs.push(-1);
            springs.push(0.0);
            prescribed_displacements.push(displacement_global);
        } else {
            dofs.push(n_dof);
            springs.push(node.spring(Direction::X));
            prescribed_displacements.push(0.0);

            n_dof += 1;
        }

        if node.support(Direction::Y) {
            let displacement_global = node.prescribed_displacement(Direction::X) * rotation.1
                + node.prescribed_displacement(Direction::Y) * rotation.0;

            dofs.push(-1);
            springs.push(0.0);
            prescribed_displacements.push(displacement_global);
        } else {
            dofs.push(n_dof);
            springs.push(node.spring(Direction::Y));
            prescribed_displacements.push(0.0);

            n_dof += 1;
        }

        if node.support(Direction::Z) && *current_extra == 0 {
            dofs.push(-1);
            springs.push(0.0);
            prescribed_displacements.push(node.prescribed_displacement(Direction::Z));
        } else {
            dofs.push(n_dof);
            springs.push(node.spring(Direction::Z));
            prescribed_displacements.push(0.0);
            n_dof += 1;

            for _ in 0..*current_extra {
                dofs.push(n_dof);
                springs.push(node.spring(Direction::Z));
                prescribed_displacements.push(0.0);
                n_dof += 1;
            }
        }
    }
    let n_dof: usize = n_dof.try_into().unwrap();

    // Initializes the structure's stiffness matrix
    let mut stiffness_matrix = DMatrixFloat::zeros(n_dof, n_dof);

    // Initializes the prescribed displacements forces vector
    let mut vector_prescribed_forces = DVectorFloat::zeros(n_dof);

    // Adds spring constants to the stiffness matrix
    let mut dofs_iter = dofs.iter();
    for spring in springs.iter() {
        let dof_current = *dofs_iter.next().unwrap();

        if dof_current >= 0 {
            stiffness_matrix[(dof_current as usize, dof_current as usize)] += *spring;
        }
    }

    // Member properties
    let mut dofs_extra_done: Vec<usize> = vec![0; nodes_list.len()];
    for member in members_list.iter() {
        // Start DOF index offsets
        let start_index = *nodes_indices.get(&member.node_start()).unwrap();
        let start_node = &nodes_list[start_index];

        // End DOF index offsets
        let end_index = *nodes_indices.get(&member.node_end()).unwrap();
        let end_node = &nodes_list[end_index];

        // Member DOF indices
        let member_indices = member_dof_indices(
            member,
            (start_index, start_node),
            (end_index, end_node),
            &dofs_extra,
            &nodes_members,
            &mut dofs_extra_done,
        );

        // Member DOFs
        let member_dofs: [i32; 6] = member_indices
            .iter()
            .map(|x| dofs[*x])
            .collect::<Vec<i32>>()
            .try_into()
            .unwrap();

        // Member prescribed displacements
        let member_displacements: [f64; 6] = member_indices
            .iter()
            .map(|x| prescribed_displacements[*x])
            .collect::<Vec<f64>>()
            .try_into()
            .unwrap();
        let member_displacements = Vector6f::from_column_slice(&member_displacements);

        // Member material
        let material = match materials_list.get(&member.material()) {
            Some(value) => value,
            None => return Err(AnalysisError::InvalidMaterialId(member.material())),
        };

        // Member section
        let section = match sections_list.get(&member.section()) {
            Some(value) => value,
            None => return Err(AnalysisError::InvalidSectionId(member.section())),
        };

        // Material and section properties
        let elasticity = material.elasticity();
        let inertia = section.inertia();
        let area = section.area();

        // Geometric properties
        let delta_x = end_node.x() - start_node.x();
        let delta_y = end_node.y() - start_node.y();
        let length = (delta_x * delta_x + delta_y * delta_y).sqrt();

        // Rotation matrices
        let cos = delta_x / length;
        let sin = delta_y / length;
        let start_rotation = nodes_rotations.get(&start_node.id()).unwrap();
        let end_rotation = nodes_rotations.get(&end_node.id()).unwrap();

        let rotation_matrix = Matrix6x6::new(
            cos, sin, 0.0, 0.0, 0.0, 0.0, -sin, cos, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, cos, sin, 0.0, 0.0, 0.0, 0.0, -sin, cos, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 1.0,
        );

        let node_rotation_matrix = Matrix6x6::new(
            start_rotation.0,
            start_rotation.1,
            0.0,
            0.0,
            0.0,
            0.0,
            -start_rotation.1,
            start_rotation.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            end_rotation.0,
            end_rotation.1,
            0.0,
            0.0,
            0.0,
            0.0,
            -end_rotation.1,
            end_rotation.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        );

        // Local stiffness matrix
        let coef = [
            elasticity * area / length,
            12.0 * elasticity * inertia / length.powi(3),
            6.0 * elasticity * inertia / length.powi(2),
            4.0 * elasticity * inertia / length,
            2.0 * elasticity * inertia / length,
        ];

        let member_stiffness_local = Matrix6x6::new(
            coef[0], 0.0, 0.0, -coef[0], 0.0, 0.0, 0.0, coef[1], coef[2], 0.0, -coef[1], coef[2],
            0.0, coef[2], coef[3], 0.0, -coef[2], coef[4], -coef[0], 0.0, 0.0, coef[0], 0.0, 0.0,
            0.0, -coef[1], -coef[2], 0.0, coef[1], -coef[2], 0.0, coef[2], coef[4], 0.0, -coef[2],
            coef[3],
        );

        let member_stiffness_global = node_rotation_matrix.transpose()
            * rotation_matrix.transpose()
            * member_stiffness_local
            * rotation_matrix
            * node_rotation_matrix;

        let member_displacement_forces = member_stiffness_global * member_displacements;

        for i in 0..6 {
            if member_dofs[i] >= 0 {
                vector_prescribed_forces[member_dofs[i] as usize] += member_displacement_forces[i];
                for j in 0..6 {
                    if member_dofs[j] >= 0 {
                        stiffness_matrix[(member_dofs[i] as usize, member_dofs[j] as usize)] +=
                            member_stiffness_global[(i, j)];
                    }
                }
            }
        }

        let props = MemberProperties {
            id: member.id(),
            start_index,
            end_index,
            elasticity,
            thermal: material.thermal(),
            inertia,
            area,
            y_sup: section.y_sup(),
            y_inf: section.y_inf(),
            length,
            dofs: member_dofs,
            dofs_indices: member_indices,
            stiffness_matrix: member_stiffness_local,
            rotation_matrix,
            node_rotation_matrix,
        };

        member_props.insert(member.id(), props);
    }

    // Factors the stiffness matrix
    let stiffness_lu = stiffness_matrix.lu();

    for (id, loadcase) in loadcases_list.iter() {
        // Current results declaration
        let mut results_current: Vec<MemberResult> = vec![];
        let mut reactions_current: Vec<NodeReaction> = vec![];

        let mut vector_total_forces = DVectorFloat::zeros(n_dof);
        let mut nodes_nodal: HashMap<usize, (f64, f64, f64)> = HashMap::new();

        // Nodal forces
        let mut dofs_iter = dofs.iter();
        let mut dofs_extra_iter = dofs_extra.iter();
        for node in nodes_list.iter() {
            let nodal = match loadcase.get_nodal(node.id()) {
                Some(value) => value,
                None => return Err(AnalysisError::InvalidNodeId(node.id())),
            };

            let cos = nodal.angle.to_radians().cos();
            let sin = nodal.angle.to_radians().sin();

            let fx = nodal.fx * cos - nodal.fy * sin;
            let fy = nodal.fx * sin + nodal.fy * cos;

            let dof_current = *dofs_iter.next().unwrap();
            if dof_current >= 0 {
                vector_total_forces[dof_current as usize] += fx;
            }

            let dof_current = *dofs_iter.next().unwrap();
            if dof_current >= 0 {
                vector_total_forces[dof_current as usize] += fy;
            }

            let dofs_extra_current = *dofs_extra_iter.next().unwrap();
            let members_current = *nodes_members.get(&node.id()).unwrap();

            let dof_current = *dofs_iter.next().unwrap();
            if !node.hinged() && dofs_extra_current != members_current - 1 {
                vector_total_forces[dof_current as usize] += nodal.mz;
            }

            for _ in 0..dofs_extra_current {
                let _ = dofs_iter.next();
            }

            reactions_current.push(NodeReaction {
                id: node.id(),
                rx: 0.0,
                ry: 0.0,
                mz: 0.0,
            });
            nodes_nodal.insert(node.id(), (fx, fy, nodal.mz));
        }

        // Member load properties
        let mut member_load_props: HashMap<usize, LoadProperties> = HashMap::new();
        let mut member_load_vectors: HashMap<usize, Vector6f> = HashMap::new();

        // Member loads
        for member in members_list.iter() {
            let props = member_props.get(&member.id()).unwrap();

            let load = match loadcase.get_load(member.id()) {
                Some(value) => value,
                None => return Err(AnalysisError::InvalidMemberId(member.id())),
            };

            let load_props = member_load_properties(props, load);

            let vector_force_local = member_force_vector(props, &load_props);
            let vector_force_global = props.node_rotation_matrix.transpose()
                * props.rotation_matrix.transpose()
                * vector_force_local;

            for i in 0..6 {
                if props.dofs[i] >= 0 {
                    vector_total_forces[props.dofs[i] as usize] += vector_force_global[i];
                }
            }

            member_load_props.insert(member.id(), load_props);
            member_load_vectors.insert(member.id(), vector_force_local);
        }

        // Subtract the prescribed displacements vector from the total
        vector_total_forces -= &vector_prescribed_forces;

        // Solve the system
        let mut displacements_total: Vec<f64>;
        if n_dof != 0 {
            let displacements_dof = match stiffness_lu.solve(&vector_total_forces) {
                Some(value) => value,
                None => return Err(AnalysisError::UnstableStructure),
            };

            displacements_total = vec![];

            let mut pdispl_iter = prescribed_displacements.iter();
            for dof in dofs.iter() {
                let pdispl_current = *pdispl_iter.next().unwrap();
                if *dof >= 0 {
                    displacements_total.push(displacements_dof[*dof as usize]);
                } else {
                    displacements_total.push(pdispl_current);
                }
            }
        } else {
            displacements_total = prescribed_displacements.clone();
        }

        for member in members_list.iter() {
            let props = member_props.get(&member.id()).unwrap();
            let load_props = member_load_props.get(&member.id()).unwrap();
            let load_vector = member_load_vectors.get(&member.id()).unwrap();

            let member_displacements_global: Vec<f64> = props
                .dofs_indices
                .iter()
                .map(|x| displacements_total[*x])
                .collect();
            let member_displacements_global = Vector6f::new(
                member_displacements_global[0],
                member_displacements_global[1],
                member_displacements_global[2],
                member_displacements_global[3],
                member_displacements_global[4],
                member_displacements_global[5],
            );

            let member_displacements_local =
                props.rotation_matrix * props.node_rotation_matrix * member_displacements_global;

            let member_forces_local =
                props.stiffness_matrix * member_displacements_local - load_vector;

            let member_forces_global = props.node_rotation_matrix.transpose()
                * props.rotation_matrix.transpose()
                * member_forces_local;

            let member_results = member_results_list(
                props,
                load_props,
                &member_displacements_local,
                &member_forces_local,
            );

            results_current.push(member_results);

            let start_index = *nodes_indices.get(&member.node_start()).unwrap();
            let start_node = &nodes_list[start_index];
            let start_nodal = nodes_nodal.get(&start_node.id()).unwrap();
            let start_reaction = &mut reactions_current[start_index];

            if start_node.support(Direction::X) || start_node.spring(Direction::X) != 0.0 {
                start_reaction.rx += member_forces_global[0] - start_nodal.0;
            }

            if start_node.support(Direction::Y) || start_node.spring(Direction::X) != 0.0 {
                start_reaction.ry += member_forces_global[1] - start_nodal.1;
            }

            let start_extra = dofs_extra[start_index];
            let start_members = *nodes_members.get(&start_node.id()).unwrap();
            if (start_node.support(Direction::Z) || start_node.spring(Direction::X) != 0.0)
                && !(start_node.hinged() || (start_extra != start_members - 1 && start_members > 1))
            {
                start_reaction.mz += member_forces_global[2] - start_nodal.2;
            }

            let end_index = *nodes_indices.get(&member.node_end()).unwrap();
            let end_node = &nodes_list[end_index];
            let end_nodal = nodes_nodal.get(&end_node.id()).unwrap();
            let end_reaction = &mut reactions_current[end_index];

            if end_node.support(Direction::X) || end_node.spring(Direction::X) != 0.0 {
                end_reaction.rx += member_forces_global[3] - end_nodal.0;
            }

            if end_node.support(Direction::Y) || end_node.spring(Direction::Y) != 0.0 {
                end_reaction.ry += member_forces_global[4] - end_nodal.1;
            }

            let end_extra = dofs_extra[end_index];
            let end_members = *nodes_members.get(&end_node.id()).unwrap();
            if (end_node.support(Direction::Z) || end_node.spring(Direction::Z) != 0.0)
                && !(end_node.hinged() || (end_extra != end_members - 1 && end_members > 1))
            {
                end_reaction.mz += member_forces_global[5] - end_nodal.2;
            }
        }

        results_total.insert(*id, results_current);
        reactions_total.insert(*id, reactions_current);
    }

    let (results_comb, reactions_comb) = combination_results(
        &nodes_list,
        &members_list,
        &member_props,
        &results_total,
        &reactions_total,
        &combinations_list,
    );

    Ok(AnalysisResults {
        loadcase_reactions: reactions_total,
        loadcase_results: results_total,
        combination_reactions: reactions_comb,
        combination_results: results_comb,
    })
}

fn member_results_list(
    member_props: &MemberProperties,
    load_props: &LoadProperties,
    end_displacements: &Vector6f,
    end_forces: &Vector6f,
) -> MemberResult {
    // Declare the result struct
    let mut result = MemberResult {
        id: member_props.id,
        dx: vec![],
        dy: vec![],
        rz: vec![],
        normal: vec![],
        shear: vec![],
        moment: vec![],
        vert_shear: (-1.0, 0.0),
        vert_normal: (-1.0, 0.0),
        max_moment: (-1.0, 0.0),
        min_moment: (-1.0, 0.0),
    };

    // Find the number of points to be sampled (and convert it to usize)
    let num_points_float = (member_props.length / 10.0).ceil().max(15.0);
    let num_points: usize = num_points_float as usize;

    // Member properties
    let ei = member_props.elasticity * member_props.inertia;
    let ea = member_props.elasticity * member_props.area;

    // Distance between two consecutive sampled points
    let step = member_props.length / (num_points_float - 1.0);

    // Auxiliar variables (renamed for ease)
    let f0 = end_forces[0];
    let f1 = end_forces[1];
    let f2 = end_forces[2];

    let u0 = end_displacements[0];
    let v0 = end_displacements[1];
    let theta_0 = end_displacements[2];

    // Iterate through each sampled point
    for index in 0..num_points {
        // Convert the index to float and use it to find the current coordinate
        let i_float = index as f64;
        let x = i_float * step;

        // Find the point's internal forces
        let n = -f0 + x * (-load_props.qx0 - x * load_props.delta_qx / 2.0 / member_props.length);
        let q = f1 + x * (load_props.qy0 + x * load_props.delta_qy / 2.0 / member_props.length);
        let m = -f2
            + x * (f1
                + x * (load_props.qy0 / 2.0 + x * load_props.delta_qy / 6.0 / member_props.length));

        // Find the point's displacements
        let u = u0
            - x * (f0
                + x * (load_props.qx0 / 2.0 + x * load_props.delta_qx / 6.0 / member_props.length))
                / ea;
        let theta = x
            * (-f2
                + x * (f1 / 2.0
                    + x * (load_props.qy0 + x * load_props.delta_qy / 24.0 / member_props.length)))
            / ei
            + theta_0;
        let v = x.powi(2)
            * (-f2 / 2.0
                + x * (f1 / 6.0
                    + x * (load_props.qy0 / 24.0
                        + x * load_props.delta_qy / 60.0 / member_props.length)))
            / ei
            + theta_0 * x
            + v0;

        // Push each value to the respective vector
        result.dx.push(u);
        result.dy.push(v);
        result.rz.push(theta);

        result.normal.push(n);
        result.shear.push(q);
        result.moment.push(m);
    }

    // Check if there is a critical point (max or min) for the normal force, and store it
    if load_props.delta_qx != 0.0 {
        let x_vert = -load_props.qx0 * member_props.length / load_props.delta_qx;

        if x_vert > 0.0 && x_vert < member_props.length {
            let q_vert = -f0
                + x_vert
                    * (-load_props.qx0 - x_vert * load_props.delta_qx / 2.0 / member_props.length);
            result.vert_normal = (x_vert, q_vert);
        }
    }

    // Check if there is a critical point (max or min) for the shear force, and store it
    if load_props.delta_qy != 0.0 {
        let x_vert = -load_props.qy0 * member_props.length / load_props.delta_qy;

        if x_vert > 0.0 && x_vert < member_props.length {
            let q_vert = f1
                + x_vert
                    * (load_props.qy0 + x_vert * load_props.delta_qy / 2.0 / member_props.length);
            result.vert_shear = (x_vert, q_vert);
        }
    }

    // Check if there are any (1 or 2) critical points for the bending moment, and store them
    let discriminant =
        load_props.qy0.powi(2) - 2.0 * load_props.delta_qy * f1 / member_props.length;
    if load_props.delta_qy != 0.0 && discriminant >= 0.0 {
        let b = load_props.qy0;
        let a = load_props.delta_qy / 2.0 / member_props.length;

        let x0 = (-b + discriminant.sqrt()) / 2.0 / a;
        let x1 = (-b - discriminant.sqrt()) / 2.0 / a;
        let m0 = -f2
            + x0 * (f1
                + x0 * (load_props.qy0 / 2.0
                    + x0 * load_props.delta_qy / 6.0 / member_props.length));
        let m1 = -f2
            + x1 * (f1
                + x1 * (load_props.qy0 / 2.0
                    + x1 * load_props.delta_qy / 6.0 / member_props.length));

        let mut x_max = -1.0;
        let mut x_min = -1.0;
        let mut m_max = 0.0;
        let mut m_min = 0.0;

        if m0 > m1 {
            if x0 > 0.0 && x0 < member_props.length {
                x_max = x0;
                m_max = m0;
            }
            if x1 > 0.0 && x1 < member_props.length {
                x_min = x1;
                m_min = m1;
            }
        } else {
            if x0 > 0.0 && x0 < member_props.length {
                x_max = x1;
                m_max = m1;
            }
            if x1 > 0.0 && x1 < member_props.length {
                x_min = x0;
                m_min = m0;
            }
        }

        result.max_moment = (x_max, m_max);
        result.min_moment = (x_min, m_min);
    } else if load_props.delta_qy == 0.0 && load_props.qy0 != 0.0 {
        let x_max = -f1 / load_props.qy0;

        if x_max > 0.0 && x_max < member_props.length {
            let m_max = -f2 + x_max * (f1 + x_max * (load_props.qy0 / 2.0));
            result.max_moment = (x_max, m_max);
        }
    }

    // Return the result
    result
}

fn combination_results(
    nodes_list: &[Node],
    members_list: &[Member],
    member_props: &HashMap<usize, MemberProperties>,
    results_members: &HashMap<usize, Vec<MemberResult>>,
    results_reactions: &HashMap<usize, Vec<NodeReaction>>,
    combinations_list: &HashMap<usize, Combination>,
) -> (
    HashMap<usize, Vec<MemberResult>>,
    HashMap<usize, Vec<NodeReaction>>,
) {
    let mut final_members: HashMap<usize, Vec<MemberResult>> = HashMap::new();
    let mut final_reactions: HashMap<usize, Vec<NodeReaction>> = HashMap::new();

    let mut member_id_to_index: HashMap<usize, usize> = HashMap::new();
    let mut node_id_to_index: HashMap<usize, usize> = HashMap::new();
    let mut num_points: Vec<usize> = vec![];

    for (index, node) in nodes_list.iter().enumerate() {
        node_id_to_index.insert(node.id(), index);
    }

    for (index, member) in members_list.iter().enumerate() {
        let props = member_props.get(&member.id()).unwrap();

        member_id_to_index.insert(member.id(), index);
        num_points.push((props.length / 10.0).ceil().max(15.0) as usize);
    }

    for (id_comb, comb) in combinations_list.iter() {
        let mut comb_members: Vec<MemberResult> = vec![];
        let mut comb_reactions: Vec<NodeReaction> = vec![];
        let factors = comb.get_all_factors();

        for node in nodes_list.iter() {
            comb_reactions.push(NodeReaction {
                id: node.id(),
                rx: 0.0,
                ry: 0.0,
                mz: 0.0,
            });
        }

        let mut iter_points = num_points.iter();
        for member in members_list.iter() {
            let points = *iter_points.next().unwrap();
            comb_members.push(MemberResult {
                id: member.id(),
                dx: vec![0.0; points],
                dy: vec![0.0; points],
                rz: vec![0.0; points],
                normal: vec![0.0; points],
                shear: vec![0.0; points],
                moment: vec![0.0; points],
                vert_shear: (-1.0, 0.0),
                vert_normal: (-1.0, 0.0),
                max_moment: (-1.0, 0.0),
                min_moment: (-1.0, 0.0),
            });
        }

        for (id_case, case) in results_members.iter() {
            let current_factor = match factors.get(id_case) {
                Some(value) => *value,
                None => 0.0,
            };

            // Member results
            let mut iter_member_final = comb_members.iter_mut();
            for (index, member) in case.iter().enumerate() {
                let current_member_final = iter_member_final.next().unwrap();

                let points = num_points[index];

                let mut iter_case_dx = member.dx.iter();
                let mut iter_case_dy = member.dy.iter();
                let mut iter_case_rz = member.rz.iter();
                let mut iter_case_normal = member.normal.iter();
                let mut iter_case_shear = member.shear.iter();
                let mut iter_case_moment = member.moment.iter();

                let mut iter_result_dx = current_member_final.dx.iter_mut();
                let mut iter_result_dy = current_member_final.dy.iter_mut();
                let mut iter_result_rz = current_member_final.rz.iter_mut();
                let mut iter_result_normal = current_member_final.normal.iter_mut();
                let mut iter_result_shear = current_member_final.shear.iter_mut();
                let mut iter_result_moment = current_member_final.moment.iter_mut();

                for _ in 0..points {
                    *iter_result_dx.next().unwrap() +=
                        *iter_case_dx.next().unwrap() * current_factor;
                    *iter_result_dy.next().unwrap() +=
                        *iter_case_dy.next().unwrap() * current_factor;
                    *iter_result_rz.next().unwrap() +=
                        *iter_case_rz.next().unwrap() * current_factor;
                    *iter_result_normal.next().unwrap() +=
                        *iter_case_normal.next().unwrap() * current_factor;
                    *iter_result_shear.next().unwrap() +=
                        *iter_case_shear.next().unwrap() * current_factor;
                    *iter_result_moment.next().unwrap() +=
                        *iter_case_moment.next().unwrap() * current_factor;
                }
            }
        }

        // Member extrema
        let mut iter_points = num_points.iter();
        for member in comb_members.iter_mut() {
            let props = member_props.get(&member.id).unwrap();
            let points = *iter_points.next().unwrap();

            let step = props.length / ((points as f64) - 1.0);

            let moment_max = *member
                .moment
                .iter()
                .max_by(|x, y| x.partial_cmp(y).unwrap())
                .unwrap();
            let moment_max_index = member.moment.iter().position(|x| *x == moment_max).unwrap();

            if moment_max_index != 0 && moment_max_index != member.moment.len() - 1 {
                let x = (moment_max_index as f64) * step;
                member.max_moment = (x, moment_max);
            }

            let moment_min = *member
                .moment
                .iter()
                .min_by(|x, y| x.partial_cmp(y).unwrap())
                .unwrap();
            let moment_min_index = member.moment.iter().position(|x| *x == moment_min).unwrap();

            if moment_min_index != 0 && moment_min_index != member.moment.len() - 1 {
                let x = (moment_min_index as f64) * step;
                member.min_moment = (x, moment_min);
            }

            let shear_vert = *member
                .shear
                .iter()
                .max_by(|x, y| x.abs().partial_cmp(&y.abs()).unwrap())
                .unwrap();
            let shear_vert_index = member.shear.iter().position(|x| *x == shear_vert).unwrap();

            if shear_vert_index != 0 && shear_vert_index != member.shear.len() - 1 {
                let x = (shear_vert_index as f64) * step;
                member.vert_shear = (x, shear_vert);
            }

            let normal_vert = *member
                .normal
                .iter()
                .max_by(|x, y| x.abs().partial_cmp(&y.abs()).unwrap())
                .unwrap();
            let normal_vert_index = member
                .normal
                .iter()
                .position(|x| *x == normal_vert)
                .unwrap();

            if normal_vert_index != 0 && normal_vert_index != member.normal.len() - 1 {
                let x = (normal_vert_index as f64) * step;
                member.vert_normal = (x, normal_vert);
            }
        }

        // Node reactions
        for (id_case, case) in results_reactions.iter() {
            let current_factor = match factors.get(id_case) {
                Some(value) => *value,
                None => 0.0,
            };

            let mut iter_reaction_final = comb_reactions.iter_mut();
            for node in case.iter() {
                let current_reaction_final = iter_reaction_final.next().unwrap();

                current_reaction_final.rx += node.rx * current_factor;
                current_reaction_final.ry += node.ry * current_factor;
                current_reaction_final.mz += node.mz * current_factor;
            }
        }

        final_members.insert(*id_comb, comb_members);
        final_reactions.insert(*id_comb, comb_reactions);
    }

    (final_members, final_reactions)
}
