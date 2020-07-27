use model::{PaymentCondition, Provider, ShipmentOption, IfraIngredient, OtherIngredients};
use rocket_contrib::json::Json;
use super::db::Connection;
use dsl::{payment_conditions_of, all, shipment_options_of, providers_evaluable_efficiency_by, providers_evaluable_initial_by, ifra_ingredients_of, other_ingredients_of};
use rocket::http::Cookies;
use super::auth::session;
use super::db::{process_query_deser, process_query};
use rocket_contrib::json::JsonValue;
pub mod model;
pub mod dsl;

fn all_ingredients_of(conn: &Connection, id: i32) -> Option<JsonValue>
{
    process_query_deser::<_,IfraIngredient>(ifra_ingredients_of(id), &conn)
    .and_then(
      |ifing|
      {
        process_query_deser::<_,OtherIngredients>(other_ingredients_of(id), &conn)
          .map(|othering|
               json!({
                   "ifra": ifing,
                   "others": othering
                 })
          )   
      }
    )
}

#[get("/providers")]
pub fn get_providers(conn: Connection) -> Option<Json<Vec<Provider>>>{
  process_query(all(), &conn)
}

#[get("/providers/payment_conditions")]
pub fn get_session_payment_conditions(cookies: Cookies, conn: Connection) -> Option<Json<Vec<PaymentCondition>>> {
  session::get_session_of("providerId", cookies)
    .and_then(|id|process_query(payment_conditions_of(id), &conn))
}

#[get("/providers/shipment_options")]
pub fn get_session_shipment_options(cookies: Cookies, conn: Connection) -> Option<Json<Vec<ShipmentOption>>> {
  session::get_session_of("providerId", cookies)
    .and_then(|id| process_query(shipment_options_of(id), &conn))
}

#[get("/providers/payment_conditions/<id>", rank= 2)]
pub fn get_payment_conditions_of(id: i32, conn: Connection) -> Option<Json<Vec<PaymentCondition>>> {
  process_query(payment_conditions_of(id), &conn)
}

#[get("/providers/shipment_options/<id>", rank= 2)]
pub fn get_shipment_options_of(id: i32, conn: Connection) -> Option<Json<Vec<ShipmentOption>>> {
  process_query(shipment_options_of(id), &conn)
}

#[get("/providers/evaluable_initial_by", rank = 3)]
pub fn get_evaluable_initial_providers(cookies: Cookies, conn: Connection) -> Option<Json<Vec<Provider>>> {
  session::get_session_of("producerId", cookies)
    .and_then(|id| process_query(providers_evaluable_initial_by(id), &conn))
}

#[get("/providers/evaluable_efficiency_by", rank = 3)]
pub fn get_evaluable_efficiency_providers(cookies: Cookies, conn: Connection) -> Option<Json<Vec<Provider>>> {
  session::get_session_of("producerId", cookies)
    .and_then(|id| process_query(providers_evaluable_efficiency_by(id), &conn))
}

#[get("/providers/<id>/ingredients", rank=3)]
pub fn get_ingredients_of(id: i32, conn: Connection) -> Option<JsonValue> {
  all_ingredients_of(&conn, id)
}

#[get("/providers/ingredients")]
pub fn get_session_ingredients(cookies: Cookies, conn: Connection) -> Option<JsonValue> { 
  session::get_session_of("providerId", cookies)
    .and_then(|id_provider|   all_ingredients_of(&conn, id_provider))
}
