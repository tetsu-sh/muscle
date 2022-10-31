#[derive(Debug, Clone, PartialEq)]
pub struct Muscle {
    name: String,
    position: BodyPosition,
    size: MuscleSize,
}

impl Muscle {
    pub fn new(name: String, position: BodyPosition, size: MuscleSize) -> Self {
        Self {
            name,
            position,
            size,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BodyPosition {
    Arm,
    Back,
    Leg,
    Chest,
    Tolso,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MuscleSize {
    Large,
    Middle,
    Small,
}
