#![feature(proc_macro_hygiene, decl_macro)]
use rocket::Rocket;
use rocket::fairing::AdHoc;
pub(crate) mod schema;
pub(crate) mod lib;

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde;
#[macro_use] extern crate rocket_contrib;
extern crate bigdecimal;
extern crate dotenv;
extern crate chrono;

use dotenv::dotenv;
use rocket::custom;
use rocket::config::*;

use crate::lib::db::connect;

fn config() -> rocket::config::Result<Config> {
        dotenv().ok();
        let port = std::env::var("PORT").expect("PORT not found").parse().expect("PORT is not valid port");
        let config = Config::build(Environment::Production).port(port).log_level(LoggingLevel::Normal);
        
        std::env::var("SECRET_KEY")
                .map(|secret_key|
                     config
                     .secret_key(secret_key)
                     .finalize())
                .ok()
                .unwrap_or_else(||
                                Config::build(Environment::Production)
                                .port(port)
                                .log_level(LoggingLevel::Normal)
                                .finalize())
        
}

fn main() {
        custom(config().unwrap())
                .manage(connect())
                .mount("/api", routes![
                        lib::providers::get_providers,
                        lib::providers::get_session_shipment_options,
                        lib::providers::get_session_payment_conditions,
                        lib::providers::get_payment_conditions_of,
                        lib::providers::get_shipment_options_of,
                        lib::providers::get_ingredients_of,
                        lib::providers::get_session_ingredients,
                        lib::providers::get_evaluable_initial_providers,
                        lib::providers::get_evaluable_efficiency_providers,
                        lib::producers::new_evaluation_scale,
                        lib::producers::cancel_scale,
                        lib::producers::update_scale,
                        lib::producers::update_criteria,
                        lib::producers::new_criteria,
                        lib::producers::cancel_criteria,
                        lib::producers::get_producers,
                        lib::producers::get_criteria_of,
                        lib::producers::get_scales_of,
                        lib::auth::login,
                        lib::auth::get_logged_provider_user,
                        lib::auth::get_logged_producer_user,
                        lib::contract::new_contract,
                        lib::contract::renew_contract,
                        lib::perfume_recommender::get_recommender_source
                ])
                .launch();
}
