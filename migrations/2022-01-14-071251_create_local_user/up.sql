CREATE TABLE local_users (
    id serial primary key,
    user_id int references users on update cascade on delete cascade not null unique,
    password_encrypted text NOT NULL,
    email text unique,
    show_avatars boolean DEFAULT true NOT NULL,
    send_notifications_to_email boolean DEFAULT false NOT NULL,
    validator_time timestamp without time zone DEFAULT now() NOT NULL,
    show_bot_accounts boolean DEFAULT true NOT NULL,
    email_verified boolean DEFAULT false NOT NULL,
    accepted_application boolean DEFAULT false NOT NULL
);