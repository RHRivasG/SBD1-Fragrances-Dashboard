package models

//EnvioParticular .
type EnvioParticular struct {
	tableName       struct{} `pg:"kmr_contrato_particulares,alias:kmr_contrato_particulares, select:kmr_contrato_particulares"`
	ID              int      `pg:"id,pk" json:"id"`
	IDEnvioPais     int      `pg:"id_envio_pais" json:"id_envio_pais"`
	IDEnvioPaisProv int      `pg:"id_envio_pais_prov" json:"id_envio_pais_prov"`
	Nombre          string   `pg:"nombre" json:"nombre"`
}

//PagoParticular .
type PagoParticular struct {
	tableName      struct{} `pg:"kmr_contrato_particulares,alias:kmr_contrato_particulares, select:kmr_contrato_particulares"`
	ID             int      `pg:"id,pk" json:"id"`
	IDCondPago     int      `pg:"id_cond_pago" json:"id_cond_pago"`
	IDCondPagoProv int      `pg:"id_cond_pago_prov" json:"id_cond_pago_prov"`
	Tipo           string   `pg:"tipo" json:"tipo"`
}
