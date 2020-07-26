package main

import (
	"echo-pedidos/config"
	"echo-pedidos/server"
)

func main() {
	db := config.ConnectDatabase()
	defer db.Close()
	server.StartServer(db)

}
