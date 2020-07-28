use crate::lib::contract::model::ContractIngredient;
use crate::lib::contract::model::ParticularContract;
use diesel::query_builder::InsertStatement;
use crate::lib::contract::model::Contract;
use crate::diesel::sql_types::Timestamp;
use crate::diesel::prelude::*;
use crate::diesel::dsl::*;
use crate::schema::*;
use super::model::*;


pub fn insert_to_some_contract<CI, T>(c: CI, table: T, contract: i32) -> InsertStatement<T, < <CI as IntoSomeContract>::Contract as Insertable<T>>::Values>
where
    T: Table,
    CI: IntoSomeContract,
    CI::Contract: Insertable<T>,
<CI as super::model::IntoSomeContract>::Contract: diesel::query_builder::UndecoratedInsertRecord<T>
{
    insert_into(table)
        .values(c.with_contract(contract))
}

pub fn renew_with<T>(values: T) -> InsertStatement<kmr_renueva::table, <T as Insertable<kmr_renueva::table>>::Values>
where
    T: Insertable<kmr_renueva::table>
{
    use crate::schema::kmr_renueva::*;
    insert_into(table)
        .values(values)
}

pub fn inf_of_contract_between(id_provider: i32, id_producer: i32) -> Select<Filter<kmr_contrato::table, And<Eq<kmr_contrato::id_emp_prov, i32>, Eq<kmr_contrato::id_emp_prod, i32>>>, (kmr_contrato::id, kmr_contrato::id_emp_prov, diesel::dsl::date<<now as diesel::expression::AsExpression<Timestamp>>::Expression>)>
{
    use crate::schema::kmr_contrato::*;

    
    table
        .select((id, id_emp_prov, date(now)))
        .filter(
            id_emp_prov
                .eq(id_provider)
                .and(
                    id_emp_prod
                        .eq(id_producer)
                )
        )
}
