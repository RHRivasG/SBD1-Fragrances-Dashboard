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
    id: i32,
    id_emp_prov: i32,
    tipo: String,
    cuotas: Option<i32>
}

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name="kmr_envio_pais"]
pub struct ShipmentOption {
    id_emp_prov: i32,
    id_pais: i32,
    dias_entrega: i32,
    tipo_transporte: String,
    costo: Option<f32>
}

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize, FromForm, QueryableByName)]
#[table_name="kmr_ifra_ingrediente"]
pub struct IfraIngredient {
    cas_number: i32,
    id_emp_prov: i32,
    descripcion_visual: String,
    proceso: String,
    vida_util: String,
    solubilidad: Option<String>,
    procesodescripcion: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DateStruct(NaiveDate);

impl AsRef<NaiveDate> for DateStruct {
    fn as_ref(&self) -> &NaiveDate {
        &*self
    }
}

impl Deref for DateStruct {
    type Target= NaiveDate;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<NaiveDate> for DateStruct {

    fn into(self) -> NaiveDate {
        self.0
    }
    
} 

#[derive(Insertable, Queryable, Identifiable, PartialEq)]
#[table_name="kmr_contrato"]
pub struct Contract {
    pub id: Option<i32>,
    pub id_emp_prod: i32,
    pub id_emp_prov: i32,
    pub exclusividad: bool,
    pub fecha_emision: NaiveDate,
    pub fecha_cancelado: Option<NaiveDate>,
    pub motivo_cancelo: Option<String>
}

impl Contract {
    pub fn new(id: Option<i32>, id_emp_prod: i32, id_emp_prov: i32, exclusividad: bool, fecha_emision: NaiveDate, fecha_cancelado: Option<NaiveDate>, motivo_cancelo: Option<String>) -> Self {
        Contract {
            id,
            id_emp_prod,
            id_emp_prov,
            exclusividad,
            fecha_emision,
            fecha_cancelado,
            motivo_cancelo
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ContractViewModel {
    pub id_emp_prod: i32,
    pub id_emp_prov: i32,
    pub exclusividad: Option<bool>,
    pub fecha_emision: DateStruct,
    pub fecha_cancelado: Option<DateStruct>,
    pub motivo_cancelado: Option<String>,
}

impl Into<Contract> for ContractViewModel {

    fn into(self) -> Contract {
        Contract::new(None, self.id_emp_prod, self.id_emp_prov, self.exclusividad.unwrap_or(false), self.fecha_emision.into(), self.fecha_cancelado.map(|ds| ds.into()), self.motivo_cancelado)
    }
    
}

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize, FromForm, QueryableByName)]
#[table_name = "kmr_ingrediente_otros"]
pub struct OtherIngredients {
    ipc: i32,
    nombre: String,
    tipo: String,
    tsca_cas: Option<i32>,
    id_emp_prov: Option<i32>
}
