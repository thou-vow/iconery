CREATE TABLE products (
  id BIGINT PRIMARY KEY AUTO_INCREMENT,
  name VARCHAR(255) NOT NULL,
  description TEXT NULL,
  price BIGINT NOT NULL,
  is_featured BIT(1) NOT NULL
);

CREATE TABLE customers (
  id BIGINT PRIMARY KEY AUTO_INCREMENT,
  name VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL,
  password VARCHAR(255) NOT NULL,
  phone_number VARCHAR(255),
  address VARCHAR(255),
  is_active BIT(1) NOT NULL
);

CREATE TABLE orders (
  id BIGINT PRIMARY KEY AUTO_INCREMENT,
  customer_id BIGINT NOT NULL,
  FOREIGN KEY (customer_id) REFERENCES customers(id) ON DELETE CASCADE
);

CREATE TABLE order_items (
  id BIGINT PRIMARY KEY AUTO_INCREMENT,
  order_id BIGINT NOT NULL,
  product_id BIGINT NOT NULL,
  amount BIGINT NOT NULL,
  FOREIGN KEY (order_id) REFERENCES orders(id) ON DELETE CASCADE,
  FOREIGN KEY (product_id) REFERENCES products(id),
  INDEX (order_id)
);
  
INSERT INTO products (name, description, price, is_featured) VALUES
('Baú Aberto', 'Baú aberto', 10, 1),
('Cabeça de Dragão', 'Cabeça de dragão', 5, 1),
('Poção de Vida', 'Poção de vida', 3, 1),
('Escudo', 'Escudo', 2, 0),
('Elmo', 'Elmo', 10, 0),
('Chave', 'Chave', 9, 0),
('Bússola', 'Bússola', 1, 0),
('Duas Moedas', 'Duas moedas', 100, 0);

-- Clientes (customers)
INSERT INTO customers (name, email, password, phone_number, address, is_active) VALUES
('João Silva',     'joao.silva@example.com',     'senha_joao',   '11999990001', 'R. das Laranjeiras, 10, São Paulo - SP', 1),
('Maria Santos',   'maria.santos@example.com',  'senha_maria',  '21988880002', 'Av. Rio Branco, 123, Rio de Janeiro - RJ', 1),
('Pedro Almeida',   'pedro.almeida@example.com', 'senha_pedro',  '31977770003', 'R. das Oliveiras, 45, Belo Horizonte - MG', 0),
('Ana Costa',      'ana.costa@example.com',     'senha_ana',    '11977776666', 'R. das Flores, 7, São Paulo - SP', 1),
('Empresa ABC',    'vendas@empresaabc.com',     'senha_empresa','1133334444',  'Av. Paulista, 1000, São Paulo - SP', 1),
('Lucas Pereira',  'lucas.pereira@example.com', 'senha_lucas',  '51999990004', 'R. das Acácias, 200, Porto Alegre - RS', 1);

-- Pedidos (orders) — cada linha cria um pedido associado a um customer_id
INSERT INTO orders (customer_id) VALUES
(1), -- pedido 1: João
(2), -- pedido 2: Maria
(1), -- pedido 3: João (segundo pedido)
(4), -- pedido 4: Ana
(5), -- pedido 5: Empresa ABC
(3), -- pedido 6: Pedro (cliente inativo)
(6), -- pedido 7: Lucas
(2); -- pedido 8: Maria (segundo pedido)

-- Itens de pedido (order_items)
INSERT INTO order_items (order_id, product_id, amount) VALUES
-- Pedido 1 (João)
(1, 3, 2),  -- 2 x Poção de Vida
(1, 8, 5),  -- 5 x Duas Moedas

-- Pedido 2 (Maria)
(2, 5, 1),  -- 1 x Elmo
(2, 4, 2),  -- 2 x Escudo

-- Pedido 3 (João)
(3, 2, 1),  -- 1 x Cabeça de Dragão
(3, 1, 3),  -- 3 x Baú Aberto

-- Pedido 4 (Ana)
(4, 6, 2),  -- 2 x Chave

-- Pedido 5 (Empresa ABC)
(5, 7, 10), -- 10 x Bússola
(5, 5, 2),  -- 2 x Elmo

-- Pedido 6 (Pedro — cliente inativo)
(6, 1, 1),  -- 1 x Baú Aberto
(6, 3, 1),  -- 1 x Poção de Vida

-- Pedido 7 (Lucas)
(7, 4, 1),  -- 1 x Escudo
(7, 8, 20), -- 20 x Duas Moedas (ex.: compra em atacado)
(7, 2, 1),  -- 1 x Cabeça de Dragão

-- Pedido 8 (Maria — segundo pedido)
(8, 3, 4),  -- 4 x Poção de Vida
(8, 6, 2);  -- 2 x Chave
