package routes

import (
	"echo-pedidos/app/controllers"

	"github.com/go-pg/pg"
	"github.com/labstack/echo"
)

//SetPedidoRoutes .
func SetPedidoRoutes(e *echo.Group, db *pg.DB) {
	controller := controllers.PedidoController{}

	e.POST("/pedido/:idprod/:idprov", func(ctx echo.Context) error {
		return controller.CreatePedido(ctx, db)
	})

}
