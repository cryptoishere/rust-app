-- Add migration script here

create table if not exists order_service_items (
    product_no integer REFERENCES services ON DELETE RESTRICT,
    order_id integer REFERENCES orders ON DELETE CASCADE,
    quantity integer,
    PRIMARY KEY (product_no, order_id)
);