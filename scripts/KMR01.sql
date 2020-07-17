CREATE TABLE KMR_Pais(
    id SMALLINT NOT NULL PRIMARY KEY,
    nombre VARCHAR(255) NOT NULL,
    continente VARCHAR(30) NOT NULL
);

CREATE TABLE KMR_Asociacion_Nacional(
    id INT NOT NULL PRIMARY KEY,
    nombre VARCHAR NOT NULL,
    region VARCHAR NOT NULL,
    id_pais SMALLINT NOT NULL REFERENCES KMR_Pais(id)
);

CREATE TABLE KMR_Empresa_Productora(
    id INT NOT NULL PRIMARY KEY,
    nombre VARCHAR NOT NULL,
    pag_web VARCHAR NOT NULL,
    inf_contacto VARCHAR NOT NULL,
    id_asoc_nacional INT REFERENCES KMR_Asociacion_Nacional(id)
);

CREATE TABLE KMR_EP_P(
    id_emp_prod INT NOT NULL REFERENCES KMR_Empresa_Productora(id),
    id_pais INT NOT NULL REFERENCES KMR_Pais(id),
    PRIMARY KEY (id_emp_prod,id_pais)
);

CREATE TABLE KMR_Perfume(
    id INT NOT NULL PRIMARY KEY,
    nombre VARCHAR NOT NULL,
    tipo CHAR NOT NULL CHECK(tipo='F' OR tipo='M'),
    genero CHAR NOT NULL CHECK(genero='H' OR genero='M' OR genero='U'),
    edad VARCHAR NOT NULL CHECK(edad='JO' OR edad='AD' OR edad='AT'),
    id_emp_prod INT NOT NULL REFERENCES KMR_Empresa_Productora(id)
);

CREATE TABLE KMR_Perfumista(
    id INT NOT NULL PRIMARY KEY,
    nombre VARCHAR NOT NULL,
    primer_apellido VARCHAR NOT NULL,
    segundo_apellido VARCHAR NOT NULL,
    genero VARCHAR NOT NULL CHECK(genero='H' OR genero='M'),
    id_pais SMALLINT NOT NULL REFERENCES KMR_Pais(id)
);

CREATE TABLE KMR_P_P(
    id_perfume INT NOT NULL REFERENCES KMR_Perfume(id),
    id_perfumista INT NOT NULL REFERENCES KMR_Perfumista(id),
    PRIMARY KEY (id_perfume, id_perfumista)
);

CREATE TABLE KMR_Intensidad(
    id INT NOT NULL,
    id_perfume INT NOT NULL REFERENCES KMR_Perfume(id),
    tipo VARCHAR NOT NULL,
    concentracion DECIMAL,
    descripcion VARCHAR,
    PRIMARY KEY (id,id_perfume)
);

CREATE TABLE KMR_Presentacion(
    id INT NOT NULL,
    id_intensidad INT NOT NULL REFERENCES KMR_Intensidad(id),
    id_perfume INT NOT NULL REFERENCES KMR_Intensidad(id),
    volml INT NOT NULL,
    PRIMARY KEY (id,id_intensidad,id_perfume)
);

CREATE TABLE KMR_Esencia_Perfume(
    tsca_cas INT NOT NULL PRIMARY KEY,
    nombre VARCHAR NOT NULL,
    tipo VARCHAR NOT NULL,
    descripcion VARCHAR NOT NULL
);

CREATE TABLE KMR_Perfume_Fases(
    id_perfume INT NOT NULL REFERENCES KMR_Perfume(id),
    id_esencia_per INT NOT NULL REFERENCES KMR_Esencia_Perfume(id),
    tiponota VARCHAR NOT NULL,
    PRIMARY KEY (id_perfume,id_esencia_per)
);

CREATE TABLE KMR_Monolitico(
    id_perfume INT NOT NULL REFERENCES KMR_Perfume(id),
    id_esencia_perf INT NOT NULL REFERENCES KMR_Esencia_Perfume(id),
    PRIMARY KEY (id_perfume,id_esencia_per)
);

CREATE TABLE KMR_Familia_Olf(
    id INT NOT NULL PRIMARY KEY,
    nombre VARCHAR NOT NULL,
    descripcion VARCHAR NOT NULL
);

CREATE TABLE KMR_P_FO(
    id_perfume INT NOT NULL REFERENCES KMR_Perfume(id),
    id_familia_olf INT NOT NULL REFERENCES KMR_Familia_Olf(id),
    PRIMARY KEY (id_perfume,id_familia_olf)
);

CREATE TABLE KMR_EP_FO(
    id_esencia_perf INT NOT NULL REFERENCES KMR_Esencia_Perfume(id),
    id_familia_olf INT NOT NULL REFERENCES KMR_Familia_Olf(id),
    PRIMARY KEY (id_esencia_perf,id_familia_olf)
);

CREATE TABLE KMR_Palabra_Clave(
    id INT NOT NULL PRIMARY KEY, 
    palabra_unica VARCHAR UNIQUE NOT NULL
);

