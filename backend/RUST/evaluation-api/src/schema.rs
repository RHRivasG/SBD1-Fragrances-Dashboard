table! {
    kmr_asociacion_nacional (id) {
        id -> Int4,
        nombre -> Varchar,
        region -> Varchar,
        id_pais -> Int2,
    }
}

table! {
    kmr_comp_extra (id_perfume, id_ing_presentacion) {
        id_perfume -> Int4,
        id_ing_presentacion -> Int4,
    }
}

table! {
    kmr_condiciones_pago (id, id_emp_prov) {
        id -> Int4,
        id_emp_prov -> Int4,
        tipo -> Varchar,
        cuotas -> Nullable<Int4>,
    }
}

table! {
    kmr_contrato (id, id_emp_prov) {
        id -> Int4,
        id_emp_prod -> Int4,
        id_emp_prov -> Int4,
        exclusividad -> Bool,
        fecha_emision -> Date,
        fecha_cancelado -> Nullable<Date>,
        motivo_cancelo -> Nullable<Varchar>,
    }
}

table! {
    kmr_contrato_particulares (id, id_contrato, id_emp_prov) {
        id -> Int4,
        id_contrato -> Int4,
        id_emp_prov -> Int4,
        id_cond_pago -> Nullable<Int4>,
        id_cond_pago_prov -> Nullable<Int4>,
        id_envio_pais -> Nullable<Int2>,
        id_envio_pais_prov -> Nullable<Int4>,
        descripcion -> Nullable<Varchar>,
    }
}

table! {
    kmr_criterio (id) {
        id -> Int4,
        nombre -> Varchar,
        descripcion -> Varchar,
    }
}

table! {
    kmr_criterio_eval (fechai, id_emp_prod, id_criterio) {
        fechai -> Timestamp,
        fechaf -> Nullable<Date>,
        tipoformula -> Varchar,
        peso -> Int4,
        id_emp_prod -> Int4,
        id_criterio -> Int4,
    }
}

table! {
    kmr_empresa_productora (id) {
        id -> Int4,
        nombre -> Varchar,
        pag_web -> Varchar,
        inf_contacto -> Varchar,
        id_asoc_nacional -> Nullable<Int4>,
    }
}

table! {
    kmr_empresa_proveedora (id) {
        id -> Int4,
        nombre -> Varchar,
        pag_web -> Varchar,
        inf_contacto -> Varchar,
        id_asoc_nacional -> Nullable<Int4>,
    }
}

table! {
    kmr_envio_pais (id_emp_prov, id_pais) {
        id_emp_prov -> Int4,
        id_pais -> Int2,
        dias_entrega -> Int4,
        tipo_transporte -> Bpchar,
        costo -> Nullable<Float4>,
    }
}

table! {
    kmr_ep_fo (id_esencia_perf, id_familia_olf) {
        id_esencia_perf -> Int4,
        id_familia_olf -> Int4,
    }
}

table! {
    kmr_ep_p (id_emp_prod, id_pais) {
        id_emp_prod -> Int4,
        id_pais -> Int2,
    }
}

table! {
    kmr_escala_eval (fechai, id_emp_prod) {
        fechai -> Timestamp,
        fechaf -> Nullable<Date>,
        rangoi -> Int4,
        rangf -> Int4,
        id_emp_prod -> Int4,
    }
}

table! {
    kmr_esencia_perfume (tsca_cas) {
        tsca_cas -> Int4,
        nombre -> Varchar,
        tipo -> Varchar,
        descripcion -> Varchar,
    }
}

table! {
    kmr_familia_olf (id) {
        id -> Int4,
        nombre -> Varchar,
        descripcion -> Varchar,
    }
}

table! {
    kmr_fo_if (id_ifra_ing, id_familia_olf) {
        id_ifra_ing -> Int4,
        id_familia_olf -> Int4,
    }
}

table! {
    kmr_ifra_ingrediente (cas_number) {
        cas_number -> Int4,
        id_emp_prov -> Int4,
        descripcion_visual -> Varchar,
        proceso -> Varchar,
        vida_util -> Varchar,
        solubilidad -> Nullable<Varchar>,
        procesodescripcion -> Nullable<Varchar>,
    }
}

table! {
    kmr_ing_contrato (id, id_contrato, id_emp_prov) {
        id -> Int4,
        id_contrato -> Int4,
        id_emp_prov -> Int4,
        id_ing_otros -> Nullable<Int4>,
        id_ing_ifra -> Nullable<Int4>,
    }
}

table! {
    kmr_ingrediente_otros (ipc) {
        ipc -> Int4,
        nombre -> Varchar,
        tipo -> Varchar,
        tsca_cas -> Nullable<Int4>,
        id_emp_prov -> Nullable<Int4>,
    }
}

table! {
    kmr_ingrediente_presentacion (id) {
        id -> Int4,
        vol -> Int4,
        unidades -> Nullable<Varchar>,
        cantidad_almacenada -> Int4,
        precio_unitario -> Int4,
        id_ifra_ing -> Nullable<Int4>,
        id_ing_otros -> Nullable<Int4>,
    }
}

