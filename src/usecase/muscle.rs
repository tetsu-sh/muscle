struct MuscleUsecase {}

impl MuscleUsecase {
    pub fn create(name: MuscleName, position: BodyPosition, size: MuscleSize) {
        let new_muscle = muscle::new(name, position, size);
    }
}
