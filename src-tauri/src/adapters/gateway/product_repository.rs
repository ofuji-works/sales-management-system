use crate::application::{
    repository::product_repository::{CreateProductResult, ProductAbstructRepository},
    usecase::product::{create_product::CreateProductInput, search_product::SearchProductInput},
};
use crate::domain::product::{
    DefaultPrice, Product, ProductCode, ProductId, ProductName, ProductUnit, StandardStockQuantity,
};
use async_trait::async_trait;
use sqlx::sqlite::SqliteQueryResult;
use sqlx::{pool::PoolConnection, Sqlite, SqlitePool};
use std::error::Error;
use time::PrimitiveDateTime;

#[derive(sqlx::FromRow, Debug)]
pub struct ProductRow {
    id: i64,
    name: String,
    code: String,
    unit: String,
    default_price: i64,
    standard_stock_quantity: i64,
    created_at: PrimitiveDateTime,
    updated_at: PrimitiveDateTime,
    deleted_at: Option<PrimitiveDateTime>,
}

pub struct SqliteProductRepository {
    pool: SqlitePool,
}

impl SqliteProductRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProductAbstructRepository for SqliteProductRepository {
    async fn find_by_id(&self, product_id: &ProductId) -> Result<Option<Product>, Box<dyn Error>> {
        let mut conn = self.pool.acquire().await?;
        let product = ProductRepository::find_by_id(&mut conn, &product_id).await?;

        Ok(product)
    }

    async fn search(&self, input: &SearchProductInput) -> Result<Vec<Product>, Box<dyn Error>> {
        let mut conn = self.pool.acquire().await?;
        let products = ProductRepository::search(&mut conn, input).await?;

        Ok(products)
    }

    async fn create(
        &self,
        product: &CreateProductInput,
    ) -> Result<CreateProductResult, Box<dyn Error>> {
        let mut conn = self.pool.acquire().await?;
        let result = ProductRepository::create(&mut conn, &product).await?;
        let product_id = ProductId::new(&result.last_insert_rowid());

        Ok(CreateProductResult { product_id })
    }

    async fn update(&self, product_id: &ProductId) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    async fn delete(&self, product_id: &ProductId) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

struct ProductRepository {}

impl ProductRepository {
    async fn find_by_id(
        conn: &mut PoolConnection<Sqlite>,
        product_id: &ProductId,
    ) -> Result<Option<Product>, Box<dyn Error>> {
        let row = sqlx::query_as::<_, ProductRow>("SELECT * FROM m_products WHERE id = ?")
            .bind(product_id.value())
            .fetch_optional(conn)
            .await?;

        match row {
            None => Ok(None),
            Some(row) => {
                let id = ProductId::new(&row.id);
                let name = ProductName::new(&row.name);
                let code = ProductCode::new(&row.code);
                let unit = ProductUnit::new(&row.unit);
                let default_price = DefaultPrice::new(&row.default_price);
                let standard_stock_quantity =
                    StandardStockQuantity::new(&row.standard_stock_quantity);
                let product = Product::new(
                    id,
                    name,
                    code,
                    unit,
                    default_price,
                    standard_stock_quantity,
                    row.created_at,
                    row.updated_at,
                    row.deleted_at,
                );

                Ok(Some(product))
            }
        }
    }

    async fn search(
        conn: &mut PoolConnection<Sqlite>,
        input: &SearchProductInput,
    ) -> Result<Vec<Product>, Box<dyn Error>> {
        let rows: Vec<ProductRow> = sqlx::query_as(
            "SELECT * FROM m_products WHERE name = COALESCE(?, name) AND code = COALESCE(?, code) LIMIT ? OFFSET ?",
        )
        .bind(input.name())
        .bind(input.code())
        .bind(input.limit())
        .bind(input.offset())
        .fetch_all(conn)
        .await?;
        let products = rows
            .into_iter()
            .map(|row| {
                let id = ProductId::new(&row.id);
                let name = ProductName::new(&row.name);
                let code = ProductCode::new(&row.code);
                let unit = ProductUnit::new(&row.unit);
                let default_price = DefaultPrice::new(&row.default_price);
                let standard_stock_quantity =
                    StandardStockQuantity::new(&row.standard_stock_quantity);

                Product::new(
                    id,
                    name,
                    code,
                    unit,
                    default_price,
                    standard_stock_quantity,
                    row.created_at,
                    row.updated_at,
                    row.deleted_at,
                )
            })
            .collect();

        Ok(products)
    }

    async fn create(
        conn: &mut PoolConnection<Sqlite>,
        product: &CreateProductInput,
    ) -> Result<SqliteQueryResult, Box<dyn Error>> {
        let result = sqlx::query(
            "INSERT INTO m_products (
                name, 
                code, 
                unit, 
                default_price, 
                standard_stock_quantity
            ) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(product.name.value())
        .bind(product.code.value())
        .bind(product.unit.value())
        .bind(product.default_price.value())
        .bind(product.standard_stock_quantity.value())
        .execute(conn)
        .await?;

        Ok(result)
    }

    async fn update(&self, product_id: &ProductId) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    async fn delete(&self, product_id: &ProductId) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use sqlx::SqlitePool;

    use crate::{
        adapters::gateway::product_repository::{ProductRepository, SqliteProductRepository},
        application::{
            repository::product_repository::ProductAbstructRepository,
            usecase::product::{
                create_product::CreateProductInput, search_product::SearchProductInput,
            },
        },
        infrastructure::database::MIGRATOR,
    };

    #[sqlx::test(migrator = "MIGRATOR")]
    async fn search_test(pool: SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
        let repository = SqliteProductRepository::new(pool);
        let input = CreateProductInput::new(
            String::from("商品1"),
            String::from("product001"),
            String::from("個"),
            2000,
            10,
        );
        repository.create(&input).await?;
        let product_name = String::from("商品1");
        let input = SearchProductInput::new(None, None, Some(product_name), None);
        let products = repository.search(&input).await?;
        println!("search_test: {:?}", products);

        Ok(())
    }

    #[sqlx::test(migrator = "MIGRATOR")]
    async fn create_test(pool: SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
        let input = CreateProductInput::new(
            String::from("商品1"),
            String::from("product001"),
            String::from("個"),
            2000,
            10,
        );
        let mut conn = pool.acquire().await?;
        let result = ProductRepository::create(&mut conn, &input).await?;

        assert_eq!(result.rows_affected(), 1);

        Ok(())
    }

    #[sqlx::test(migrator = "MIGRATOR")]
    async fn find_by_id_test(pool: SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
        let repository = SqliteProductRepository::new(pool);
        let input = CreateProductInput::new(
            String::from("商品1"),
            String::from("product001"),
            String::from("個"),
            2000,
            10,
        );
        let result = repository.create(&input).await?;
        repository.create(&input).await?;
        let product_id = result.product_id();
        let product = repository.find_by_id(&product_id).await?;

        Ok(())
    }
}
