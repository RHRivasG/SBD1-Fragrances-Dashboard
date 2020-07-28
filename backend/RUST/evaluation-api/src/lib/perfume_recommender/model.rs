use crate::schema::*;
use crate::bigdecimal::BigDecimal;
use bigdecimal::ToPrimitive;

#[derive(Insertable, Queryable, Identifiable, PartialEq, Serialize, Deserialize)]
#[table_name="kmr_perfume"]
pub struct Perfume {
    id: i32,
    nombre: String,
    tipo: String,
    genero: String,
    edad: String,
    id_emp_prod: i32
}

#[derive(Insertable, Queryable, Identifiable, PartialEq, Serialize, Deserialize, QueryableByName)]
#[table_name="kmr_palabra_clave"]
pub struct Keyword {
    id: i32,
    palabra_unica: String
}

#[derive(Insertable, Queryable, Identifiable, PartialEq)]
#[table_name="kmr_intensidad"]
pub struct Intensity {
    id: i32,
    id_perfume: i32,
    tipo: String,
    concentracion: Option<BigDecimal>,
    descripcion: Option<String>
}

#[derive(Insertable, Queryable, Identifiable, PartialEq, Serialize, Deserialize, QueryableByName)]
#[table_name="kmr_familia_olf"]
pub struct OlfFamily {
    id: i32,
    nombre: String,
    descripcion: String
}


#[derive(Serialize, Deserialize)]
pub struct PerfumeVM {
    pub id: i32,
    pub nombre: String,
    pub tipo: String,
    pub genero: String,
    pub edad: String,    
    pub id_emp_prod: i32,
    pub keywords: Option<Vec<Keyword>>,
    pub intensities: Option<Vec<IntensityVM>>,
    pub families: Option<Vec<OlfFamily>>
}

impl From<Perfume> for PerfumeVM {
    
    fn from(any: Perfume) -> Self {
        PerfumeVM {
            id: any.id,
            nombre: any.nombre,
            id_emp_prod: any.id_emp_prod,
            tipo: any.tipo,
            genero: any.genero,
            edad: any.edad,
            keywords: None,
            intensities: None,
            families: None
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct IntensityVM {
    id: i32,
    id_perfume: i32,
    tipo: String,
    concentracion: Option<f64>,
    descripcion: Option<String>
}

impl From<Intensity> for IntensityVM {
    
    fn from(it: Intensity) -> Self {
        let concentracion = it.concentracion;
        let finaldec = concentracion.map(|f| f.to_f64()).flatten();
        IntensityVM {
            id: it.id,
            id_perfume: it.id_perfume,
            tipo: it.tipo,
            concentracion: finaldec,
            descripcion: it.descripcion
        }
    }
    
}
