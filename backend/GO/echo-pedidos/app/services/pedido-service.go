package services

import (
	"echo-pedidos/app/models"
	"fmt"
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

	var condEnvioContrato int
	var condEnvioProv int
	var pais int
	if err := db.Model().
		Table("kmr_contrato_particulares").
		Column("id_contrato", "id_emp_prov", "id_envio_pais").
		Where("id = ?", pedidoForm.ContratoEnvio).
		Select(&condEnvioContrato, &condEnvioProv, &pais); err != nil {
		fmt.Println(err)
		return err
	}
	var a float64
	if err := db.Model().
		Table("kmr_envio_pais").
		Column("costo").
		Where("id_pais = ?", pais).
		Where("id_emp_prov = ?", condEnvioProv).
		Select(&a); err != nil {
		fmt.Println(err)
		return err
	}
	pedido.PagoTotal = a
	var condPagoContrato int
	var condPagoProv int
	if err := db.Model().
		Table("kmr_contrato_particulares").
		Column("id_contrato", "id_emp_prov").
		Where("id = ?", pedidoForm.ContratoPago).
		Select(&condPagoContrato, &condPagoProv); err != nil {
		fmt.Println(err)
		return err
	}

	pedido.IDCondcontrapago = pedidoForm.ContratoPago
	pedido.IDCondcontrapagoContrato = condPagoContrato
	pedido.IDCondcontrapagoProv = condPagoProv
	pedido.IDCondcontenvio = pedidoForm.ContratoEnvio
	pedido.IDCondcontenvioContrato = condEnvioContrato
	pedido.IDCondcontenvioProv = condEnvioProv

	for _, presentacion := range pedidoForm.Presentaciones {
		var precioUnitario float64
		if err := db.Model().
			Table("kmr_ingrediente_presentacion").
			Column("precio_unitario").
			Where("id = ?", presentacion.ID).
			Select(&precioUnitario); err != nil {
			fmt.Println(err)
			return err
		}
		pedido.PagoTotal += precioUnitario * float64(presentacion.Cantidad)
	}

	var idPedido int
	if _, err := db.Model(&pedido).
		Returning("id", idPedido).
		Insert(); err != nil {
		return err
	}
	if err := CreateLote(pedido.ID, pedidoForm.Presentaciones, db); err != nil {
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
		if err := db.Insert(&lote); err != nil {
			fmt.Println(err)
			return err
		}
	}
	return nil
}

//ShowAll .
func (s *PedidoService) ShowAll(idProd int, db *pg.DB) ([]models.Pedido, error) {
	var pedidos []models.Pedido
	if err := db.Model(&pedidos).Select(); err != nil {
		return nil, err
	}
	return pedidos, nil
}
