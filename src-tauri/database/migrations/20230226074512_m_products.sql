CREATE TABLE IF NOT EXISTS m_products (
    id int AUTO_INCREMENT,
    name VARCHAR(100) NOT NULL,
    code VARCHAR(100) NOT NULL,
    unit VARCHAR(5) NOT NULL,
    default_price int DEFAULT 0 NOT NULL,
    standard_stock_quantity int DEFAULT 0 NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP DEFAULT null,

   PRIMARY KEY (id)
);
