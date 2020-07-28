use chrono::naive::serde::ts_seconds;
use crate::{lib::contract::model::IntoSomeContract, schema::{kmr_empresa_productora,  kmr_criterio_eval, kmr_escala_eval}};
use chrono::{NaiveDateTime, NaiveDate};

#[derive(Queryable, Insertable, AsChangeset, Identifiable, Serialize, Deserialize)]
#[table_name="kmr_empresa_productora"]
pub struct Producer {
    id: i32,
    nombre: String,
    pag_web: String,
    inf_contacto: String,
    id_asoc_nacional: Option<i32>
}

#[derive(Queryable, Insertable, Serialize, Deserialize, AsChangeset)]
#[table_name = "kmr_escala_eval"]
pub struct EvalScale {
    #[serde(with="ts_seconds")]
    pub fechai: NaiveDateTime,
    fechaf: Option<NaiveDate>,
    rangoi: i32,
    rangf: i32,
    pub id_emp_prod: i32
}

#[derive(Queryable, Insertable, Serialize, Deserialize, AsChangeset)]
#[table_name = "kmr_criterio_eval"]
pub struct EvalCriteria {
    #[serde(with="ts_seconds")]
    pub fechai: NaiveDateTime,
    fechaf: Option<NaiveDate>,
    tipoformula: String,
    peso: i32,
    pub id_emp_prod: i32,
    pub id_criterio: i32
}


#[derive(Serialize, Deserialize)]
pub struct EvalCriteriaVM {
    #[serde(with="ts_seconds")]
    fechai: NaiveDateTime,
    fechaf: Option<NaiveDate>,
    tipoformula: String,
    peso: i32,
    id_criterio: i32    
}

#[derive(Serialize, Deserialize)]
pub struct EvalScaleVM {
    #[serde(with="ts_seconds")]
    fechai: NaiveDateTime,
    fechaf: Option<NaiveDate>,
    rangoi: i32,
    rangf: i32
}

impl IntoSomeContract for EvalCriteriaVM {
    type Contract = EvalCriteria;
    fn with_contract(self, id_contract: i32) -> Self::Contract {
        EvalCriteria {
            fechai: self.fechai,
            fechaf: self.fechaf,
            tipoformula: self.tipoformula,
            peso: self.peso,
            id_criterio: self.id_criterio,
            id_emp_prod: id_contract
        }
    }
}

impl IntoSomeContract for EvalScaleVM {
    type Contract = EvalScale;
    fn with_contract(self, id_contract: i32) -> Self::Contract {
        EvalScale {
            fechai: self.fechai,
            fechaf: self.fechaf,
            rangoi: self.rangoi,
            rangf: self.rangf,
            id_emp_prod: id_contract
        }
    }
}
