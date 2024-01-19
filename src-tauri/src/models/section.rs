use std::f64::consts;

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum SectionType {
    Generic = 0,
    Circle,
    Rectangle,
    SymmetricW,
    AsymmetricW,
}

#[derive(serde::Serialize)]
pub struct SectionDTO {
    pub id: usize,
    pub name: String,
    pub section_type: SectionType,
    pub inertia: f64,
    pub area: f64,
    pub y_sup: f64,
    pub y_inf: f64,
    pub params: Vec<f64>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Section {
    name: String,
    section_type: SectionType,
    inertia: f64,
    area: f64,
    y_sup: f64,
    y_inf: f64,
    params: Vec<f64>,
}

#[derive(Debug)]
pub enum SectionError {
    EmptyName,
    NegativeInertia,
    NegativeArea,
    NegativeDimension,
    LargerInnerDimension,
}

impl Section {
    fn new(name: &str) -> Section {
        Section {
            name: String::from(name),
            section_type: SectionType::Generic,
            inertia: 0.0,
            area: 0.0,
            y_sup: 0.0,
            y_inf: 0.0,
            params: Vec::new(),
        }
    }

    pub fn new_generic(
        name: &str,
        inertia: f64,
        area: f64,
        y_sup: f64,
        y_inf: f64,
    ) -> Result<Section, SectionError> {
        if name.trim().is_empty() {
            Err(SectionError::EmptyName)
        } else {
            let mut sec = Section::new(name);
            let result = sec.set_generic(inertia, area, y_sup, y_inf);

            match result {
                Ok(_) => Ok(sec),
                Err(e) => Err(e),
            }
        }
    }

    pub fn new_circle(
        name: &str,
        diameter_ext: f64,
        diameter_int: f64,
    ) -> Result<Section, SectionError> {
        let mut sec = Section::new(name);
        let result = sec.set_circle(diameter_ext, diameter_int);

        match result {
            Ok(_) => Ok(sec),
            Err(e) => Err(e),
        }
    }

    pub fn new_rect(
        name: &str,
        width: f64,
        height: f64,
        inner_width: f64,
        inner_height: f64,
    ) -> Result<Section, SectionError> {
        let mut sec = Section::new(name);
        let result = sec.set_rect(width, height, inner_width, inner_height);

        match result {
            Ok(_) => Ok(sec),
            Err(e) => Err(e),
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn inertia(&self) -> f64 {
        self.inertia
    }

    pub fn area(&self) -> f64 {
        self.area
    }

    pub fn y_sup(&self) -> f64 {
        self.y_sup
    }

    pub fn y_inf(&self) -> f64 {
        self.y_inf
    }

    pub fn section_type(&self) -> SectionType {
        self.section_type
    }

    pub fn params(&self) -> Vec<f64> {
        if self.section_type == SectionType::Generic {
            return vec![self.inertia, self.area, self.y_sup, self.y_inf];
        }

        self.params.clone()
    }

    pub fn set_name(&mut self, name: &str) -> Result<(), SectionError> {
        if name.trim().is_empty() {
            Err(SectionError::EmptyName)
        } else {
            self.name = String::from(name);
            Ok(())
        }
    }

    pub fn set_generic(
        &mut self,
        inertia: f64,
        area: f64,
        y_sup: f64,
        y_inf: f64,
    ) -> Result<(), SectionError> {
        if inertia < 0.0 {
            return Err(SectionError::NegativeInertia);
        } else if area < 0.0 {
            return Err(SectionError::NegativeArea);
        } else if y_sup < 0.0 || y_inf < 0.0 {
            return Err(SectionError::NegativeDimension);
        }

        self.section_type = SectionType::Generic;
        self.inertia = inertia;
        self.area = area;
        self.y_sup = y_sup;
        self.y_inf = y_inf;
        self.params = Vec::new();

        Ok(())
    }

    pub fn set_circle(&mut self, diameter_ext: f64, diameter_int: f64) -> Result<(), SectionError> {
        if diameter_ext < 0.0 || diameter_int < 0.0 {
            return Err(SectionError::NegativeDimension);
        } else if diameter_ext <= diameter_int {
            return Err(SectionError::LargerInnerDimension);
        }

        self.section_type = SectionType::Circle;
        self.inertia = consts::PI * (diameter_ext.powi(4) - diameter_int.powi(4)) / 64.0;
        self.area = consts::PI * (diameter_ext * diameter_ext - diameter_int * diameter_int) / 4.0;
        self.y_sup = diameter_ext / 2.0;
        self.y_inf = diameter_ext / 2.0;
        self.params = vec![diameter_ext, diameter_int];

        Ok(())
    }

    pub fn set_rect(
        &mut self,
        width: f64,
        height: f64,
        inner_width: f64,
        inner_height: f64,
    ) -> Result<(), SectionError> {
        if width < 0.0 || height < 0.0 {
            return Err(SectionError::NegativeDimension);
        } else if width <= inner_width || height <= inner_height {
            return Err(SectionError::LargerInnerDimension);
        }

        self.section_type = SectionType::Rectangle;
        self.inertia = (width * height.powi(3) - inner_width * inner_height.powi(3)) / 12.0;
        self.area = width * height - inner_width * inner_height;
        self.y_sup = height / 2.0;
        self.y_inf = height / 2.0;
        self.params = vec![width, height, inner_width, inner_height];

        Ok(())
    }
}
