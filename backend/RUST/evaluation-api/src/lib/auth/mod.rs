pub(crate) mod session;

use rocket_contrib::json::Json;
use rocket::{response::status, http::{Cookie, Cookies}};
use super::{providers::model::Provider, db::Connection, producers::model::Producer};
use diesel::select;
use diesel::{query_dsl::{LoadQuery, methods::LimitDsl}, prelude::*};

fn unload_expression<T, K>(exp: T, conn: &Connection) -> Option<K>
where
    T: LimitDsl,
    T::Output: LoadQuery<PgConnection, K>,
    T::Output: RunQueryDsl<K>
{
    exp
        .limit(1)
        .load(&*conn)
        .ok()
        .map(|v| v.into_iter().next())
        .flatten()
} 

#[post("/login?<login_type>", format="application/json", data="<user_id>")]
pub fn login(user_id: Json<i32>, login_type: String, mut cookies: Cookies, conn: Connection) -> Result<status::Accepted<String>, status::BadRequest<String>> {
    use super::{providers, producers};
    let id  = user_id.into_inner();
    select(
        providers::dsl::exists_by_id(id).or(producers::dsl::exists_by_id(id))
    )
        .get_result(&*conn)
        .map(|result| {
            match &*login_type {
                "provider" if result => {cookies.add_private(Cookie::new("providerId", id.to_string())); Ok(status::Accepted(Some("Iniciado sesion como proveedor".into())))}
                "producer" if result => {cookies.add_private(Cookie::new("producerId", id.to_string())); Ok(status::Accepted(Some("Iniciado sesion como productor".into())))}
                _ if !result => Err(status::BadRequest(Some("Error: No se ha encontrado la empresa con la que ha intentado iniciar sesion".to_string()))),
                _ => Err(status::BadRequest(Some("Error: Tipo de login no permitido".into())))
            }
        })
        .map_err(|e| Err(status::BadRequest((e.to_string()).into())))
        .unwrap_or_else(|e| e)
}

fn session<T, Q, F>(cookie: String, f: F) -> impl FnOnce(Cookies, Connection) -> Result<Json<Option<T>>, status::NotFound<String>>
where
    Q: LimitDsl,
    F: FnOnce(i32) -> Q + 'static,
    Q::Output: LoadQuery<PgConnection, T>,
    Q::Output: RunQueryDsl<T>
{
//    Box::new(
    move |mut cookies, conn| {
        cookies
            .get_private(&cookie)
            .map(|id|
                 id
                 .value()
                 .parse::<i32>()
                 .map(|id_enterprise| Json(unload_expression(f(id_enterprise), &conn)))
                 .map_err(|e| status::NotFound(e.to_string()))
            )
            .unwrap_or(Err(status::NotFound("Id not found".into())))
    }
//    )
}


#[get("/login/logged?auth_type=provider", rank=2)] 
pub fn get_logged_provider_user(cookies: Cookies, conn: Connection) -> Result<Json<Option<Provider>>, status::NotFound<String>> {
    session("providerId".into(), |id_provider| super::providers::dsl::filter_by_id(id_provider))(cookies, conn)
}

#[get("/login/logged?auth_type=producer")] 
pub fn get_logged_producer_user(cookies: Cookies, conn: Connection) -> Result<Json<Option<Producer>>, status::NotFound<String>> {
    session("producerId".into(), |id_producer| super::producers::dsl::find_by_id(id_producer))(cookies, conn)
}
