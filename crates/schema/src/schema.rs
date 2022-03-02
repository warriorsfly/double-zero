table! {
    local_users (id) {
        id -> Int4,
        user_id -> Int4,
        password_encrypted -> Text,
        email -> Nullable<Text>,
        show_avatars -> Bool,
        send_notifications_to_email -> Bool,
        validator_time -> Timestamp,
        show_bot_accounts -> Bool,
        email_verified -> Bool,
        accepted_application -> Bool,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        display_name -> Nullable<Varchar>,
        avatar -> Nullable<Text>,
        bio -> Nullable<Text>,
        local -> Bool,
        private_key -> Nullable<Text>,
        public_key -> Text,
        last_refreshed_at -> Timestamp,
        inbox_url -> Nullable<Varchar>,
        shared_inbox_url -> Nullable<Varchar>,
        admin -> Bool,
        bot_account -> Bool,
        updated_at -> Nullable<Timestamp>,
    }
}

joinable!(local_users -> users (user_id));

allow_tables_to_appear_in_same_query!(
    local_users,
    users,
);
