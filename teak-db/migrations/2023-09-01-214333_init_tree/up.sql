create extension "uuid-ossp";

create type "tree_type" as enum(
    'project',
    'folder',
    'inbox',
    'task',
    'document',
    'bug',
    'feature',
    'meeting'
);

create type "tree_state" as enum('open', 'closed');

create table
    "tree" (
        "id" uuid not null primary key default uuid_generate_v4 (),
        "path" uuid[] not null unique,
        "title" text not null,
        "type" "tree_type" not null default 'task',
        "state" "tree_state" not null default 'open',
        "created_at" timestamp without time zone not null default now(),
        "updated_at" timestamp without time zone not null default now(),
        "deleted_at" timestamp without time zone,
        check (array_length("path", 1) > 0),
        check ("path" [array_length("path", 1)] = "id")
    );

create index "tree_path_idx" on "tree" using gin ("path");

create table
    "graft" (
        "id" uuid not null primary key,
        "node_path" uuid[] not null references "tree" ("path"),
        "target_path" uuid[] not null references "tree" ("path"),
        "created_at" timestamp without time zone not null default now(),
        "updated_at" timestamp without time zone not null default now(),
        "deleted_at" timestamp without time zone
    );

create index "graft_node_path_idx" on "graft" using gin ("node_path");

create index "graft_target_path_idx" on "graft" using gin ("target_path");