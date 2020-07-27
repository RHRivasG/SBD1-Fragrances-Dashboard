package controllers

import (
	"echo-pedidos/app/models"
	"echo-pedidos/app/services"
	"net/http"
	"strconv"

	"github.com/go-pg/pg"
	"github.com/labstack/echo"
)

//PedidoControllerI .
type PedidoControllerI interface {
	CreatePedido(ctx echo.Context, db *pg.DB) error
}

//PedidoController .
type PedidoController struct{}

//CreatePedido .
func (c *PedidoController) CreatePedido(ctx echo.Context, db *pg.DB) error {
	idProd, err := strconv.Atoi(ctx.Param("idprod"))
	if err != nil {
		return echo.NewHTTPError(http.StatusBadRequest, err)
	}
	idProv, err := strconv.Atoi(ctx.Param("idprov"))
	if err != nil {
		return echo.NewHTTPError(http.StatusBadRequest, err)
	}
	var pedidoForm models.PedidoForm
	if err := ctx.Bind(&pedidoForm); err != nil {
		return err
	}
	service := services.PedidoService{}

	if err := service.CreatePedido(idProd, idProv, pedidoForm, db); err != nil {
		return err
	}
	return ctx.JSON(http.StatusCreated, "Se ha creado el pedido")
}
