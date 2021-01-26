table! {
    content (page) {
        groupname -> Nullable<Varchar>,
        page -> Varchar,
        contentbody -> Nullable<Varchar>,
    }
}

table! {
    groups (name) {
        name -> Varchar,
    }
}

table! {
    privillege (user_id, groupname) {
        user_id -> Int4,
        groupname -> Varchar,
        rights -> Bpchar,
    }
}

table! {
    sessions (session_id) {
        session_id -> Varchar,
        user_id -> Int4,
        expire -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
    }
}

joinable!(content -> groups (groupname));
joinable!(privillege -> groups (groupname));
joinable!(privillege -> users (user_id));
joinable!(sessions -> users (user_id));

allow_tables_to_appear_in_same_query!(
    content,
    groups,
    privillege,
    sessions,
    users,
);
