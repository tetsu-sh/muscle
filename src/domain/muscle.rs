
#[derive(Debug,Clone)]
pub struct Muscle{
    name:MuscleName,
    position:BodyPosition,
    size:MuscleSize
}

impl Muscle {
    fn new(name:MuscleName,position:BodyPosition,size:MuscleSize)->Self{
        Self{
            name,
            position,
            size,
        }

    }
}

#[derive(Debug,Clone)]
enum BodyPosition{
    Arm,
    Back,
    Leg,
    Chest,
    Tolso,
}

#[derive(Debug,Clone)]
struct MuscleName(String);


#[derive(Debug,Clone)]
enum MuscleSize{
    Large,
    Middle,
    Small,
}
