pub mod model;
pub mod dsl;
use crate::lib::perfume_recommender::model::Perfume;
use crate::lib::perfume_recommender::model::IntensityVM;
use crate::lib::perfume_recommender::model::PerfumeVM;
use rocket_contrib::json::Json;
use rocket::response::status;
use crate::lib::db::Connection;

#[get("/recommender/source")]
pub fn get_recommender_source(conn: Connection) -> Result<Json<Vec<PerfumeVM>>, status::BadRequest<String>>
{
    use dsl::*;

    all(&conn)
        .and_then(|result: Vec<Perfume>|
                  {
                      Ok(result
                         .into_iter()
                         .map(PerfumeVM::from)
                         .collect())
                  }
        )
        .and_then(|result: Vec<PerfumeVM>| {
            Ok(
                result.into_iter()
                  .map(|any| {
                      let keywords= keywords_of(any.id, &conn).ok();
                      PerfumeVM {
                          id: any.id,
                          nombre: any.nombre,
                          id_emp_prod: any.id_emp_prod,
                          tipo: any.tipo,
                          genero: any.genero,
                          edad: any.edad,
                          keywords: keywords,
                          intensities: any.intensities,
                          families: any.families
                      }
                  })
                  .collect()
            )
        })
        .and_then(|result: Vec<PerfumeVM>| {
            Ok(
                result.into_iter()
                  .map(|any| {
                      let intensities= intensity_of(any.id, &conn)
                          .ok()
                          .map(|result|
                               result.into_iter()
                               .map(IntensityVM::from).collect()
                          );
                      PerfumeVM {
                          id: any.id,
                          nombre: any.nombre,
                          id_emp_prod: any.id_emp_prod,
                          tipo: any.tipo,
                          genero: any.genero,
                          edad: any.edad,
                          keywords: any.keywords,
                          intensities: intensities,
                          families: any.families
                      }
                  })
                  .collect()
            )
        })
        .and_then(|result: Vec<PerfumeVM>| {
            Ok(
                result.into_iter()
                  .map(|any| {
                      let families= olf_family_of(any.id, &conn)
                          .ok();
                      PerfumeVM {
                          id: any.id,
                          nombre: any.nombre,
                          id_emp_prod: any.id_emp_prod,
                          tipo: any.tipo,
                          genero: any.genero,
                          edad: any.edad,
                          keywords: any.keywords,
                          intensities: any.intensities,
                          families: families
                      }
                  })
                  .collect()
            )
        })
        .map(|v| Json(v))
        .map_err(|e| status::BadRequest(Some(e.to_string())))
}
