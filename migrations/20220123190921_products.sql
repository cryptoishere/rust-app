-- Add migration script here

create table if not exists products (
    product_no integer PRIMARY KEY,
    name text,
    price numeric
);