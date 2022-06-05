-- Add migration script here

create table if not exists orders (
    order_id integer PRIMARY KEY,
    shipping_address text
);