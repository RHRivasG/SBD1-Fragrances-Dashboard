use crate::schema::{kmr_empresa_proveedora, kmr_condiciones_pago, kmr_envio_pais, kmr_ifra_ingrediente, kmr_contrato, kmr_ingrediente_otros};
use chrono::NaiveDate;
use std::ops::Deref;

#[derive(Queryable, Insertable, AsChangeset, Identifiable, Serialize, Deserialize, QueryableByName)]
#[table_name="kmr_empresa_proveedora"]
pub struct Provider {
    id: i32,
    nombre: String,
    pag_web: String,
    inf_contacto: String,
    id_asoc_nacional: Option<i32>
}

#[derive(Queryable, Insertable, AsChangeset, Identifiable, Serialize, Deserialize)]
#[table_name="kmr_condiciones_pago"]
pub struct PaymentCondition {
    pub id: i32,
    pub id_emp_prov: i32,
    pub tipo: String,
    pub cuotas: Option<i32>
}

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name="kmr_envio_pais"]
pub struct ShipmentOption {
    pub id_emp_prov: i32,
    pub id_pais: i32,
    pub dias_entrega: i32,
    pub tipo_transporte: String,
    pub costo: Option<f32>
}

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize, FromForm, QueryableByName)]
#[table_name="kmr_ifra_ingrediente"]
pub struct IfraIngredient {
    pub cas_number: i32,
    pub id_emp_prov: i32,
    descripcion_visual: String,
    proceso: String,
    vida_util: String,
    solubilidad: Option<String>,
    procesodescripcion: Option<String>
}

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize, FromForm, QueryableByName)]
#[table_name = "kmr_ingrediente_otros"]
pub struct OtherIngredients {
    pub ipc: i32,
    nombre: String,
    tipo: String,
    tsca_cas: Option<i32>,
    pub id_emp_prov: Option<i32>
}
