package server

import (
	"echo-pedidos/server/routes"

	"github.com/go-pg/pg"
	"github.com/go-playground/validator/v10"
	"github.com/labstack/echo"
	"github.com/labstack/echo/middleware"
)

//CustomValidator struct
type CustomValidator struct {
	validator *validator.Validate
}

//Validate data
func (cv *CustomValidator) Validate(i interface{}) error {
	return cv.validator.Struct(i)
}

//StartServer ...
func StartServer(db *pg.DB) {
	e := echo.New()

	//Validator
	e.Validator = &CustomValidator{validator: validator.New()}

	//Middlewares
	e.Use(middleware.Logger())
	e.Use(middleware.Recover())
	e.Use(middleware.CORS())

	//Routes
	api := e.Group("/api")
	{
		routes.SetIngredienteRoutes(api, db)
		routes.SetParticularRoutes(api, db)
		routes.SetPedidoRoutes(api, db)
		routes.SetProveedoresRoutes(api, db)
	}

	e.Logger.Fatal(e.Start(":1323"))
}
