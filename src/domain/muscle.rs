
pub struct Muscle{
    name:MuscleName,
    position:BodyPosition,
    size:MuscleSize
}

enum BodyPosition{
    Arm,
    Back,
    Leg,
    Chest,
    Tolso,
}

struct MuscleName(String);


enum MuscleSize{
    large,
    middle,
    small,
}
