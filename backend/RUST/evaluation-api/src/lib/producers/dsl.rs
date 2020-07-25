use crate::schema::kmr_empresa_productora::columns::*;
use diesel::dsl::*;
use diesel::{prelude::*, expression::exists::Exists};
use crate::schema::*;

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
