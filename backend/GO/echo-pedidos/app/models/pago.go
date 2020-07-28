package models

import "time"

//Pago .
type Pago struct {
	tableName struct{}  `pg:"kmr_pago" sql:"kmr_pago"`
	ID        int       `pg:"id"`
	IDPedido  int       `pg:"id_pedido"`
	FechaPago time.Time `pg:"fecha_pago"`
	Monto     float64   `pg:"monto"`
}

//PagoView .
type PagoView struct {
	IDPedido   int
	Cuotas     int
	MontoCuota float64
}