table! {
    kmr_intensidad (id, id_perfume) {
        id -> Int4,
        id_perfume -> Int4,
        tipo -> Varchar,
        concentracion -> Nullable<Numeric>,
        descripcion -> Nullable<Varchar>,
    }
}

table! {
    kmr_membresia_ifra (id) {
        id -> Int4,
        fechai -> Date,
        fechat -> Nullable<Date>,
        tipo -> Varchar,
        id_emp_prod -> Nullable<Int4>,
        id_emp_prov -> Nullable<Int4>,
    }
}

table! {
    kmr_monolitico (id_perfume, id_esencia_perf) {
        id_perfume -> Int4,
        id_esencia_perf -> Int4,
    }
}

table! {
    kmr_origen (id_ifra_ing, id_pais) {
        id_ifra_ing -> Int4,
        id_pais -> Int2,
    }
}

table! {
    kmr_otros (id_ifra_ing, id_ing_otros) {
        id_ifra_ing -> Int4,
        id_ing_otros -> Int4,
    }
}

table! {
    kmr_pago (id, id_pedido) {
        id -> Int4,
        id_pedido -> Int4,
        fecha_pago -> Date,
        monto -> Numeric,
    }
}

table! {
    kmr_pais (id) {
        id -> Int2,
        nombre -> Varchar,
        continente -> Varchar,
    }
}

table! {
    kmr_palabra_clave (id) {
        id -> Int4,
        palabra_unica -> Varchar,
    }
}

table! {
    kmr_pc_fo (id_familia_olf, id_pal_clave) {
        id_familia_olf -> Int4,
        id_pal_clave -> Int4,
    }
}

table! {
    kmr_pedido (id) {
        id -> Int4,
        id_emp_prod -> Int4,
        id_emp_prov -> Int4,
        fecha_emision -> Date,
        pago_total -> Int4,
        status -> Varchar,
        fecha_confirma -> Nullable<Date>,
        nro_factura -> Nullable<Int4>,
        id_condcontrapago -> Nullable<Int4>,
        id_condcontrapago_contrato -> Nullable<Int4>,
        id_condcontrapago_prov -> Nullable<Int4>,
        id_condcontenvio -> Nullable<Int4>,
        id_condcontenvio_contrato -> Nullable<Int4>,
        id_condcontenvio_prov -> Nullable<Int4>,
    }
}

table! {
    kmr_pedido_lote (id, id_pedido) {
        id -> Int4,
        id_pedido -> Int4,
        id_ing_presentacion -> Int4,
        cantidad -> Int4,
    }
}

table! {
    kmr_perfume (id) {
        id -> Int4,
        nombre -> Varchar,
        tipo -> Bpchar,
        genero -> Bpchar,
        edad -> Varchar,
        id_emp_prod -> Int4,
    }
}

table! {
    kmr_perfume_fases (id_perfume, id_esencia_per) {
        id_perfume -> Int4,
        id_esencia_per -> Int4,
        tiponota -> Bpchar,
    }
}

table! {
    kmr_perfumista (id) {
        id -> Int4,
        nombre -> Varchar,
        primer_apellido -> Varchar,
        segundo_apellido -> Varchar,
        genero -> Bpchar,
        id_pais -> Int2,
    }
}

table! {
    kmr_p_fo (id_perfume, id_familia_olf) {
        id_perfume -> Int4,
        id_familia_olf -> Int4,
    }
}

table! {
    kmr_p_p (id_perfume, id_perfumista) {
        id_perfume -> Int4,
        id_perfumista -> Int4,
    }
}

table! {
    kmr_presentacion (id, id_intensidad, id_perfume) {
        id -> Int4,
        id_intensidad -> Int4,
        id_perfume -> Int4,
        volml -> Int4,
    }
}

table! {
    kmr_prohibida (tscacas) {
        tscacas -> Int4,
        nombre -> Varchar,
    }
}

table! {
    kmr_renueva (id, id_contrato, id_emp_prov) {
        id -> Int4,
        id_contrato -> Int4,
        id_emp_prov -> Int4,
        fecha -> Date,
    }
}

table! {
    kmr_resul_eval (fecha, id_emp_prod, id_emp_prov) {
        fecha -> Int4,
        id_emp_prod -> Int4,
        id_emp_prov -> Int4,
        resultado -> Varchar,
        tipoformula -> Varchar,
    }
}

