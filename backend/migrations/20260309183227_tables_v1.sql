CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE STATUS AS ENUM('in progress', 'complete');

CREATE TABLE orders (
id UUID PRIMARY KEY uuid_generate_v4(),
product_name VARCHAR(100) NOT NULL,
product_count INT NOT NULL,
product_id uuid FOREIGN KEY(id) REFRENCES products(id),
order_name(50) VARCHAR NOT NULL,
order_phone_number(10) VARCHAR NOT NULL,
order_location POINT NOT NULL,
total_price INT NOT NULL,
status STATUS NOT NULL
);

CREATE TABLE products (
id UUID PRIMARY KEY uuid_generate_v4(),
seller_id UUID FOREIGN KEY(id) REFRENCES seller(id),
name VARCHAR(100) NOT NULL,
count INT NOT NULL,
price INT NOT NULL 
description TEXT,
image TEXT
);

CREATE TABLE seller (
id UUID PRIMARY KEY uuid_generate_v4(),
name VARCHAR(50) NOT NULL,
email VARCHAR(255) NOT NULL,
password VARCHAR(100) NOT NULL
);
