table! {
    accounts (id) {
        id -> Varchar,
        name -> Varchar,
    }
}

table! {
    body_conditions (id) {
        id -> Varchar,
        account_id -> Varchar,
        body_part_id -> Varchar,
        eccumulative_value -> Integer,
    }
}

table! {
    body_parts (id) {
        id -> Varchar,
        name -> Varchar,
    }
}

table! {
    body_profile (id) {
        id -> Varchar,
        account_id -> Varchar,
        weight -> Nullable<Integer>,
        height -> Nullable<Integer>,
    }
}

table! {
    muscles (id) {
        id -> Varchar,
        body_part_id -> Varchar,
        name -> Varchar,
        size -> Varchar,
    }
}

table! {
    muscle_damages (id) {
        id -> Varchar,
        account_id -> Varchar,
        muscle_id -> Varchar,
        value -> Integer,
    }
}

table! {
    trainees (id) {
        id -> Varchar,
        account_id -> Varchar,
    }
}

table! {
    trains (id) {
        id -> Varchar,
        account_id -> Varchar,
        name -> Varchar,
        volume -> Integer,
        rep -> Integer,
    }
}

table! {
    train_muscle (train_id, muscle_id) {
        train_id -> Varchar,
        muscle_id -> Varchar,
    }
}

joinable!(body_conditions -> accounts (account_id));
joinable!(body_conditions -> body_parts (body_part_id));
joinable!(body_profile -> accounts (account_id));
joinable!(muscle_damages -> accounts (account_id));
joinable!(muscle_damages -> muscles (muscle_id));
joinable!(muscles -> body_parts (body_part_id));
joinable!(train_muscle -> muscles (muscle_id));
joinable!(train_muscle -> trains (train_id));
joinable!(trainees -> accounts (account_id));
joinable!(trains -> accounts (account_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    body_conditions,
    body_parts,
    body_profile,
    muscles,
    muscle_damages,
    trainees,
    trains,
    train_muscle,
);
