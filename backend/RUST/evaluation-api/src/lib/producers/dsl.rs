use crate::lib::producers::EvalCriteria;
use diesel::query_builder::{UpdateStatement, InsertStatement, AsChangeset, IntoUpdateTarget};
use crate::schema::kmr_empresa_productora::columns::*;
use diesel::dsl::*;
use diesel::{prelude::*, expression::{AsExpression, exists::Exists}, pg::Pg, associations::HasTable, sql_types::Date};
use crate::schema::*;
use chrono::{Utc, NaiveDate, NaiveDateTime};
use super::model::EvalScale;

pub fn all() -> Select<kmr_empresa_productora::table, (id, nombre, pag_web, inf_contacto, id_asoc_nacional)> {
    use kmr_empresa_productora::*;
    table.select((id, nombre, pag_web, inf_contacto, id_asoc_nacional))
}

pub fn find_by_id(producer_id: i32) -> Filter<kmr_empresa_productora::table, Eq<kmr_empresa_productora::id, i32>> {
    use kmr_empresa_productora::*;
    table.filter(id.eq(producer_id))
}

pub fn exists_by_id(producer_id: i32) -> Exists<Filter<kmr_empresa_productora::table, Eq<kmr_empresa_productora::id, i32>>> {
    exists(find_by_id(producer_id))
}

pub fn criteria_of(prod_id: i32) -> Filter<Select<kmr_criterio_eval::table, (kmr_criterio_eval::fechai, kmr_criterio_eval::fechaf, kmr_criterio_eval::tipoformula, kmr_criterio_eval::peso, kmr_criterio_eval::id_emp_prod, kmr_criterio_eval::id_criterio)>, Eq<kmr_criterio_eval::id_emp_prod, i32>> {
    use kmr_criterio_eval::*;
    use kmr_criterio_eval::columns::*;
    table.select((fechai, fechaf, tipoformula, peso, id_emp_prod, id_criterio)).filter(id_emp_prod.eq(prod_id))
}

pub fn scales_of(prod_id: i32) -> Filter<Select<kmr_escala_eval::table, (kmr_escala_eval::fechai, kmr_escala_eval::fechaf, kmr_escala_eval::rangoi, kmr_escala_eval::rangf, kmr_escala_eval::id_emp_prod)>, Eq<kmr_escala_eval::id_emp_prod, i32>> {
    use kmr_escala_eval::*;
    use kmr_escala_eval::columns::*;
    table.select((fechai, fechaf, rangoi, rangf, id_emp_prod)).filter(id_emp_prod.eq(prod_id))
}

pub fn find_scale_by(id_prod: i32,date: NaiveDateTime) -> Filter<kmr_escala_eval::table, And<Eq<kmr_escala_eval::fechai, NaiveDateTime>, Eq<kmr_escala_eval::id_emp_prod, i32>>> {
    kmr_escala_eval::table.filter(kmr_escala_eval::fechai.eq(date).and(kmr_escala_eval::id_emp_prod.eq(id_prod)))
}

pub fn scale_cancelled() -> Eq<kmr_escala_eval::fechaf, Option<NaiveDate>>
{
    let date_now = Utc::now().naive_utc().date();
    kmr_escala_eval::fechaf.eq(Some(date_now))
}

pub fn insert_scale(s: EvalScale) -> InsertStatement<kmr_escala_eval::table, <EvalScale as Insertable<kmr_escala_eval::table>>::Values> {
    insert_into(kmr_escala_eval::table)
        .values(s)
}


pub fn insert_criteria(c: EvalCriteria) -> InsertStatement<kmr_criterio_eval::table, <EvalCriteria as Insertable<kmr_criterio_eval::table>>::Values> {
    insert_into(kmr_criterio_eval::table)
        .values(c)
}

pub fn find_criteria_by(prod_id: i32, criteria_id: i32, fecha: NaiveDateTime) -> Filter<kmr_criterio_eval::table, And<And<Eq<kmr_criterio_eval::id_emp_prod, i32>, Eq<kmr_criterio_eval::id_criterio, i32>>, Eq<kmr_criterio_eval::fechai, NaiveDateTime>>> {
    kmr_criterio_eval::table
        .filter(kmr_criterio_eval::id_emp_prod.eq(prod_id).and(kmr_criterio_eval::id_criterio.eq(criteria_id)).and(kmr_criterio_eval::fechai.eq(fecha)))
}

pub fn criteria_cancelled() -> Eq<kmr_criterio_eval::fechaf, Option<NaiveDate>>
{
    let date_now = Utc::now().naive_utc().date();
    kmr_criterio_eval::fechaf.eq(Some(date_now))
}
