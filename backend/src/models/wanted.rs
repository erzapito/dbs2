use crate::database::PoolType;
use crate::errors::ApiError;
use crate::schema::wanted;
use diesel::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable)]
#[table_name = "wanted"]
pub struct Wanted {
    pub id: i32,
    pub artist: String,
    pub disc: String,
    pub done: i32,
    pub weeks: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, Insertable)]
#[table_name = "wanted"]
pub struct NewWanted {
    pub artist: String,
    pub disc: String,
    pub done: i32,
    pub weeks: i32,
}

pub fn get_all(pool: &PoolType) -> Result<Vec<Wanted>, ApiError> {
    use crate::schema::wanted::dsl::{wanted, done, weeks};

    let conn = pool.get()?;
    let result = wanted
      .filter(weeks.gt(-1))
      .order((done.asc(),weeks.desc()))
      .load(&conn)?;

    Ok(result.into())
}

pub fn count_all(pool: &PoolType) -> Result<i64, ApiError> {
    use crate::schema::wanted::dsl::{wanted, done};

    let conn = pool.get()?;
    let result = wanted.count()
      .filter(done.eq(0))
      .get_result(&conn)?;

    Ok(result)
}

#[cfg(test)]
pub fn find(pool: &PoolType, item_id: i32) -> Result<Wanted, ApiError> {
    use crate::schema::wanted::dsl::{id, wanted};

    let not_found = format!("Wanted {} not found", item_id);
    let conn = pool.get()?;
    let result = wanted
        .filter(id.eq(item_id))
        .first::<Wanted>(&conn)
        .map_err(|_| ApiError::NotFound(not_found))?;

    Ok(result)
}

#[cfg(test)]
pub fn create(pool: &PoolType, new_item: &NewWanted) -> Result<(), ApiError> {
    use crate::schema::wanted::dsl::wanted;

    let conn = pool.get()?;
    diesel::insert_into(wanted).values(new_item).execute(&conn)?;
    Ok(())
}

pub fn mark_as_downloaded (pool: &PoolType, item_id: i32) -> Result<(), ApiError> {
    use crate::schema::wanted::dsl::{wanted, id, weeks};

    let conn = pool.get()?;
    diesel::update(wanted)
        .filter(id.eq(item_id))
        .set(weeks.eq(-1))
        .execute(&conn)?;
    Ok(())
}

pub fn increase_wanted_week (pool: &PoolType, item_id: i32) -> Result<(), ApiError> {
    use crate::schema::wanted::dsl::{wanted, id, weeks, done};

    let conn = pool.get()?;
    diesel::update(wanted)
        .filter(id.eq(item_id))
        .set( ( weeks.eq( weeks + 1 ), done.eq(1) ) )
        .execute(&conn)?;
    Ok(())    
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::tests::helpers::tests::init_test_pool;

    pub fn create_wanted(pool: &PoolType) -> Result<(), ApiError> {
        let new_item = NewWanted {
            artist: "Artist".to_string(),
            disc: "Disc".to_string(),
            done: 0,
            weeks: 0,
        };
        create(pool, &new_item)
    }

    #[test]
    fn it_marks_a_wanted() {
        let pool = init_test_pool();
        let created = create_wanted(&pool);
        assert!(created.is_ok());
        let items = get_all(&pool).unwrap();
        let item = &items[0];
        assert_eq!(0, item.done);
        mark_as_downloaded(&pool, item.id).unwrap();
        let found_item = find(&pool, item.id).unwrap();
        assert_eq!(1, found_item.done);
    }

    #[test]
    fn it_increase_wanted_week() {
        let pool = init_test_pool();
        let created = create_wanted(&pool);
        assert!(created.is_ok());
        let items = get_all(&pool).unwrap();
        let item = &items[0];
        assert_eq!(0, item.weeks);
        increase_wanted_week(&pool, item.id).unwrap();
        let found_item = find(&pool, item.id).unwrap();
        assert_eq!(1, found_item.weeks);
        increase_wanted_week(&pool, item.id).unwrap();
        let found_item2 = find(&pool, item.id).unwrap();
        assert_eq!(2, found_item2.weeks);
    }

}
