use crate::lib::perfume_recommender::model::OlfFamily;
use crate::schema::*;
use diesel::{BoxableExpression, pg::Pg, QueryResult};
use super::model::{Keyword, Perfume, Intensity};
use diesel::prelude::*;

pub fn all(conn: &crate::lib::db::Connection) -> QueryResult<Vec<Perfume>>{
    use kmr_perfume::*;
    table.select(all_columns)
        .load(&conn.0)
}

pub fn keywords_of(id_perfume: i32, conn: &crate::lib::db::Connection) -> QueryResult<Vec<Keyword>> {
    diesel::sql_query(format!("SELECT k.* FROM \
                       kmr_palabra_clave k \
                       JOIN kmr_pc_fo po ON po.id_pal_clave = k.id \
                       JOIN kmr_familia_olf fo ON po.id_familia_olf = fo.id \
                       JOIN kmr_p_fo pfo ON pfo.id_familia_olf = fo.id \
                       WHERE pfo.id_perfume = {}", id_perfume)).load(&conn.0)
}

pub fn intensity_of(id_perfume: i32, conn: &crate::lib::db::Connection) -> QueryResult<Vec<Intensity>> {
    kmr_intensidad::table.filter(kmr_intensidad::id_perfume.eq(id_perfume))
                         .load(&conn.0)
}

pub fn olf_family_of(id_perfume: i32, conn: &crate::lib::db::Connection) -> QueryResult<Vec<OlfFamily>> {
    diesel::sql_query(format!("SELECT fo.* FROM \
                               kmr_familia_olf fo \
                               JOIN kmr_p_fo pfo ON pfo.id_familia_olf = fo.id \
                               WHERE pfo.id_perfume = {}", id_perfume)).load(&conn.0)
}
