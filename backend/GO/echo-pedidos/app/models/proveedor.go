package models

//Proveedor .
type Proveedor struct {
	tableName      struct{} `pg:"kmr_empresa_proveedora" sql:"kmr_empresa_proveedora"`
	ID             int      `pg:"id" json:"id"`
	Nombre         string   `pg:"nombre" json:"nombre"`
	PagWeb         string   `pg:"pag_web" json:"pag_web"`
	InfContacto    string   `pg:"inf_contacto" json:"inf_contacto"`
	IDAsocNacional int      `pg:"id_asoc_nacional" json:"id_asoc_nacional"`
}
