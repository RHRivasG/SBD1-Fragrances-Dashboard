use rocket_contrib::json::Json;
use model::{EvalCriteria, Producer, EvalScale};
use dsl::{criteria_of, all, scales_of};
use super::db::Connection;
use diesel::prelude::*;
use rocket::http::Cookies;
pub mod model;
pub mod dsl;

#[get("/producers")]
pub fn get_producers(conn: Connection) -> Option<Json<Vec<Producer>>>
{
    all()
        .load(&*conn)
        .map(|r| Json(r))
        .ok()
}

#[get("/producers/criteria")]
pub fn get_criteria_of(mut cookies: Cookies, conn: Connection) -> Option<Json<Vec<EvalCriteria>>>
{
    cookies
        .get_private("producerId")
        .map(|v| v.value().parse().ok())
        .flatten()
        .and_then(|id|
                  criteria_of(id)
                  .load(&*conn)
                  .map(|r| Json(r))
                  .ok()
        )
}

#[get("/producers/scales")]
pub fn get_scales_of(mut cookies: Cookies, conn: Connection) -> Option<Json<Vec<EvalScale>>>
{
    cookies
        .get_private("producerId")
        .map(|v| v.value().parse().ok())
        .flatten()
        .and_then(|id|
                  scales_of(id)
                  .load(&*conn)
                  .map(|r| Json(r))
                  .ok()
    )
}
