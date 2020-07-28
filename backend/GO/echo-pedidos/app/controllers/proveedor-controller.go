package controllers

import (
	"echo-pedidos/app/services"
	"net/http"
	"strconv"

	"github.com/go-pg/pg"
	"github.com/labstack/echo"
)

//ProveedorControllerI .
type ProveedorControllerI interface {
	ShowAllConContrato(ctx echo.Context, db *pg.DB) error
}

//ProveedorController .
type ProveedorController struct{}

//ShowAllConContrato .
func (c *ProveedorController) ShowAllConContrato(ctx echo.Context, db *pg.DB) error {
	idProd, err := strconv.Atoi(ctx.Param("idprod"))
	if err != nil {
		return echo.NewHTTPError(http.StatusBadRequest, err)
	}
	service := services.ProveedorService{}
	proveedores, err := service.ShowAllConContrato(idProd, db)
	if err != nil {
		return echo.NewHTTPError(http.StatusBadRequest, err)
	}
	return ctx.JSON(200, proveedores)

}
