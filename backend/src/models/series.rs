use crate::database::PoolType;
use crate::errors::ApiError;
use crate::handlers::series::SeriesResponse;
use crate::helpers::get_search_token;
use crate::schema::serie;
use diesel::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable)]
#[table_name = "serie"]
pub struct Series {
    pub id: i32,
    pub capitulos: String,
    pub categoria: String,
    pub fansub: String,
    pub idioma: String,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Insertable)]
#[table_name = "serie"]
pub struct NewSeries {
  pub capitulos: String,
  pub categoria: String,
  pub fansub: String,
  pub idioma: String,
  pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, AsChangeset)]
#[table_name = "serie"]
pub struct UpdateSeries {
    pub id: i32,
    pub capitulos: String,
    pub categoria: String,
    pub fansub: String,
    pub idioma: String,
    pub name: String,
}

/// Create a new user
pub fn create(pool: &PoolType, new_item: &NewSeries) -> Result<(), ApiError> {
    use crate::schema::serie::dsl::serie;

    let conn = pool.get()?;
    diesel::insert_into(serie).values(new_item).execute(&conn)?;
    Ok(())
}

pub fn delete(pool: &PoolType, item_id: i32) -> Result<(), ApiError> {
    use crate::schema::serie::dsl::{id, serie};

    let conn = pool.get()?;
    diesel::delete(serie)
        .filter(id.eq(item_id))
        .execute(&conn)?;
    Ok(())
}

pub fn find(pool: &PoolType, item_id: i32) -> Result<SeriesResponse, ApiError> {
    use crate::schema::serie::dsl::{id, serie};

    let not_found = format!("Serie {} not found", item_id);
    let conn = pool.get()?;
    let result = serie
        .filter(id.eq(item_id))
        .first::<Series>(&conn)
        .map_err(|_| ApiError::NotFound(not_found))?;

    Ok(result.into())
}

pub fn get_all(pool: &PoolType, src_search_token: &String, page: i64, page_size: i64) -> Result<Vec<Series>, ApiError> {
    use crate::schema::serie::dsl::{serie,name};

    let search_token = get_search_token( src_search_token );

    let conn = pool.get()?;
    let result = serie
      .filter( name.like(&search_token) )
      .order( name.asc() )
      .limit(page_size)
      .offset(page * page_size)
      .load(&conn)?;

    Ok(result.into())
}

pub fn count_all(pool: &PoolType, src_search_token: &String) -> Result<i64, ApiError> {
    use crate::schema::serie::dsl::{serie,name};

    let search_token = get_search_token( src_search_token );

    let conn = pool.get()?;
    let result = serie.count()
      .filter( name.like(&search_token) )
      .get_result(&conn)?;

    Ok(result)
}

pub fn update(pool: &PoolType, update_item: &UpdateSeries) -> Result<(), ApiError> {
    use crate::schema::serie::dsl::{id, serie};

    let conn = pool.get()?;
    diesel::update(serie)
        .filter(id.eq(update_item.id))
        .set(update_item)
        .execute(&conn)?;
    Ok(())
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::tests::helpers::tests::init_test_pool;

    pub fn get_all_series(pool: &PoolType) -> Result<Vec<Series>, ApiError> {
        get_all(&pool,&"".to_string(),0,10)
    }

    pub fn create_serie(pool: &PoolType) -> Result<(), ApiError> {
        let new_item = NewSeries {
            capitulos: "Capitulos".to_string(),
            categoria: "Categoria".to_string(),
            fansub: "Fansub".to_string(),
            idioma: "Idioma".to_string(),
            name: "Name".to_string(),
        };
        create(pool, &new_item)
    }

    #[test]
    fn it_creates_a_serie() {
        let pool = init_test_pool();

        let created = create_serie(&pool);
        assert!(created.is_ok());
        let items = get_all_series(&pool).unwrap();
        assert_eq!(1, items.len());
        let item = &items[0];
        let found_item = find(&pool, item.id).unwrap();
        assert_eq!("Capitulos", found_item.capitulos);
        assert_eq!("Categoria", found_item.categoria);
        assert_eq!("Fansub", found_item.fansub);
        assert_eq!("Idioma", found_item.idioma);
        assert_eq!("Name", found_item.name);
    }

    #[test]
    fn it_deletes_a_serie() {
        let pool = init_test_pool();
        let created = create_serie(&pool);
        assert!(created.is_ok());
        let items = get_all_series(&pool).unwrap();
        assert_eq!(1, items.len());
        let item = &items[0];
        let item_id = item.id;
        assert_eq!("Capitulos", item.capitulos);
        let item = find(&pool, item_id);
        assert!(item.is_ok());
        delete(&pool, item_id).unwrap();
        let item = find(&pool, item_id);
        assert!(item.is_err());
    }

    #[test]
    fn test_find() {
        let pool = init_test_pool();
        let created = create_serie(&pool);
        assert!(created.is_ok());
        let items = get_all_series(&pool).unwrap();
        let item = &items[0];
        let found_item = find(&pool, items[0].id).unwrap();
        assert_eq!(item.id, found_item.id);
    }

    #[test]
    fn it_doesnt_find() {
        let pool = init_test_pool();
        let item_id = -1;
        let not_found_item = find(&pool, item_id);
        assert!(not_found_item.is_err());
    }

    #[test]
    fn it_updates_a_serie() {
        let pool = init_test_pool();
        let created = create_serie(&pool);
        assert!(created.is_ok());
        let items = get_all_series(&pool).unwrap();
        let item = &items[0];
        let update_item = UpdateSeries {
            id: item.id,
            capitulos: "CapitulosUpdate".to_string(),
            categoria: "CategoriaUpdate".to_string(),
            fansub: "FansubUpdate".to_string(),
            idioma: "IdiomaUpdate".to_string(),
            name: "NameUpdate".to_string(),
        };
        let updated = update(&pool, &update_item);
        assert!(updated.is_ok());
        let found_item = find(&pool, item.id).unwrap();
        assert_eq!("NameUpdate", found_item.name);
    }

}
