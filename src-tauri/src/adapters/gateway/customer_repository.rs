use std::error::Error;
use sqlx::{pool::PoolConnection, Sqlite, SqlitePool, sqlite::SqliteQueryResult, Execute};
use time::PrimitiveDateTime;

use crate::{application::{repository::customer_repository::{CustomerAbstructRepository, CreateCustomerResult, UpdateCustomerResult}, usecase::{customer::{create_customer::CreateCustomerInput, update_customer::UpdateCustomerInput}}}, domain::customer::{Customer, Id}};

#[derive(sqlx::FromRow)]
pub struct CustomerRow {
    id: i64,
    name: String,
    postal: i64,
    address: String,
    created_at: PrimitiveDateTime,
    updated_at: PrimitiveDateTime,
    deleted_at: Option<PrimitiveDateTime>,
}

pub struct SqliteCustomerRespository {
    pool: SqlitePool,
}
impl From<SqlitePool> for SqliteCustomerRespository {
    fn from(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl CustomerAbstructRepository for SqliteCustomerRespository {

    async fn find_by_id(&self, id: &Id) -> Result<Option<Customer>, Box<dyn Error>> {
        let mut conn = self.pool.acquire().await?;
        let cusotmer = CustomerRepository::find_by_id(&mut conn, id).await?;
        
        Ok(cusotmer)
    }

    async fn create(&self, input: CreateCustomerInput) -> Result<CreateCustomerResult, Box<dyn Error>> {
        let mut conn = self.pool.acquire().await?;
        let result = CustomerRepository::create(&mut conn, input).await?;

        Ok(CreateCustomerResult::from(result.last_insert_rowid()))
    }

    async fn update(&self, input: UpdateCustomerInput) -> Result<UpdateCustomerResult, Box<dyn Error>> {
        let mut conn = self.pool.acquire().await?;
        CustomerRepository::update(&mut conn, &input).await?;

        Ok(UpdateCustomerResult::from(input.id()))
    }
}

pub struct CustomerRepository {}
impl CustomerRepository {
    async fn find_by_id(conn: &mut PoolConnection<Sqlite>, id: &Id) -> Result<Option<Customer>, Box<dyn Error>> {
        let result = sqlx::query_as::<Sqlite, CustomerRow>("SELECT * FROM m_customers WHERE id = ?").bind(id).fetch_optional(conn).await?;

        match result {
            Some(row) => {
                Ok(Some(Customer::new(row.id, row.name, row.postal, row.address, row.created_at, row.updated_at, row.deleted_at)))
            }
            None => Ok(None)
        }
    }

    async fn create(conn: &mut PoolConnection<Sqlite>, input: CreateCustomerInput) -> Result<SqliteQueryResult, Box<dyn Error>> {
        let result = sqlx::query("INSERT INTO m_customers (name, postal, address) VALUES (?, ?, ?)")
            .bind(input.name())
            .bind(input.postal())
            .bind(input.address())
            .execute(conn).await?;

        Ok(result)
    }

    async fn update(conn: &mut PoolConnection<Sqlite>, input: &UpdateCustomerInput) -> Result<Option<SqliteQueryResult>, Box<dyn Error>> {
        let mut pre_query_builder = sqlx::query_builder::QueryBuilder::<Sqlite>::new("UPDATE m_customers SET");

        let mut separated = pre_query_builder.separated(",");
        if let Some(name) = input.name() {
            separated.push("name = ");
            separated.push_bind_unseparated(name);
        }

        if let Some(postal) = input.postal() {
            separated.push("postal = ");
            separated.push_bind_unseparated(postal);
        }

        if let Some(address) = input.address() {
            separated.push("address = ");
            separated.push_bind_unseparated(address);
        }

        let query_without_where = pre_query_builder.build();
        if query_without_where.sql().ends_with("= ?") {
            return Ok(None);
        }

        let mut query_builder = sqlx::query_builder::QueryBuilder::<Sqlite>::new(query_without_where.sql());
        query_builder.push(" WHERE id = ");
        query_builder.push_bind(input.id());

        let query = query_builder.build();
        let result = sqlx::query(query.sql()).execute(conn).await?;

        Ok(Some(result))
    }
}

#[cfg(test)]
mod tests {

    use sqlx::SqlitePool;

    use crate::{infrastructure::database::MIGRATOR, adapters::gateway::customer_repository::SqliteCustomerRespository, application::{repository::customer_repository::CustomerAbstructRepository, usecase::customer::create_customer::CreateCustomerInput}};

    #[sqlx::test(migrator = "MIGRATOR")]
    async fn find_by_id_test(pool: SqlitePool)  {
        let repository = SqliteCustomerRespository::from(pool);
        let input = CreateCustomerInput::new(String::from("sample.inc"), 1234567, String::from("東京都"));
        let result = repository.create(input).await.unwrap();

        assert_eq!(*result.customer_id(), 1);
    }

    #[sqlx::test(migrator = "MIGRATOR")]
    async fn create_test(pool: SqlitePool)  {
        let repository = SqliteCustomerRespository::from(pool);
        let input = CreateCustomerInput::new(String::from("sample.inc"), 1234567, String::from("東京都"));
        let result = repository.create(input).await.unwrap();
        let product = repository.find_by_id(result.customer_id()).await.unwrap();

        assert_eq!(product.is_some(), true);
    }
}