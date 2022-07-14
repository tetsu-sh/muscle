
struct MuscleUsecase{
    // TODO repository
}

impl MuscleUsecase{
    fn create(name:MuscleName,position:BodyPosition,size:MuscleSize){
        let new_muscle=muscle::new(name,position,size);
    }
}