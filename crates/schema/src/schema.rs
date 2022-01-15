table! {
    local_user (id) {
        id -> Int4,
        user_id -> Int4,
        password_encrypted -> Text,
        email -> Nullable<Text>,
        show_nsfw -> Bool,
        theme -> Varchar,
        default_sort_type -> Int2,
        default_listing_type -> Int2,
        lang -> Varchar,
        show_avatars -> Bool,
        send_notifications_to_email -> Bool,
        validator_time -> Timestamp,
        show_scores -> Bool,
        show_bot_accounts -> Bool,
        show_read_posts -> Bool,
        show_new_post_notifs -> Bool,
        email_verified -> Bool,
        accepted_application -> Bool,
    }
}

table! {
    user_ (id) {
        id -> Int4,
        name -> Varchar,
        display_name -> Nullable<Varchar>,
        avatar -> Nullable<Text>,
        banned -> Bool,
        published -> Timestamp,
        updated -> Nullable<Timestamp>,
        actor_id -> Varchar,
        bio -> Nullable<Text>,
        local -> Bool,
        private_key -> Nullable<Text>,
        public_key -> Text,
        last_refreshed_at -> Timestamp,
        banner -> Nullable<Text>,
        deleted -> Bool,
        inbox_url -> Varchar,
        shared_inbox_url -> Nullable<Varchar>,
        matrix_user_id -> Nullable<Text>,
        admin -> Bool,
        bot_account -> Bool,
        ban_expires -> Nullable<Timestamp>,
    }
}

table! {
    user_ban (id) {
        id -> Int4,
        user_id -> Int4,
        published -> Timestamp,
    }
}

joinable!(local_user -> user_ (user_id));
joinable!(user_ban -> user_ (user_id));

allow_tables_to_appear_in_same_query!(
    local_user,
    user_,
    user_ban,
);
