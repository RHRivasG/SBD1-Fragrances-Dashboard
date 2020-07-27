package models

//Lote .
type Lote struct {
	tableName         struct{} `pg:"kmr_pedido_lote"`
	ID                int      `pg:"id"`
	IDPedido          int      `pg:"id_pedido"`
	IDIngPresentacion int      `pg:"id_ing_prsentacion"`
	Cantidad          int      `pg:"cantidad"`
}
