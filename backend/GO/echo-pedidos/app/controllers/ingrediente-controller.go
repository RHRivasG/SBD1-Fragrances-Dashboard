package controllers

import (
	"echo-pedidos/app/services"
	"net/http"
	"strconv"

	"github.com/go-pg/pg"
	"github.com/labstack/echo"
)

//IngredienteControllerI .
type IngredienteControllerI interface {
	FindAll(ctx echo.Context, db *pg.DB) error
	FindPresentacionesIFRA(ctx echo.Context, db *pg.DB) error
}

//IngredienteController .
type IngredienteController struct{}

//FindPresentacionesIFRA .
func (c *IngredienteController) FindPresentacionesIFRA(ctx echo.Context, db *pg.DB) error {
	idProd, err := strconv.Atoi(ctx.Param("idprod"))
	if err != nil {
		return echo.NewHTTPError(http.StatusBadRequest, err)
	}
	idProv, err := strconv.Atoi(ctx.Param("idprov"))
	service := services.IngredienteService{}
	presentaciones, err := service.FindPresentacionesIFRA(idProd, idProv, db)
	if err != nil {
		return echo.NewHTTPError(http.StatusBadRequest, err)
	}
	return ctx.JSON(200, presentaciones)
}
