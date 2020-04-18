use diesel::r2d2::{PooledConnection, ConnectionManager};
use diesel::prelude::*;

mod models;
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
    post: models::Post, conn: &Conn
) -> Result<models::Post, diesel::result::Error> {
    use self::schema::posts::dsl::*;

    let new_post = models::InsertablePost{
        title: post.title.as_str(),
        category: post.category.as_str(),
        content: post.content.as_str()
    };

    let item: i32 = diesel::insert_into(posts)
        .values(&new_post)
        .returning(id)
        .get_results(conn)
        .unwrap()
        .pop()
        .unwrap();

    Ok(models::Post{
        id: item,
        title: post.title,
        category: post.category,
        content: post.content,
    })
}
