use crate::schema::kmr_empresa_proveedora::columns::*;
use diesel::{query_builder::{SqlQuery, InsertStatement}, dsl::{Filter, Select, Eq, exists}, expression::exists::Exists, insert_into};
use crate::diesel::prelude::*;
use crate::schema::{self, kmr_empresa_proveedora, kmr_ifra_ingrediente};
use super::model::Contract;


type AllProviderColumns = (id, nombre, pag_web, inf_contacto, id_asoc_nacional);
const ALLPROVIDERCOLUMNS: AllProviderColumns =(id, nombre, pag_web, inf_contacto, id_asoc_nacional);

type AllPaymentConditionColumns = (schema::kmr_condiciones_pago::id, schema::kmr_condiciones_pago::id_emp_prov, schema::kmr_condiciones_pago::tipo, schema::kmr_condiciones_pago::cuotas);
const ALLPAYMENTCONDITIONCOLUMNS: AllPaymentConditionColumns = (schema::kmr_condiciones_pago::id, schema::kmr_condiciones_pago::id_emp_prov, schema::kmr_condiciones_pago::tipo, schema::kmr_condiciones_pago::cuotas);


pub fn all() -> Select<schema::kmr_empresa_proveedora::table, AllProviderColumns> {
    use crate::schema::kmr_empresa_proveedora::*;
    table.select(ALLPROVIDERCOLUMNS)
}

pub fn payment_conditions_of(id_provider: i32) -> Filter<Select<schema::kmr_condiciones_pago::table, AllPaymentConditionColumns>, Eq<schema::kmr_condiciones_pago::id_emp_prov, i32>>{
    use crate::schema::kmr_condiciones_pago::*;
    table.select(ALLPAYMENTCONDITIONCOLUMNS).filter(id_emp_prov.eq(id_provider))
}

type AllShipmentOptions = (schema::kmr_envio_pais::id_emp_prov, schema::kmr_envio_pais::id_pais, schema::kmr_envio_pais::dias_entrega, schema::kmr_envio_pais::tipo_transporte, schema::kmr_envio_pais::costo);
const ALLSHIPMENTOPTIONS: AllShipmentOptions =  (schema::kmr_envio_pais::id_emp_prov, schema::kmr_envio_pais::id_pais, schema::kmr_envio_pais::dias_entrega, schema::kmr_envio_pais::tipo_transporte, schema::kmr_envio_pais::costo);


pub fn shipment_options_of(id_provider: i32) -> Filter<Select<schema::kmr_envio_pais::table, AllShipmentOptions>, Eq<schema::kmr_envio_pais::id_emp_prov, i32>>{
    use crate::schema::kmr_envio_pais::*;
    table.select(ALLSHIPMENTOPTIONS).filter(id_emp_prov.eq(id_provider))
}

pub fn providers_evaluable_initial_by(provider_id: i32) -> SqlQuery {
    diesel::sql_query(format!(
        "SELECT DISTINCT v.* \
         FROM kmr_empresa_proveedora v \
         JOIN kmr_envio_pais ep ON ep.id_emp_prov = v.id \
         JOIN kmr_ep_p epp ON epp.id_pais = ep.id_pais \
         WHERE epp.id_emp_prod = {} AND NOT EXISTS \
         (SELECT * FROM kmr_contrato WHERE id_emp_prod = {} AND id_emp_prov = v.id);", provider_id, provider_id))
}

pub fn providers_evaluable_efficiency_by(provider_id: i32) -> SqlQuery {
    diesel::sql_query(format!(
        "SELECT DISTINCT v.* \
         FROM kmr_empresa_proveedora v \
         JOIN kmr_contrato c ON c.id_emp_prov = v.id
         WHERE c.id_emp_prod = {} AND \
         (NOW() - c.fecha_emision) > interval '11 months'", provider_id))
}

pub fn ifra_ingredients_of(provider_id: i32) -> Filter<kmr_ifra_ingrediente::table, Eq<kmr_ifra_ingrediente::id_emp_prov, i32>>
{
    use kmr_ifra_ingrediente::*;

    table.filter(id_emp_prov.eq(provider_id))
}

pub fn other_ingredients_of(provider_id: i32) -> Filter<schema::kmr_ingrediente_otros::table, Eq<schema::kmr_ingrediente_otros::id_emp_prov, i32>>
{
    use schema::kmr_ingrediente_otros::*;

    table.filter(id_emp_prov.eq(provider_id))
}

pub fn filter_by_id(id_producer: i32) -> Filter<kmr_empresa_proveedora::table, Eq<kmr_empresa_proveedora::id, i32>> {
    use kmr_empresa_proveedora::*;

    table.filter(id.eq(id_producer))
}

pub fn exists_by_id(id_producer: i32) -> Exists<Filter<kmr_empresa_proveedora::table, Eq<kmr_empresa_proveedora::id, i32>>> {
    use kmr_empresa_proveedora::*;

    exists(
        table.filter(id.eq(id_producer))
    )
}

pub fn insert_contract<C: Into<Contract>>(c: C) -> InsertStatement<schema::kmr_contrato::table, <Contract as Insertable<schema::kmr_contrato::table>>::Values> {
    insert_into(schema::kmr_contrato::table)
             .values(c.into())
}
