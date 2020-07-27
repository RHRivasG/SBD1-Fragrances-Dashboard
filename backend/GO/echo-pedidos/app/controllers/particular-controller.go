package controllers

import (
	"echo-pedidos/app/services"
	"net/http"
	"strconv"

	"github.com/go-pg/pg"
	"github.com/labstack/echo"
)

//ParticularControllerI .
type ParticularControllerI interface {
	FindEnviosParticulares(ctx echo.Context, db *pg.DB) error
	FindPagosParticulares(ctx echo.Context, db *pg.DB) error
}

//ParticularController .
type ParticularController struct{}

//FindEnviosParticulares .
func (c *ParticularController) FindEnviosParticulares(ctx echo.Context, db *pg.DB) error {
	idProd, err := strconv.Atoi(ctx.Param("idprod"))
	if err != nil {
		return echo.NewHTTPError(http.StatusBadRequest, err)
	}
	idProv, err := strconv.Atoi(ctx.Param("idprov"))
	if err != nil {
		return echo.NewHTTPError(http.StatusBadRequest, err)
	}
	service := services.ParticularService{}
	enviosParticulares, err := service.FindEnviosParticulares(idProd, idProv, db)
	if err != nil {
		return echo.NewHTTPError(http.StatusBadRequest, err)
	}
	return ctx.JSON(200, enviosParticulares)
}

//FindPagosParticulares .
func (c *ParticularController) FindPagosParticulares(ctx echo.Context, db *pg.DB) error {
	idProd, err := strconv.Atoi(ctx.Param("idprod"))
	if err != nil {
		return echo.NewHTTPError(http.StatusBadRequest, err)
	}
	idProv, err := strconv.Atoi(ctx.Param("idprov"))
	service := services.ParticularService{}
	pagosParticulares, err := service.FindPagosParticulares(idProd, idProv, db)
	if err != nil {
		return echo.NewHTTPError(http.StatusBadRequest, err)
	}
	return ctx.JSON(200, pagosParticulares)
}
