pub(crate) mod db;
pub(crate) mod perfume_recommender;
pub(crate) mod providers;
pub(crate) mod producers;
pub(crate) mod auth;
pub(crate) mod contract;

mod tests {
    use crate::lib::producers::model::EvalCriteria;
use crate::lib::producers::model::EvalScale;
use crate::lib::producers::dsl::criteria_of;
use crate::lib::producers::dsl::scales_of;
use crate::lib::providers::model::ShipmentOption;
use super::db::{connect, process_query};
    use crate::diesel::{query_builder::SqlQuery ,RunQueryDsl};
    use super::providers::{model::{IfraIngredient, OtherIngredients, Provider}, dsl::{providers_evaluable_efficiency_by, ifra_ingredients_of, shipment_options_of}};

    #[test]
    pub fn test() {
        crate::dotenv::dotenv().ok();
        let connection = connect().get().expect("Connection unreachable");
        let results: Vec<Provider> = providers_evaluable_efficiency_by(1)
            .load(&connection)
            .expect("Query Failed");
        assert!(results.len() > 0);
    }

    #[test]
    pub fn test_ingredients_query() {
        crate::dotenv::dotenv().ok();
        let conn = connect().get().expect("Connection unreachable");
      
        let result: Vec<IfraIngredient>=
            ifra_ingredients_of(2)
            .load(&conn)
            .expect("Query Failed");
        assert!(result.len() > 0)
    }

    #[test]
    pub fn test_shipment_options() {
        crate::dotenv::dotenv().ok();
        let conn = connect().get().expect("Connection unreachable");

        let result: Vec<ShipmentOption>=
            shipment_options_of(2)
            .load(&conn)
            .expect("Query Failed");
        assert_eq!(result.len(), 3)
    }

    #[test]
    pub fn test_eval_scales() {
        crate::dotenv::dotenv().ok();
        let conn = connect().get().expect("Connection unreachable");

        let result: Vec<EvalScale>=
            scales_of(1)
            .load(&conn)
            .expect("Query Failed");
        assert_eq!(result.len(), 2)
    }

    #[test]
    pub fn test_eval_criteria() {
        crate::dotenv::dotenv().ok();
        let conn = connect().get().expect("Connection unreachable");

        let result: Vec<EvalCriteria>=
            criteria_of(1)
            .load(&conn)
            .expect("Query Failed");
        assert_eq!(result.len(), 3)
    }
}
