CREATE TABLE users(
    id serial primary key,
    name varchar(255) NOT NULL unique,
    display_name varchar(255),
    avatar text,
    bio text,
    local boolean DEFAULT true NOT NULL,
    private_key text,
    public_key text NOT NULL,
    last_refreshed_at timestamp without time zone DEFAULT now() NOT NULL,
    inbox_url varchar(255) DEFAULT '',
    shared_inbox_url varchar(255),
    admin boolean DEFAULT false NOT NULL,
    bot_account boolean DEFAULT false NOT NULL,
    updated_at timestamp without time zone
);