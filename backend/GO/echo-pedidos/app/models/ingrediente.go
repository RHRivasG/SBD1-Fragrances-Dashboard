package models

//Presentacion .
type Presentacion struct {
	tableName      struct{} `pg:"kmr_ingrediente_presentacion,select:kmr_ingrediente_presentacion"`
	ID             int      `pg:"id,pk" json:"id"`
	IDIfraIng      int      `pg:"id_ifra_ing" json:"id_ifra_ing"`
	Vol            int      `pg:"vol" json:"vol"`
	Unidades       string   `pg:"unidades" json:"unidades"`
	PrecioUnitario int      `pg:"precio_unitario" json:"precio_unitario"`
}
