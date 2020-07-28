pub mod model;
pub mod dsl;

use chrono::{NaiveDateTime, NaiveDate};
use crate::lib::db::process_insert;
use crate::lib::db::process_query;
use super::auth as auth;
use rocket_contrib::json::Json;
use model::{EvalCriteria, Producer, EvalScale, EvalScaleVM, EvalCriteriaVM};
use dsl::{criteria_of, all, scales_of, find_scale_by, find_criteria_by};
use super::{contract::model::{IntoSomeContract, DateStruct}, db::Connection};
use diesel::{update, prelude::*, dsl::{now, date}};
use rocket::http::Cookies;
use rocket::response::status;
use std::str::FromStr;

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

#[post("/producers/criteria/new", format="application/json", data="<criteria>")]
pub fn new_criteria(criteria: Json<EvalCriteriaVM>, cookies: Cookies, conn: Connection)-> Result<status::Accepted<String>, String> {
    use dsl::*;
    use crate::schema::kmr_criterio_eval;
    
    auth::session::get_session_of("producerId", cookies)
        .and_then(|id|
                  process_insert::<_, NaiveDateTime>(insert_criteria(criteria.into_inner().with_contract(id))
                                                 .returning(kmr_criterio_eval::fechai)
                                                 , &conn)
        )
        .map(|r| status::Accepted(Some(format!("El criterio fue creado con exito con fecha: {}", r.to_string()))))
        .ok_or("Error al procesar la insercion del criterio".into())
}

#[patch("/producers/criteria/cancel?<criteria_id>&<date_epoch>")]
pub fn cancel_criteria(criteria_id: i32, date_epoch: i64, cookies: Cookies, conn: Connection) -> Result<status::Accepted<String> , String> {
    use dsl::*;
    use crate::schema::kmr_criterio_eval;

    let date = NaiveDateTime::from_timestamp(date_epoch, 0);

    auth::session::get_session_of("producerId", cookies)
        .and_then(|prod_id|
                  process_insert(
                      update(find_criteria_by(prod_id, criteria_id, date))
                          .set(criteria_cancelled())
                          .returning(kmr_criterio_eval::fechaf), &conn)
                  .flatten()
                  
        )
        .map(|f: NaiveDate| status::Accepted(Some(format!("Criterio cancelado: {}", f.to_string()))))
        .ok_or("Fallo al cancelar el criterio".into())
}

#[put("/producers/criteria/update", format="application/json", data="<criteria>")]
pub fn update_criteria(criteria: Json<EvalCriteria>, cookies: Cookies, conn: Connection) -> Result<status::Accepted<String>, String>
{
    use crate::schema::kmr_criterio_eval;

    let criteria = criteria.into_inner();

    auth::session::get_session_of("producerId", cookies)
        .and(
            process_insert(
                update(find_criteria_by(criteria.id_emp_prod, criteria.id_criterio, criteria.fechai))
                    .set(criteria)
                    .returning(kmr_criterio_eval::fechai),
                &conn)
        )
        .map(|f: NaiveDateTime| status::Accepted(Some("Actualizado Criterio".into())))
        .ok_or("Actualizacion fallida".into())
}

#[post("/producers/scales/new", format="application/json", data="<scale>")]
pub fn new_evaluation_scale(scale: Json<EvalScaleVM>, cookies: Cookies, conn: Connection) -> Result<status::Accepted<String>, String>{
    use dsl::*;
    use crate::schema::kmr_escala_eval;

    let scale = scale.into_inner();

    auth::session::get_session_of("producerId", cookies)
        .and_then(|id|                  
                  process_insert(
                      insert_scale(scale.with_contract(id))
                          .returning(kmr_escala_eval::fechai),
                      &conn
                  )
        )
        .map(|f: NaiveDateTime| status::Accepted(Some(format!("Creada nueva escala de evaluacion {}", f.to_string()))))
        .ok_or("No se pudo crear la escala de evaluacion".into())
}

#[patch("/producers/scale/cancel?<eval_id>")]
pub fn cancel_scale(eval_id: i64, cookies: Cookies, conn: Connection) -> Result<status::Accepted<String> , String> {
    use dsl::*;
    use crate::schema::kmr_escala_eval;

    Ok::<_, String>(NaiveDateTime::from_timestamp(eval_id, 0))
        .ok()
        .and_then(|eval_id| 
                  auth::session::get_session_of("producerId", cookies)
                  .and_then(|prod_id|
                            process_insert(
                                update(find_scale_by(prod_id, eval_id))
                                    .set(scale_cancelled())
                                    .returning(kmr_escala_eval::fechaf), &conn)
                            .flatten()
                            
                  )
                  .map(|f: NaiveDate| status::Accepted(Some(format!("Criterio cancelado: {}", f.to_string()))))
        )
        .ok_or("Error fecha invalida".into())
}

#[put("/producers/scale/update", format="application/json", data="<scale>")]
pub fn update_scale(scale: Json<EvalScale>, cookies: Cookies, conn: Connection) -> Result<status::Accepted<String>, String>
{
    use crate::schema::kmr_escala_eval;

    let scale = scale.into_inner();

    auth::session::get_session_of("producerId", cookies)
        .and(
            process_insert(
                update(find_scale_by(scale.id_emp_prod, scale.fechai))
                    .set(scale)
                    .returning(kmr_escala_eval::fechai),
                &conn)
        )
        .map(|f: NaiveDateTime| status::Accepted(Some(format!("Actualizado Escala: {}", f.to_string()))))
        .ok_or("Actualizacion fallida".into())
}
