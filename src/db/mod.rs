use diesel::r2d2::{self, PooledConnection, ConnectionManager};
use diesel::prelude::*;

mod models;
mod schema;

pub type Conn = PooledConnection<ConnectionManager<diesel::PgConnection>>;

pub fn query(item: i32, conn: &Conn) -> Result<models::NewsItem, diesel::result::Error> {
    use self::schema::newsitems::dsl::*;

    let mut item = newsitems.filter(id.eq(&item)).load::<models::NewsItem>(conn)?;
    Ok(item.pop().unwrap())
}

pub fn get_all(conn: &Conn) -> Result<Vec<models::NewsItem>, diesel::result::Error> {
    use self::schema::newsitems::dsl::*;

    let items = newsitems.load::<models::NewsItem>(conn)?;
    Ok(items)
}

pub fn additem(
    newstitle: String, conn: &Conn
) -> Result<models::NewsItem, diesel::result::Error> {
    use self::schema::newsitems::dsl::*;

    let new_item = models::InsertableNewsItem{
        title: newstitle.as_str()
    };

    let item: i32 = diesel::insert_into(newsitems)
        .values(&new_item)
        .returning(id)
        .get_results(conn)
        .unwrap()
        .pop()
        .unwrap();

    Ok(models::NewsItem{
        id: item,
        title: newstitle
    })
}
