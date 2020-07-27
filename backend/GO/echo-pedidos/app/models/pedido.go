package models

import "time"

//Pedido .
type Pedido struct {
	tableName                struct{}  `pg:"kmr_pedido"`
	ID                       int       `pg:"id,pk" json:"id"`
	IDEmpProd                int       `pg:"id_emp_prod" json:"id_emp_prod"`
	IDEmpProv                int       `pg:"id_emp_prov" json:"id_emp_prov"`
	FechaEmision             time.Time `pg:"fecha_emision" json:"fecha_emision"`
	PagoTotal                float64   `pg:"pago_total" json:"pago_total"`
	FechaConfirma            time.Time `pg:"fecha_confirma" json:"fecha_confirma"`
	NroFactura               int       `pg:"nro_factura" json:"nro_factura"`
	IDCondcontrapago         int       `pg:"id_condcontrapago" json:"id_condcontrapago"`
	IDCondcontrapagoContrato int       `pg:"id_condcontrapago_contrato" json:"id_condcontrapago_contrato"`
	IDCondcontrapagoProv     int       `pg:"id_condcontrapago_prov" json:"id_condcontrapago_prov"`
	IDCondcontenvio          int       `pg:"id_condcontenvio" json:"id_condcontenvio"`
	IDCondcontenvioContrato  int       `pg:"id_condcontenvio_contrato" json:"id_condcontenvio_contrato"`
	IDCondcontenvioProv      int       `pg:"id_condcontenvio_prov" json:"id_condcontenvio_prov"`
}

//PedidoForm .
type PedidoForm struct {
	ContratoPago   int   `json:"contrato_pago"`
	ContratoEnvio  int   `json:"contrato_envio"`
	Presentaciones []Par `json:"presentaciones"`
}

//Par .
type Par struct {
	ID       int
	Cantidad int
}
