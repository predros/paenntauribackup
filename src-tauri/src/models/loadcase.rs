use std::collections::HashMap;

use super::{Member, Node};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct NodalForces {
    pub fx: f64,
    pub fy: f64,
    pub mz: f64,
    pub angle: f64,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct MemberLoads {
    pub qx0: f64,
    pub qy0: f64,
    pub qx1: f64,
    pub qy1: f64,
    pub is_global: bool,
    pub t_sup: f64,
    pub t_inf: f64,
}

#[derive(serde::Serialize)]
pub struct LoadcaseDTO {
    pub id: usize,
    pub name: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Loadcase {
    name: String,
    nodes: HashMap<usize, NodalForces>,
    members: HashMap<usize, MemberLoads>,
}

impl Loadcase {
    pub fn new(name: &str, nodes_list: &[Node], members_list: &[Member]) -> Loadcase {
        let mut nodes: HashMap<usize, NodalForces> = HashMap::new();
        let mut members: HashMap<usize, MemberLoads> = HashMap::new();

        for node in nodes_list.iter() {
            nodes.insert(
                node.id(),
                NodalForces {
                    fx: 0.0,
                    fy: 0.0,
                    mz: 0.0,
                    angle: 0.0,
                },
            );
        }

        for member in members_list.iter() {
            members.insert(
                member.id(),
                MemberLoads {
                    qx0: 0.0,
                    qy0: 0.0,
                    qx1: 0.0,
                    qy1: 0.0,
                    is_global: false,
                    t_sup: 0.0,
                    t_inf: 0.0,
                },
            );
        }

        Loadcase {
            name: name.to_string(),
            nodes,
            members,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn get_nodal(&self, id: usize) -> Option<&NodalForces> {
        return self.nodes.get(&id);
    }

    pub fn get_all_nodals(&self) -> &HashMap<usize, NodalForces> {
        &self.nodes
    }

    pub fn get_load(&self, id: usize) -> Option<&MemberLoads> {
        return self.members.get(&id);
    }

    pub fn get_all_loads(&self) -> &HashMap<usize, MemberLoads> {
        &self.members
    }

    pub fn set_name(&mut self, name: String) -> Result<(), bool> {
        self.name = name;
        Ok(())
    }

    pub fn add_node(&mut self, id: usize) -> Result<(), bool> {
        if self.nodes.contains_key(&id) {
            return Err(false);
        }

        self.nodes.insert(
            id,
            NodalForces {
                fx: 0.0,
                fy: 0.0,
                mz: 0.0,
                angle: 0.0,
            },
        );
        Ok(())
    }

    pub fn add_member(&mut self, id: usize) -> Result<(), bool> {
        if self.members.contains_key(&id) {
            return Err(false);
        }

        self.members.insert(
            id,
            MemberLoads {
                qx0: 0.0,
                qy0: 0.0,
                qx1: 0.0,
                qy1: 0.0,
                is_global: false,
                t_sup: 0.0,
                t_inf: 0.0,
            },
        );
        Ok(())
    }

    pub fn remove_node(&mut self, id: usize) -> Result<(), bool> {
        if !self.nodes.contains_key(&id) {
            return Err(false);
        }

        self.nodes.remove(&id);

        Ok(())
    }

    pub fn remove_member(&mut self, id: usize) -> Result<(), bool> {
        if !self.members.contains_key(&id) {
            return Err(false);
        }

        self.members.remove(&id);

        Ok(())
    }

    pub fn set_nodal(
        &mut self,
        id: usize,
        fx: f64,
        fy: f64,
        mz: f64,
        angle: f64,
    ) -> Result<(), bool> {
        let nodal = self.nodes.get_mut(&id);
        match nodal {
            Some(value) => {
                value.fx = fx;
                value.fy = fy;
                value.mz = mz;
                value.angle = angle;
                Ok(())
            }
            None => Err(false),
        }
    }

    pub fn set_load(
        &mut self,
        id: usize,
        qx0: f64,
        qy0: f64,
        qx1: f64,
        qy1: f64,
        is_global: bool,
    ) -> Result<(), bool> {
        let load: Option<&mut MemberLoads> = self.members.get_mut(&id);
        match load {
            Some(value) => {
                value.qx0 = qx0;
                value.qy0 = qy0;
                value.qx1 = qx1;
                value.qy1 = qy1;
                value.is_global = is_global;
                Ok(())
            }
            None => Err(false),
        }
    }

    pub fn set_temperature(&mut self, id: usize, t_sup: f64, t_inf: f64) -> Result<(), bool> {
        let load: Option<&mut MemberLoads> = self.members.get_mut(&id);
        match load {
            Some(value) => {
                value.t_sup = t_sup;
                value.t_inf = t_inf;
                Ok(())
            }
            None => Err(false),
        }
    }
}
