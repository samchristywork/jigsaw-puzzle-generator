use crate::vector::Vector;

#[derive(Copy, Clone)]
pub enum Kind {
    Offset,
    Scale,
    Rotate,
}

#[derive(Copy, Clone)]
pub struct Operation {
    pub kind: Kind,
    pub v: Vector,
}

pub struct Transform {
    pub operations: Vec<Operation>,
}

impl Transform {
    pub fn apply(&self, mut v: Vector) -> Vector {
        for operation in &self.operations {
            v = match operation.kind {
                Kind::Offset => v + operation.v,
                Kind::Scale => v * operation.v,
                Kind::Rotate => v.rotate(operation.v.x),
            }
        }
        v
    }
}