CREATE TABLE KMR_PC_FO(
    id_familia_olf INT NOT NULL REFERENCES KMR_Familia_Olf(id),
    id_pal_clave INT NOT NULL REFERENCES KMR_Palabra_Clave(id),
    PRIMARY KEY (id_familia_olf, id_pal_clave)
);

CREATE TABLE KMR_Escala_Eval(
    fechai DATE NOT NULL,
    fechaf DATE,
    rangoi INT NOT NULL,
    rangf INT NOT NULL,
    id_emp_prod INT NOT NULL REFERENCES KMR_Empresa_Productora(id),
    PRIMARY KEY (fechai, id_emp_prod)
);

CREATE TABLE KMR_Criterio(
    id INT NOT NULL PRIMARY KEY,
    nombre VARCHAR NOT NULL,
    descripcion VARCHAR NOT NULL
);

CREATE TABLE KMR_Criterio_Eval(
    fechai DATE NOT NULL,
    fechaf DATE,
    tipoformula VARCHAR NOT NULL CHECK(tipoformula='E' OR tipoformula='I'),
    id_emp_prod INT NOT NULL REFERENCES KMR_Empresa_Productora(id),
    id_criterio INT REFERENCES KMR_Criterio(id),
    PRIMARY KEY (fechai, id_emp_prod,id_criterio)
);

CREATE TABLE KMR_Empresa_Proveedora(
    id INT NOT NULL PRIMARY KEY,
    nombre VARCHAR NOT NULL,
    pag_web VARCHAR NOT NULL,
    inf_contacto VARCHAR NOT NULL,
    id_asoc_nacional INT REFERENCES KMR_Asociacion_Nacional(id)
);

CREATE TABLE KMR_IFRA_Ingrediente(
    cas_number INT NOT NULL,
    id_emp_prov INT NOT NULL REFERENCES KMR_Empresa_Proveedora(id),
    descripcion_visual VARCHAR NOT NULL,
    proceso VARCHAR NOT NULL,
    vida_util VARCHAR NOT NULL,
    solubilidad VARCHAR,
    procesodescripcion VARCHAR,
    PRIMARY KEY (cas_number,id_emp_prov)
);

CREATE TABLE KMR_Origen(
    id_ifra_ing INT NOT NULL REFERENCES KMR_IFRA_Ingrediente(cas_number),
    id_emp_prov INT NOT NULL REFERENCES KMR_IFRA_Ingrediente(id_emp_prov),
    id_pais INT NOT NULL REFERENCES KMR_Pais(id),
    PRIMARY KEY (id_ifra_ing,id_pais)
);

CREATE TABLE KMR_FO_IF(
    id_ifra_ing INT NOT NULL REFERENCES KMR_IFRA_Ingrediente(cas_number),
    id_emp_prov INT NOT NULL REFERENCES KMR_IFRA_Ingrediente(id_emp_prov),
    id_familia_olf INT NOT NULL REFERENCES KMR_Familia_Olf(id),
    PRIMARY KEY (id_ifra_ing,id_emp_prov,id_familia_olf)
);

CREATE TABLE KMR_Ingrediente_Otros(
    ipc INT NOT NULL PRIMARY KEY,
    nombre VARCHAR NOT NULL,
    tipo VARCHAR NOT NULL,
    tsca_cas INT,
    id_emp_prov INT REFERENCES KMR_Empresa_Proveedora(id)
);

CREATE TABLE KMR_Otros(
    id_ifra_ing INT NOT NULL REFERENCES KMR_IFRA_Ingrediente(cas_number),
    id_emp_prov INT NOT NULL REFERENCES KMR_IFRA_Ingrediente(id_emp_prov),
    id_ing_otros INT NOT NULL REFERENCES KMR_Ingrediente_Otros(ipc),
    PRIMARY KEY (id_ifra_ing,id_emp_prov,id_ing_otros)
);

CREATE TABLE KMR_Ingrediente_Presentacion(
    id INT NOT NULL PRIMARY KEY,
    vol INT NOT NULL,
    cantidad_almacenada INT NOT NULL,
    precio_unitario INT NOT NULL,
    unidades INT,
    envase CHAR CHECK(envase='P' OR envase='V'),
    id_ifra_ing INT REFERENCES KMR_IFRA_Ingrediente(cas_number),
    id_emp_prov INT REFERENCES KMR_IFRA_Ingrediente(id_emp_prov),
    id_ing_otros INT REFERENCES KMR_Ingrediente_Otros(ipc)
);

CREATE TABLE KMR_Comp_Extra(
    id_perfume INT NOT NULL REFERENCES KMR_Perfume(id),
    id_ing_presentacion INT NOT NULL REFERENCES KMR_Ingrediente_Presentacion(id),
    PRIMARY KEY (id_perfume,id_ing_presentacion)
);

