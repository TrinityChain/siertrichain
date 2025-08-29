use serde::{Deserialize, Serialize};
use crate::core::triangle::Triangle;
use crate::core::address::TriangleAddress;
use crate::core::hash::H256;
use ed25519_dalek::{Signature, Signer, Keypair, PublicKey, Verifier};
use crate::core::fractal::{FractalTriangle, TriangleState};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TriangleOperation {
    Create(Triangle),
    Subdivide { 
        parent: TriangleAddress, 
        children: [Triangle; 3] 
    },
    Transfer {
        from: TriangleAddress,
        to: PublicKey,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub operation: TriangleOperation,
    pub signature: Signature,
    pub public_key: PublicKey,
    hash: H256, // Cached hash
}

impl Transaction {
    pub fn new(operation: TriangleOperation, keypair: &Keypair) -> Self {
        let public_key = keypair.public;
        let operation_bytes = bincode::serialize(&operation).unwrap();
        let signature = keypair.sign(&operation_bytes);
        let hash = blake3::hash(&operation_bytes).into();
        Self {
            operation,
            signature,
            public_key,
            hash,
        }
    }

    pub fn new_genesis(triangle: Triangle) -> Self {
        // Create a dummy keypair for the genesis transaction
        let mut csprng = rand::thread_rng();
        let keypair = Keypair::generate(&mut csprng);
        Self::new(TriangleOperation::Create(triangle), &keypair)
    }

    pub fn validate(&self) -> bool {
        let operation_bytes = bincode::serialize(&self.operation).unwrap();
        self.public_key.verify(&operation_bytes, &self.signature).is_ok()
    }

    pub fn hash(&self) -> &H256 {
        &self.hash
    }

    pub fn get_fractal_triangles(&self) -> Vec<FractalTriangle> {
        let mut triangles = Vec::new();
        match &self.operation {
            TriangleOperation::Create(triangle) => {
                let address = TriangleAddress::new(Vec::new());
                triangles.push(FractalTriangle::new(triangle.clone(), TriangleState::Genesis, 0, None, address));
            }
            TriangleOperation::Subdivide { parent, children } => {
                for (i, child) in children.iter().enumerate() {
                    let address = parent.append(i as u8);
                    // This is not quite right, we need to get the parent from the blockchain
                    // to get the correct depth and parent_id.
                    // For now, we will just use a depth of 0 and no parent.
                    triangles.push(FractalTriangle::new(child.clone(), TriangleState::Active, 0, None, address));
                }
            }
            _ => {}
        }
        triangles
    }

}
