use crate::application::{
    repository::product_repository::{
        CreateProductResult, ProductAbstructRepository, UpdateProductResult,
    },
    usecase::product::{
        create_product::CreateProductInput, search_product::SearchProductInput,
        update_product::UpdateProductInput,
    },
};
use crate::domain::product::{
    Product, ProductCode, ProductDefaultPrice, ProductId, ProductName,
    ProductStandardStockQuantity, ProductUnit,
};
use async_trait::async_trait;
use sqlx::{pool::PoolConnection, Execute, Sqlite, SqlitePool};
use sqlx::{query_builder, sqlite::SqliteQueryResult};
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
        let create_product_result = CreateProductResult::new(product_id);

        Ok(create_product_result)
    }

    async fn update(
        &self,
        input: &UpdateProductInput,
    ) -> Result<UpdateProductResult, Box<dyn Error>> {
        let mut conn = self.pool.acquire().await?;
        let result = ProductRepository::update(&mut conn, input).await?;
        let update_product_result = UpdateProductResult::new(input.id().clone());

        Ok(update_product_result)
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
                let default_price = ProductDefaultPrice::new(&row.default_price);
                let standard_stock_quantity =
                    ProductStandardStockQuantity::new(&row.standard_stock_quantity);
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
                let default_price = ProductDefaultPrice::new(&row.default_price);
                let standard_stock_quantity =
                    ProductStandardStockQuantity::new(&row.standard_stock_quantity);

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

    async fn update(
        conn: &mut PoolConnection<Sqlite>,
        input: &UpdateProductInput,
    ) -> Result<Option<SqliteQueryResult>, Box<dyn Error>> {
        let mut pre_query_builder =
            query_builder::QueryBuilder::<Sqlite>::new("UPDATE m_products SET ");

        let mut separated = pre_query_builder.separated(", ");
        if let Some(name) = input.name() {
            separated.push("name = ");
            separated.push_bind_unseparated(name.value());
        }
        if let Some(code) = input.code() {
            separated.push("code = ");
            separated.push_bind_unseparated(code.value());
        }
        if let Some(unit) = input.unit() {
            separated.push("unit = ");
            separated.push_bind_unseparated(unit.value());
        }
        if let Some(default_price) = input.default_price() {
            separated.push("default_price = ");
            separated.push_bind_unseparated(default_price.value());
        }
        if let Some(standard_stock_quantity) = input.standard_stock_quantity() {
            separated.push("standard_stock_quantity = ");
            separated.push_bind_unseparated(standard_stock_quantity.value());
        }

        let query_without_where = pre_query_builder.build();
        let is_update_colums = query_without_where.sql().ends_with("= ?");
        if !is_update_colums {
            return Ok(None);
        }

        let mut query_builder =
            query_builder::QueryBuilder::<Sqlite>::new(query_without_where.sql());
        query_builder.push(" WHERE id = ");
        query_builder.push_bind(input.id().value());
        let query = query_builder.build();
        let result = sqlx::query(&query.sql()).execute(conn).await?;

        Ok(Some(result))
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
                update_product::UpdateProductInput,
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
    async fn update_test(pool: SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
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
        let params = UpdateProductInput::new(
            product_id.value(),
            Some(String::from("商品1更新後")),
            Some(String::from("product001更新後")),
            Some(String::from("個更新後")),
            None,
            None,
            // None,
            // None,
            // None,
        );
        let result = repository.update(&params).await?;

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
