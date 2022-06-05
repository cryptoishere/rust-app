-- Add migration script here

CREATE TABLE orders (
    order_id integer PRIMARY KEY,
    shipping_address text
);