joinable!(kmr_asociacion_nacional -> kmr_pais (id_pais));
joinable!(kmr_comp_extra -> kmr_ingrediente_presentacion (id_ing_presentacion));
joinable!(kmr_comp_extra -> kmr_perfume (id_perfume));
joinable!(kmr_condiciones_pago -> kmr_empresa_proveedora (id_emp_prov));
joinable!(kmr_contrato -> kmr_empresa_productora (id_emp_prod));
joinable!(kmr_contrato -> kmr_empresa_proveedora (id_emp_prov));
joinable!(kmr_criterio_eval -> kmr_criterio (id_criterio));
joinable!(kmr_criterio_eval -> kmr_empresa_productora (id_emp_prod));
joinable!(kmr_empresa_productora -> kmr_asociacion_nacional (id_asoc_nacional));
joinable!(kmr_empresa_proveedora -> kmr_asociacion_nacional (id_asoc_nacional));
joinable!(kmr_envio_pais -> kmr_empresa_proveedora (id_emp_prov));
joinable!(kmr_envio_pais -> kmr_pais (id_pais));
joinable!(kmr_ep_fo -> kmr_esencia_perfume (id_esencia_perf));
joinable!(kmr_ep_fo -> kmr_familia_olf (id_familia_olf));
joinable!(kmr_ep_p -> kmr_empresa_productora (id_emp_prod));
joinable!(kmr_ep_p -> kmr_pais (id_pais));
joinable!(kmr_escala_eval -> kmr_empresa_productora (id_emp_prod));
joinable!(kmr_fo_if -> kmr_familia_olf (id_familia_olf));
joinable!(kmr_ifra_ingrediente -> kmr_empresa_proveedora (id_emp_prov));
joinable!(kmr_ing_contrato -> kmr_ifra_ingrediente (id_ing_ifra));
joinable!(kmr_ing_contrato -> kmr_ingrediente_otros (id_ing_otros));
joinable!(kmr_ingrediente_otros -> kmr_empresa_proveedora (id_emp_prov));
joinable!(kmr_ingrediente_presentacion -> kmr_ingrediente_otros (id_ing_otros));
joinable!(kmr_intensidad -> kmr_perfume (id_perfume));
joinable!(kmr_membresia_ifra -> kmr_empresa_productora (id_emp_prod));
joinable!(kmr_membresia_ifra -> kmr_empresa_proveedora (id_emp_prov));
joinable!(kmr_monolitico -> kmr_esencia_perfume (id_esencia_perf));
joinable!(kmr_monolitico -> kmr_perfume (id_perfume));
joinable!(kmr_origen -> kmr_pais (id_pais));
joinable!(kmr_otros -> kmr_ingrediente_otros (id_ing_otros));
joinable!(kmr_p_fo -> kmr_familia_olf (id_familia_olf));
joinable!(kmr_p_fo -> kmr_perfume (id_perfume));
joinable!(kmr_p_p -> kmr_perfume (id_perfume));
joinable!(kmr_p_p -> kmr_perfumista (id_perfumista));
joinable!(kmr_pago -> kmr_pedido (id_pedido));
joinable!(kmr_pc_fo -> kmr_familia_olf (id_familia_olf));
joinable!(kmr_pc_fo -> kmr_palabra_clave (id_pal_clave));
joinable!(kmr_pedido -> kmr_empresa_productora (id_emp_prod));
joinable!(kmr_pedido -> kmr_empresa_proveedora (id_emp_prov));
joinable!(kmr_pedido_lote -> kmr_ingrediente_presentacion (id_ing_presentacion));
joinable!(kmr_pedido_lote -> kmr_pedido (id_pedido));
joinable!(kmr_perfume -> kmr_empresa_productora (id_emp_prod));
joinable!(kmr_perfume_fases -> kmr_esencia_perfume (id_esencia_per));
joinable!(kmr_perfume_fases -> kmr_perfume (id_perfume));
joinable!(kmr_perfumista -> kmr_pais (id_pais));
joinable!(kmr_resul_eval -> kmr_empresa_productora (id_emp_prod));
joinable!(kmr_resul_eval -> kmr_empresa_proveedora (id_emp_prov));

allow_tables_to_appear_in_same_query!(
    kmr_asociacion_nacional,
    kmr_comp_extra,
    kmr_condiciones_pago,
    kmr_contrato,
    kmr_contrato_particulares,
    kmr_criterio,
    kmr_criterio_eval,
    kmr_empresa_productora,
    kmr_empresa_proveedora,
    kmr_envio_pais,
    kmr_ep_fo,
    kmr_ep_p,
    kmr_escala_eval,
    kmr_esencia_perfume,
    kmr_familia_olf,
    kmr_fo_if,
    kmr_ifra_ingrediente,
    kmr_ing_contrato,
    kmr_ingrediente_otros,
    kmr_ingrediente_presentacion,
    kmr_intensidad,
    kmr_membresia_ifra,
    kmr_monolitico,
    kmr_origen,
    kmr_otros,
    kmr_pago,
    kmr_pais,
    kmr_palabra_clave,
    kmr_pc_fo,
    kmr_pedido,
    kmr_pedido_lote,
    kmr_perfume,
    kmr_perfume_fases,
    kmr_perfumista,
    kmr_p_fo,
    kmr_p_p,
    kmr_presentacion,
    kmr_prohibida,
    kmr_renueva,
    kmr_resul_eval,
);
