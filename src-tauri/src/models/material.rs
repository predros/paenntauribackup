#[derive(Debug)]
pub enum MaterialError {
    EmptyName,
    ZeroElasticity,
    NegativeElasticity,
    ZeroThermal,
    NegativeThermal,
}

#[derive(serde::Serialize)]
pub struct MaterialDTO {
    pub id: usize,
    pub name: String,
    pub elasticity: f64,
    pub thermal: f64,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Material {
    name: String,
    elasticity: f64,
    thermal: f64,
}

impl Material {
    pub fn new(name: &str, elasticity: f64, thermal: f64) -> Result<Material, MaterialError> {
        if elasticity == 0.0 {
            return Err(MaterialError::ZeroElasticity);
        } else if elasticity < 0.0 {
            return Err(MaterialError::NegativeElasticity);
        } else if thermal == 0.0 {
            return Err(MaterialError::ZeroThermal);
        } else if thermal < 0.0 {
            return Err(MaterialError::NegativeThermal);
        } else if name.trim().is_empty() {
            return Err(MaterialError::EmptyName);
        }

        Ok(Material {
            name: String::from(name),
            elasticity,
            thermal,
        })
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn elasticity(&self) -> f64 {
        self.elasticity
    }

    pub fn thermal(&self) -> f64 {
        self.thermal
    }

    pub fn set_name(&mut self, name: &str) -> Result<(), MaterialError> {
        if name.trim().is_empty() {
            Err(MaterialError::EmptyName)
        } else {
            self.name = String::from(name);
            Ok(())
        }
    }

    pub fn set_elasticity(&mut self, elasticity: f64) -> Result<(), MaterialError> {
        if elasticity == 0.0 {
            Err(MaterialError::ZeroElasticity)
        } else if elasticity < 0.0 {
            return Err(MaterialError::NegativeElasticity);
        } else {
            self.elasticity = elasticity;
            return Ok(());
        }
    }

    pub fn set_thermal(&mut self, thermal: f64) -> Result<(), MaterialError> {
        if thermal == 0.0 {
            Err(MaterialError::ZeroThermal)
        } else if thermal < 0.0 {
            return Err(MaterialError::NegativeThermal);
        } else {
            self.thermal = thermal;
            return Ok(());
        }
    }
}
