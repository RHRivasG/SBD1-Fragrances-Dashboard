use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use diesel::{pg::PgConnection, r2d2::{ConnectionManager, Pool}, RunQueryDsl, query_dsl::{methods::LimitDsl, LoadQuery}, query_builder::Query};
use rocket_contrib::json::Json;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn process_insert<T, K>(insert: T, conn: &Connection) -> Option<K>
where
    T: RunQueryDsl<K>,
    T: LoadQuery<PgConnection, K>
{
    insert
        .get_result(&conn)
        .map_err(|e| println!("Ocurrio un error {}", e.to_string()))
        .ok()
}

pub fn process_query_deser<T, K>(query: T, conn: &Connection) -> Option<Vec<K>>
where
    T: RunQueryDsl<K>,
    T: LoadQuery<PgConnection, K>
{
    query
        .load(&*conn)
        .ok()

}

pub fn process_query<T, K>(query: T, conn: &Connection) -> Option<Json<Vec<K>>>
where
    T: RunQueryDsl<K>,
    T: LoadQuery<PgConnection, K>,
{
    query
        .load(&*conn)
        .map(|v| Json(v))
        .ok()
}

pub fn connect() -> PgPool {
    let database_url = std::env::var(r#"DATABASE_URL"#).expect("Failed reading DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager).expect("Failed to create pool")
}

// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct Connection(pub diesel::r2d2::PooledConnection<ConnectionManager<PgConnection>>);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for Connection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Connection, ()> {
        let pool = request.guard::<State<PgPool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Connection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

// For the convenience of using an &Connection as an &SqliteConnection.
impl Deref for Connection {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
