table! {
    cases (id) {
        id -> Uuid,
        data -> Jsonb,
    }
}

table! {
    events (id) {
        id -> Uuid,
        case_id -> Uuid,
        data -> Jsonb,
    }
}

joinable!(events -> cases (case_id));

allow_tables_to_appear_in_same_query!(
    cases,
    events,
);