CREATE TABLE KMR_Prohibida(
    tscacas INT NOT NULL PRIMARY KEY,
    nombre VARCHAR NOT NULL
);

CREATE TABLE KMR_Condiciones_Pago(
    id INT NOT NULL,
    id_emp_prov INT NOT NULL REFERENCES KMR_Empresa_Proveedora(id),
    tipo VARCHAR NOT NULL,
    cuotas INT,
    PRIMARY KEY (id,id_emp_prov)
);

CREATE TABLE KMR_Envio_Pais(
    id_emp_prov INT NOT NULL REFERENCES KMR_Empresa_Proveedora(id),
    id_pais INT NOT NULL REFERENCES KMR_Pais(id),
    dias_entrega INT NOT NULL,
    tipo_transporte CHAR NOT NULL CHECK (tipo_transporte='A' OR tipo_transporte='B'),
    costo INT,
    PRIMARY KEY (id_emp_prov, id_pais)
);

CREATE TABLE KMR_Contrato(
    id INT NOT NULL,
    id_emp_prod INT NOT NULL REFERENCES KMR_Empresa_Productora(id),
    id_emp_prov INT NOT NULL REFERENCES KMR_Empresa_Proveedora(id),
    fecha_emision DATE NOT NULL,
    fecha_cancelado DATE,
    motivo_cancelo VARCHAR,
    PRIMARY KEY (id,id_emp_prov)
);

CREATE TABLE KMR_Renueva(
    id INT NOT NULL,
    id_contrato INT NOT NULL REFERENCES KMR_Contrato(id),
    fecha DATE NOT NULL,
    PRIMARY KEY (id, id_contrato)
);

CREATE KMR_Contrato_Particulares(
    id INT NOT NULL,
    id_contrato INT NOT NULL REFERENCES KMR_Contrato(id),
    id_cond_pago INT REFERENCES KMR_Condiciones_Pago(id),
    id_emp_prov INT REFERENCES KMR_Condiciones_Pago(id_emp_prov),
    id_envio_pais INT REFERENCES KMR_Pais(id),
    descripcion VARCHAR,
    PRIMARY KEY (id,id_contrato)
);

CREATE KMR_Ing_Contrato(
    id INT NOT NULL,
    id_contrato INT NOT NULL REFERENCES KMR_Contrato(id),
    id_emp_prov INT NOT NULL REFERENCES KMR_Contrato(id_emp_prov),
    id_ing_otros INT REFERENCES KMR_Ingrediente_Otros(ipc),
    id_ing_ifra INT REFERENCES KMR_IFRA_Ingrediente(cas_number),
    id_ing_ifra_prov INT REFERENCES KMR_IFRA_Ingrediente(id_emp_prov),
    PRIMARY KEY(id,id_contrato,id_emp_prov)
);

CREATE TABLE KMR_Pedido(
    id INT NOT NULL PRIMARY KEY,
    id_emp_prod INT NOT NULL REFERENCES KMR_Empresa_Productora(id),
    id_emp_prov INT NOT NULL REFERENCES KMR_Empresa_Proveedora(id),
    fecha_emision DATE NOT NULL,
    pago_total INT NOT NULL,
    fecha_confirma DATE,
    nro_factura INT,
    id_contrato_pago INT REFERENCES KMR_Contrato_Particulares(id),
    id_contrato INT REFERENCES KMR_Contrato_Particulares(id_contrato),
    id_condcontenvio INT REFERENCES KMR_Contrato_Particulares(id_envio_pais)
);

CREATE TABLE KMR_Pago(
    id INT NOT NULL,
    id_pedido INT NOT NULL REFERENCES KMR_Pedido(id),
    fecha_pago DATE NOT NULL,
    monto DECIMAL NOT NULL
);

CREATE TABLE KMR_Pedido_Lote(
    id INT NOT NULL,
    id_pedido INT NOT NULL REFERENCES KMR_Pedido(id),
    id_ing_presentacion INT NOT NULL REFERENCES KMR_Ingrediente_Presentacion(id),
    cantidad INT NOT NULL,
    PRIMARY KEY (id,id_pedido)
);

CREATE TABLE Resul_Eval(
    fecha INT NOT NULL,
    id_emp_prod INT NOT NULL REFERENCES KMR_Empresa_Productora(id),
    id_emp_prov INT NOT NULL REFERENCES KMR_Empresa_Proveedora(id),
    resultado VARCHAR NOT NULL,
    tipoformula VARCHAR NOT NULL,
    PRIMARY KEY (fecha,id_emp_prod,id_emp_prov)
);

CREATE TABLE KMR_Membresia_IFRA(
    id INT NOT NULL PRIMARY KEY,
    fechai DATE NOT NULL,
    fechat DATE,
    tipo VARCHAR NOT NULL CHECK(tipo='PD' OR tipo='PV'),
    id_emp_prod INT REFERENCES KMR_Empresa_Productora(id),
    id_emp_prov INT REFERENCES KMR_Empresa_Proveedora(id)
);
