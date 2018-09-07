-- Your SQL goes here

CREATE VIEW full_products_view AS
SELECT p.id, p.name, p.description, p.stock,
       json_agg(distinct pc.*) AS product_costs,
       json_agg(distinct pp.*) AS price_costs
       FROM products p
       LEFT JOIN (SELECT pc.*, c.name 
                  FROM product_costs AS pc 
                  INNER JOIN costs c ON pc.cost_id = c.id
                  INNER JOIN suppliers s ON pc.supplier_id = s.id) AS pc ON p.id = pc.product_id
       LEFT JOIN (SELECT pp.*, p.name 
                  FROM product_prices AS pp 
                  INNER JOIN prices p ON pp.price_id = p.id) AS pp ON p.id = pp.product_id
       GROUP BY p.id;