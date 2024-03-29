struct Trainee {
    account_id: AccountId,
    name: TraineeName,
    weight: HumanWeight,
}

struct TraineeName(String);

struct HumanWeight(i8);

impl HumanWeight {
    fn new(weight: i8) {
        if weight < 0 || weight > 150 {
            return MyError::BAD_REQUEST();
        }
        Ok(Self(weight));
    }
}
