package config

import "github.com/go-pg/pg"

//ConnectDatabase .
func ConnectDatabase() *pg.DB {
	db := pg.Connect(&pg.Options{
		Addr:     ":5432",
		User:     "postgres",
		Password: "postgres",
		Database: "sbd1schema",
	})
	return db
}
