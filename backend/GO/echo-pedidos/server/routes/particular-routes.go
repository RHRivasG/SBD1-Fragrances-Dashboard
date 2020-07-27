package routes

import (
	"echo-pedidos/app/controllers"

	"github.com/go-pg/pg"
	"github.com/labstack/echo"
)

//SetParticularRoutes .
func SetParticularRoutes(e *echo.Group, db *pg.DB) {
	controller := controllers.ParticularController{}

	e.GET("/envios-particulares/:idprod/:idprov", func(ctx echo.Context) error {
		return controller.FindEnviosParticulares(ctx, db)
	})

	e.GET("/pagos-particulares/:idprod/:idprov", func(ctx echo.Context) error {
		return controller.FindPagosParticulares(ctx, db)
	})

}
