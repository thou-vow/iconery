CREATE TABLE products (
  id BIGINT PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  description TEXT NULL,
  price BIGINT NOT NULL,
  is_featured BIT(1) NOT NULL
);

CREATE TABLE customers (
  id BIGINT PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL,
  password VARCHAR(255) NOT NULL,
  phone_number VARCHAR(255),
  address VARCHAR(255),
  is_active BIT(1) NOT NULL
);
