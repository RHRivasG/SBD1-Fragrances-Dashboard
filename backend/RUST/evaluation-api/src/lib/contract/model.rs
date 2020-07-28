use crate::lib::providers::model::PaymentCondition;
use crate::lib::providers::model::ShipmentOption;
use core::ops::Deref;
use crate::lib::providers::model::{OtherIngredients, IfraIngredient};
use chrono::NaiveDate;
use crate::schema::*;

pub trait IntoSomeContract {
    type Contract;
    fn with_contract(self, id_contract: i32) -> Self::Contract; 
}

impl<I, T> IntoSomeContract for I
where
    T: IntoSomeContract,
    I: IntoIterator<Item = T>
{
    type Contract = Vec<T::Contract>;

    fn with_contract(self, id_contract: i32) -> Self::Contract {
        self
            .into_iter()
            .map(to_contract(id_contract))
            .collect()
    }
}

fn to_contract<V: IntoSomeContract>(id_contract: i32) -> (impl Fn(V) -> V::Contract) {
    move |v| v.with_contract(id_contract)
}

#[derive(Insertable, Queryable, Identifiable, PartialEq, Serialize, Deserialize)]
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

#[derive(Insertable, Queryable, Identifiable, QueryableByName, Serialize, Deserialize)]
#[table_name = "kmr_contrato_particulares"]
pub struct ParticularContract {
    id: Option<i32>,
    id_contrato: i32,
    id_emp_prov: i32,
    id_cond_pago: Option<i32>,
    id_cond_pago_prov: Option<i32>,
    id_envio_pais: Option<i16>,
    id_envio_pais_prov: Option<i32>,
    descripcion: Option<String>
}


#[derive(Insertable, Queryable, Identifiable, PartialEq, Serialize, Deserialize)]
#[table_name= "kmr_ing_contrato"]
pub struct ContractIngredient {
    id: Option<i32>,
    id_contrato: i32,
    id_emp_prov: i32,
    id_ing_otros: Option<i32>,
    id_ing_ifra: Option<i32>
}

impl IntoSomeContract for IfraIngredient {
    type Contract = ContractIngredient;
    
    fn with_contract(self, contract: i32) -> Self::Contract {
        ContractIngredient {
            id: None,
            id_contrato: contract,
            id_emp_prov: self.id_emp_prov,
            id_ing_ifra: self.cas_number.into(),
            id_ing_otros: None
        }
    }
}

impl IntoSomeContract for OtherIngredients {
    type Contract = Option<ContractIngredient>;
    
    fn with_contract(self, contract: i32) -> Self::Contract {
        self.id_emp_prov
            .map(
                |id_emp_prov|
                ContractIngredient {
                    id: None,
                    id_contrato: contract,
                    id_emp_prov: id_emp_prov,
                    id_ing_ifra: None,
                    id_ing_otros: self.ipc.into()
                }
            )
    }
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

#[derive(Insertable, Queryable, Identifiable, PartialEq, Serialize, Deserialize)]
#[table_name= "kmr_renueva"]
pub struct ContractRenewal {
    id: Option<i32>,
    id_contrato: i32,
    id_emp_prov: i32,
    fecha: NaiveDate
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

#[derive(Serialize, Deserialize)]
pub struct ContractIngredientVM
{
    pub ifra: Option<Vec<IfraIngredient>>,
    pub other: Option<Vec<OtherIngredients>>
}

impl IntoSomeContract for ContractIngredientVM {
    type Contract = Vec<ContractIngredient>; 

    fn with_contract(self, id_contract: i32) -> <Self as IntoSomeContract>::Contract {
        self.ifra
            .into_iter()
            .flatten()
            .map(to_contract(id_contract))
            .chain(self.other.into_iter().flatten().filter_map(to_contract(id_contract)))
            .collect()            
    }
}

#[derive(Serialize, Deserialize)]
pub struct ParticularContractVM
{
    pub id: Option<i32>,
    pub shipment: Option<ShipmentOption>,
    pub payment: Option<PaymentCondition>,
    pub descripcion: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct ParticularContractCollection(Vec<ParticularContractVM>);

impl Into<Vec<ParticularContractVM>> for ParticularContractCollection {
    fn into(self) -> Vec<ParticularContractVM> {
        self.0
    }
}

impl IntoSomeContract for ParticularContractCollection {
    type Contract = Vec<ParticularContract>;
    
    fn with_contract(self, id: i32) -> <Self as IntoSomeContract>::Contract {
        self.0
            .into_iter()
            .filter_map(to_contract(id))
            .collect()
    }
}

impl IntoSomeContract for ShipmentOption {
    type Contract = ParticularContract;
    
    fn with_contract(self, contract_id: i32) -> Self::Contract {
        ParticularContract {
            id: None,
            id_contrato: contract_id,
            id_emp_prov: self.id_emp_prov,
            id_cond_pago: None,
            id_cond_pago_prov: None,
            id_envio_pais: self.id_pais.into(),
            id_envio_pais_prov: self.id_emp_prov.into(),
            descripcion: None
        }
    }
}

impl IntoSomeContract for PaymentCondition {
    type Contract = ParticularContract;
    
    fn with_contract(self, contract_id: i32) -> Self::Contract {
        ParticularContract {
            id: None,
            id_contrato: contract_id,
            id_emp_prov: self.id_emp_prov,
            id_cond_pago: self.id.into(),
            id_cond_pago_prov: self.id_emp_prov.into(),
            id_envio_pais: None,
            id_envio_pais_prov: None,
            descripcion: None
        }
    }
}

impl IntoSomeContract for ParticularContractVM {
    type Contract = Option<ParticularContract>;

    fn with_contract(self, id_contract: i32) -> Self::Contract {
        let description = self.descripcion;
        let id = self.id;
        let shipment = self.shipment;
        
        self.payment
            .map(to_contract(id_contract))
            .xor(shipment.map(to_contract(id_contract)))
            .map(move |mut v| {
                v.descripcion = description;
                v.id = id;
                v
            })
    }
}

#[derive(Serialize, Deserialize)]
pub struct ContractViewModel {
    pub contract: IncompleteContract,
    pub options: ParticularContractCollection,
    pub ingredients: ContractIngredientVM
}

#[derive(Serialize, Deserialize)]
pub struct IncompleteContract {
    pub id_emp_prov: i32,
    pub exclusividad: bool,
    pub fecha_emision: NaiveDate,
    pub fecha_cancelado: Option<NaiveDate>,
    pub motivo_cancelo: Option<String>
}

impl IntoSomeContract for IncompleteContract {
    type Contract = Contract;

    fn with_contract(self, id_producer: i32) -> Self::Contract {
        Contract {
            id: None,
            id_emp_prod: id_producer,
            id_emp_prov: self.id_emp_prov,
            exclusividad: self.exclusividad,
            fecha_cancelado: self.fecha_cancelado,
            fecha_emision: self.fecha_emision,
            motivo_cancelo: self.motivo_cancelo
        }
    }
}
