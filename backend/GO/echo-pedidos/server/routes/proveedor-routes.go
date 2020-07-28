package routes

import (
	"echo-pedidos/app/controllers"

	"github.com/go-pg/pg"
	"github.com/labstack/echo"
)

//SetProveedoresRoutes .
func SetProveedoresRoutes(e *echo.Group, db *pg.DB) {
	controller := controllers.ProveedorController{}

	e.GET("/proveedores/:idprod", func(ctx echo.Context) error {
		return controller.ShowAllConContrato(ctx, db)
	})

}
