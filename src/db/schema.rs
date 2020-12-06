table! {
    answers (id) {
        id -> Int4,
        question_id -> Int4,
        body -> Text,
        created_at -> Timestamptz,
    }
}

table! {
    questions (id) {
        id -> Int4,
        body -> Text,
        hidden -> Bool,
        created_at -> Timestamptz,
    }
}

joinable!(answers -> questions (question_id));

allow_tables_to_appear_in_same_query!(
    answers,
    questions,
);
