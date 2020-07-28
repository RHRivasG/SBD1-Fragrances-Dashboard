pub mod model;
pub mod dsl;

use super::auth as auth;
use rocket_contrib::json::Json;
use model::{EvalCriteria, Producer, EvalScale};
use dsl::{criteria_of, all, scales_of};
use super::db::Connection;
use diesel::prelude::*;
use rocket::http::Cookies;

#[get("/producers")]
pub fn get_producers(conn: Connection) -> Option<Json<Vec<Producer>>>
{
    all()
        .load(&*conn)
        .map(|r| Json(r))
        .ok()
}

#[get("/producers/criteria")]
pub fn get_criteria_of(cookies: Cookies, conn: Connection) -> Option<Json<Vec<EvalCriteria>>>
{
    auth::session::get_session_of("producerId", cookies)
        .and_then(|id|
                  criteria_of(id)
                  .load(&*conn)
                  .map(|r| Json(r))
                  .ok()
        )
}

#[get("/producers/scales")]
pub fn get_scales_of(cookies: Cookies, conn: Connection) -> Option<Json<Vec<EvalScale>>>
{
    auth::session::get_session_of("producerId", cookies)
        .and_then(|id|
                  scales_of(id)
                  .load(&*conn)
                  .map(|r| Json(r))
                  .ok()
    )
}
