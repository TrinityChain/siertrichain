use crate::core::triangle::Triangle;
use crate::core::address::TriangleAddress;
use serde::{Deserialize, Serialize};
use rand::Rng;
use crate::core::subdivision::subdivide_triangle;
use crate::core::errors::TriangleError;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TriangleState {
    Genesis,
    Active,
    Subdivided,
    Void,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FractalTriangle {
    pub id: u64,
    pub triangle: Triangle,
    pub state: TriangleState,
    pub depth: u32,
    pub parent_id: Option<u64>,
    pub child_ids: Vec<u64>,
    pub address: TriangleAddress,
}

impl FractalTriangle {
    pub fn new(
        triangle: Triangle,
        state: TriangleState,
        depth: u32,
        parent_id: Option<u64>,
        address: TriangleAddress,
    ) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            id: rng.gen(),
            triangle,
            state,
            depth,
            parent_id,
            child_ids: Vec::new(),
            address,
        }
    }

    pub fn subdivide(&mut self) -> Result<Vec<FractalTriangle>, TriangleError> {
        if self.state != TriangleState::Active {
            return Err(TriangleError::InvalidTriangle(
                "Triangle must be active to be subdivided".to_string(),
            ));
        }

        let (child_triangle1, child_triangle2, child_triangle3) = subdivide_triangle(&self.triangle)?;

        let child1_address = self.address.append(0);
        let child2_address = self.address.append(1);
        let child3_address = self.address.append(2);

        let child1 = FractalTriangle::new(
            child_triangle1,
            TriangleState::Active,
            self.depth + 1,
            Some(self.id),
            child1_address,
        );
        let child2 = FractalTriangle::new(
            child_triangle2,
            TriangleState::Active,
            self.depth + 1,
            Some(self.id),
            child2_address,
        );
        let child3 = FractalTriangle::new(
            child_triangle3,
            TriangleState::Active,
            self.depth + 1,
            Some(self.id),
            child3_address,
        );

        self.state = TriangleState::Subdivided;
        self.child_ids.push(child1.id);
        self.child_ids.push(child2.id);
        self.child_ids.push(child3.id);

        Ok(vec![child1, child2, child3])
    }
}
