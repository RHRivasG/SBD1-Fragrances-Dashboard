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
	pedido.Status = "ped"

	var condEnvio int
	var condEnvioProv int
	if err := db.Model().
		Table("kmr_contrato_particulares").
		Column("id_envio_pais", "id_envio_pais_prov").
		Where("id = ?", pedidoForm.ContratoEnvio).
		Select(&condEnvio, &condEnvioProv); err != nil {
		return err
	}
	if err := db.Model().
		Table("kmr_envio_pais").
		Column("costo").
		Where("id_pais = ?", condEnvio).
		Where("id_emp_prov = ?", condEnvioProv).
		Select(&pedido.PagoTotal); err != nil {
		return err
	}

	var condPago int
	var condPagoProv int
	if err := db.Model().
		Table("kmr_contrato_particulares").
		Column("id_cond_pago", "id_cond_pago_prov").
		Where("id = ?", pedidoForm.ContratoPago).
		Select(&condPago, &condPagoProv); err != nil {
		return err
	}

	pedido.IDCondcontrapago = condPago
	pedido.IDCondcontrapagoContrato = pedidoForm.ContratoPago
	pedido.IDCondcontrapagoProv = condPagoProv
	pedido.IDCondcontenvio = condEnvio
	pedido.IDCondcontenvioContrato = pedidoForm.ContratoEnvio
	pedido.IDCondcontrapagoProv = condPagoProv

	for _, presentacion := range pedidoForm.Presentaciones {
		var precioUnitario float64
		if err := db.Model().
			Table("kmr_ingrediente_presentacion").
			Column("precio_unitario").
			Where("id = ?", presentacion.ID).
			Select(&precioUnitario); err != nil {
			return err
		}
		pedido.PagoTotal += precioUnitario * float64(presentacion.Cantidad)
	}

	var idPedido int
	if _, err := db.Model(pedido).
		Returning("id", idPedido).
		Insert(); err != nil {
		return err
	}

	if err := CreateLote(idPedido, pedidoForm.Presentaciones, db); err != nil {
		return err
	}

	return nil
}

//CreateLote .
func CreateLote(idPedido int, presentaciones []models.Par, db *pg.DB) error {
	for _, presentacion := range presentaciones {
		var lote models.Lote
		lote.IDPedido = idPedido
		lote.IDIngPresentacion = presentacion.ID
		lote.Cantidad = presentacion.Cantidad
		if err := db.Insert(lote); err != nil {
			return err
		}
	}
	return nil
}
