package services

import (
	"echo-pedidos/app/models"
	"fmt"

	"github.com/go-pg/pg"
)

//IngredienteServiceI .
type IngredienteServiceI interface {
	FindAll(db *pg.DB) ([]models.Presentacion, error)
	FindPresentacionesIFRA(idProd int, idProv int, db *pg.DB) ([]models.Presentacion, error)
}

//IngredienteService .
type IngredienteService struct{}

//FindAll .
func (s *IngredienteService) FindAll(db *pg.DB) ([]models.Presentacion, error) {
	var ids []int
	db.Model().
		Table("kmr_ingrediente_presentacion").
		ColumnExpr("array_agg(id)").
		Select(pg.Array(&ids))
	fmt.Println(ids)
	var presentaciones []models.Presentacion
	if err := db.Model().Table("kmr_ingrediente_presentacion").Column("id", "id_ifra_ing", "vol", "unidades", "precio_unitario").Select(&presentaciones); err != nil {
		return nil, err
	}
	return presentaciones, nil
}

//FindPresentacionesIFRA .
func (s *IngredienteService) FindPresentacionesIFRA(idProd int, idProv int, db *pg.DB) ([]models.Presentacion, error) {
	var contrato int
	if err := db.Model().
		Table("kmr_contrato").
		Column("id").
		Where("id_emp_prod = ?", idProd).
		Where("id_emp_prov = ?", idProv).
		Limit(1).
		Select(&contrato); err != nil {
		return nil, err
	}
	var ids []int
	if err := db.Model().
		Table("kmr_ing_contrato").
		Where("id_contrato = ?", contrato).
		ColumnExpr("array_agg(id_ing_ifra)").
		Select(pg.Array(&ids)); err != nil {
		return nil, err
	}
	var presentaciones []models.Presentacion
	if err := db.Model().
		Table("kmr_ingrediente_presentacion").
		Column("id", "id_ifra_ing", "vol", "unidades", "precio_unitario").
		Where("id_ifra_ing in (?)", pg.In(ids)).
		Select(&presentaciones); err != nil {
		return nil, err
	}
	return presentaciones, nil
}
