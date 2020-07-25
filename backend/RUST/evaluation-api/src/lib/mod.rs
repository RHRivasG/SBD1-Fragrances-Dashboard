pub(crate) mod db;
pub(crate) mod providers;
pub(crate) mod producers;
pub(crate) mod auth;

mod tests {
    use super::db::{connect, process_query};
    use crate::diesel::{query_builder::SqlQuery ,RunQueryDsl};
    use super::providers::{model::{IfraIngredient, OtherIngredients, Provider}, dsl::{providers_evaluable_efficiency_by, ifra_ingredients_of}};
    
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
            ifra_ingredients_of(1)
            .load(&conn)
            .expect("Query Failed");
        assert!(result.len() > 0)
    }
}
