CREATE TABLE user_ (
    id serial primary key,
    name character varying(255) NOT NULL unique,
    display_name character varying(255),
    avatar text,
    banned boolean DEFAULT false NOT NULL,
    published timestamp without time zone DEFAULT now() NOT NULL,
    updated timestamp without time zone,
    actor_id character varying(255) DEFAULT public.generate_unique_changeme() NOT NULL,
    bio text,
    local boolean DEFAULT true NOT NULL,
    private_key text,
    public_key text NOT NULL,
    last_refreshed_at timestamp without time zone DEFAULT now() NOT NULL,
    banner text,
    deleted boolean DEFAULT false NOT NULL,
    inbox_url character varying(255) DEFAULT public.generate_unique_changeme() NOT NULL,
    shared_inbox_url character varying(255),
    matrix_user_id text,
    admin boolean DEFAULT false NOT NULL,
    bot_account boolean DEFAULT false NOT NULL,
    ban_expires timestamp without time zone
);

create table user_ban (
  id serial primary key,
  user_id int references user_ on update cascade on delete cascade not null,
  published timestamp not null default now(),
  unique (user_id)
);
