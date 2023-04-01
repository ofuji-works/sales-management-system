use crate::{
    adapters::{
        controller::{product_controller, request::product_request::CreateProductRequest},
        gateway::product_repository::SqliteProductRepository,
        presenter::{
            product_presenter,
            response::product_response::{CreateProductResponse, SearchProductResponse},
        },
    },
    application::usecase::{
        product::create_product::CreateProductUsecase,
        product::{
            find_by_id_product::FindByIDProductUsecase, search_product::SearchProductUsecase,
        },
    },
};
use sqlx::SqlitePool;
use std::{error::Error, rc::Rc};

pub(crate) async fn search(pool: SqlitePool) -> Result<SearchProductResponse, Box<dyn Error>> {
    let repository = SqliteProductRepository::new(pool);
    let usecase = SearchProductUsecase::new(Rc::new(repository));
    let output = product_controller::search_product(usecase).await?;

    Ok(product_presenter::search_product(output))
}

#[tauri::command]
pub(crate) fn search_product(
    state: tauri::State<'_, SqlitePool>,
) -> Result<SearchProductResponse, String> {
    let pool = state.inner().clone();
    let result = tauri::async_runtime::block_on(search(pool)).map_err(|e| e.to_string())?;

    Ok(result)
}

pub(crate) async fn create(
    pool: SqlitePool,
    request: CreateProductRequest,
) -> Result<CreateProductResponse, Rc<dyn Error>> {
    let repository = Rc::new(SqliteProductRepository::new(pool));
    let create_product_usecase = CreateProductUsecase::new(repository.clone());
    let output = product_controller::create_product(create_product_usecase, request).await?;

    let find_by_id_product_usecase = FindByIDProductUsecase::new(repository.clone());
    let product = find_by_id_product_usecase
        .find_by_id(output.result().product_id())
        .await?
        .product;

    Ok(product_presenter::create_product(output, product))
}

#[tauri::command]
pub(crate) fn create_product(
    state: tauri::State<'_, SqlitePool>,
    request: CreateProductRequest,
) -> Result<CreateProductResponse, String> {
    let pool = state.inner().clone();
    let result = tauri::async_runtime::block_on(create(pool, request)).map_err(|e| e.to_string());

    result
}

#[cfg(test)]
mod tests {
    use crate::{infrastructure::database::MIGRATOR, product::search};
    use sqlx::SqlitePool;

    #[sqlx::test(migrator = "MIGRATOR")]
    async fn search_product_test(pool: SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = pool.acquire().await?;
        sqlx::query(
            "INSERT INTO
                m_products (
                    name,
                    code,
                    unit,
                    default_price,
                    standard_stock_quantity
                ) 
                VALUES (
                    \"商品1\",
                    \"product001\",
                    \"個\",
                    2000,
                    10
            )",
        )
        .execute(&mut conn)
        .await?;
        let response = search(pool).await?;
        println!("{:?}", response);

        Ok(())
    }
}
