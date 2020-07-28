use rocket_contrib::json::Json;
use rocket::http::Cookies;
use self::model::ContractViewModel;
use super::db::Connection;

pub mod model;
pub mod dsl;

#[post("/contract/new", format="application/json", data="<contract>")]
pub fn new_contract(contract: Json<ContractViewModel>, cookies: Cookies, conn: Connection) -> Option<Json<i32>>{
    use dsl::*;
    use super::db::process_insert;
    
    let vm = contract.into_inner();
    let ingredients = vm.ingredients;
    let options = vm.options;
    let contract = vm.contract;
    super::auth::session::get_session_of("producerId", cookies)
        .and_then(|id|                  
                  process_insert::<_, i32>(insert_to_some_contract(contract, crate::schema::kmr_contrato::table , id)
                                           .returning(crate::schema::kmr_contrato::id)
                                           , &conn)
        )
        .and_then(|id_contract|
                  process_insert::<_, i32>(insert_to_some_contract(ingredients, crate::schema::kmr_ing_contrato::table, id_contract)
                                           .returning(crate::schema::kmr_ing_contrato::id_contrato)
                                           , &conn)
        )
        .and_then(|id_contract|    
                  process_insert::<_, i32>(insert_to_some_contract(options, crate::schema::kmr_contrato_particulares::table, id_contract)
                                           .returning(crate::schema::kmr_contrato_particulares::id_contrato)
                                           , &conn)
        )
        .map(|v| Json(v))
}

#[post("/contract/renew", format="application/json", data="<contract>")]
pub fn renew_contract(contract: Json<i32>, cookies: Cookies, conn: Connection) -> Option<crate::rocket_contrib::json::JsonValue>{
    use dsl::*;
    use super::db::process_insert;
    use crate::schema::kmr_renueva;

    
    let vm = contract.into_inner();
    super::auth::session::get_session_of("producerId", cookies)
        .and_then(|id|
                  process_insert::<_, (i32, crate::chrono::NaiveDate)>(
                      renew_with(
                          inf_of_contract_between(vm, id)
                      )
                          .into_columns((kmr_renueva::id_contrato, kmr_renueva::id_emp_prov, kmr_renueva::fecha))
                          .returning((kmr_renueva::id, kmr_renueva::fecha))
                     , &conn)
        )
        .map(|v| json!({
            "id": v.0,
            "fecha": v.1
        }))
}
