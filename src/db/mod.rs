use diesel::r2d2::{PooledConnection, ConnectionManager};
use diesel::prelude::*;

pub mod models;
mod schema;

pub type Conn = PooledConnection<ConnectionManager<diesel::PgConnection>>;

pub fn query(item: i32, conn: &Conn) -> Result<models::Post, diesel::result::Error> {
    use self::schema::posts::dsl::*;

    let mut items = posts.filter(id.eq(&item)).load::<models::Post>(conn)?;
    if items.len() > 0 {
        Ok(items.pop().unwrap())
    } else {
        Err(diesel::result::Error::NotFound)
    }
}

pub fn get_all(conn: &Conn) -> Result<Vec<models::Post>, diesel::result::Error> {
    use self::schema::posts::dsl::*;

    let items = posts.load::<models::Post>(conn)?;
    Ok(items)
}

pub fn add_post(
    post: models::InputPost, conn: &Conn
) -> Result<models::Post, diesel::result::Error> {
    use self::schema::posts::dsl::*;

    let new_post = models::InsertablePost{
        title: post.title.as_str(),
        category: post.category.as_str(),
        content: post.content.as_str(),
        hidden: post.hidden
    };

    let item: i32 = diesel::insert_into(posts)
        .values(&new_post)
        .returning(id)
        .get_results(conn)
        .unwrap()
        .pop()
        .unwrap();

    let db_post = query(item, conn)?;
    Ok(db_post)
}
