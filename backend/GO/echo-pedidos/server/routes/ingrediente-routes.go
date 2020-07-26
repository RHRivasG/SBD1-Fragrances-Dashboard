package routes

import (
	"echo-pedidos/app/controllers"

	"github.com/go-pg/pg"
	"github.com/labstack/echo"
)

//SetIngredienteRoutes .
func SetIngredienteRoutes(e *echo.Group, db *pg.DB) {
	controller := controllers.IngredienteController{}

	e.GET("/presentaciones/:idprod/:idprov", func(ctx echo.Context) error {
		return controller.FindPresentacionesIFRA(ctx, db)
	})

}
