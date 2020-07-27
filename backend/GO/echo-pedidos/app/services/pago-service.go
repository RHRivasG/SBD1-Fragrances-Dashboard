package services

import (
	"echo-pedidos/app/models"

	"github.com/go-pg/pg"
)

//PagoServiceI .
type PagoServiceI interface {
	ShowPago(idPedido int, db *pg.DB) models.PagoView
	CreatePago()
}
