package services

import (
	"echo-pedidos/app/models"
	"time"

	"github.com/go-pg/pg"
)

//PerdidoServiceI .
type PerdidoServiceI interface {
	ShowAll(idProd int, db *pg.DB) ([]models.Pedido, error)
	CreatePedido(idProd int, idProv int, pedido models.PedidoForm, db *pg.DB) error
}

//PedidoService .
type PedidoService struct{}

//CreatePedido .
func (s *PedidoService) CreatePedido(idProd int, idProv int, pedidoForm models.PedidoForm, db *pg.DB) error {
	var pedido models.Pedido
	pedido.IDEmpProd = idProd
	pedido.IDEmpProv = idProv
	pedido.FechaEmision = time.Now()

	var condEnvio int
	if err := db.Model().
		Table("kmr_contrato_particulares").
		Column("id_envio_pais").
		Where("id = ?", pedidoForm.ContratoEnvio).
		Select(&condEnvio); err != nil {
		return err
	}
	if err := db.Model().
		Table("kmr_envio_pais").
		Column("costo").
		Where("id_pais = ?", condEnvio).
		Where("id_emp_prov = ?", idProv).
		Select(&pedido.PagoTotal); err != nil {
		return err
	}

	for _, presentacion := range pedidoForm.Presentaciones {
		var precioUnitario float32
		var precioTotal float32
		if err := db.Model().
			Table("kmr_ingrediente_presentacion").
			Column("precio_unitario").
			Where("id = ?", presentacion.ID).
			Select(&precioUnitario); err != nil {
			return err
		}
		precioTotal += precioUnitario * presentacion.Cantidad
	}

	return nil
}
