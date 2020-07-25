use crate::schema::{kmr_empresa_productora,  kmr_criterio_eval, kmr_escala_eval};
use chrono::NaiveDate;

#[derive(Queryable, Insertable, AsChangeset, Identifiable, Serialize, Deserialize)]
#[table_name="kmr_empresa_productora"]
pub struct Producer {
    id: i32,
    nombre: String,
    pag_web: String,
    inf_contacto: String,
    id_asoc_nacional: Option<i32>
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "kmr_escala_eval"]
pub struct EvalScale {
    fechai: NaiveDate,
    fechaf: Option<NaiveDate>,
    rangoi: i32,
    rangf: i32,
    id_emp_prod: i32
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "kmr_criterio_eval"]
pub struct EvalCriteria {
    fechai: NaiveDate,
    fechaf: Option<NaiveDate>,
    tipoformula: String,
    peso: i32,
    id_emp_prod: i32,
    id_criterio: i32
}
