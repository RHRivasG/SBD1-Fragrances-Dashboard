package services

import (
	"echo-pedidos/app/models"
	"fmt"

	"github.com/go-pg/pg"
)

//ProveedorServiceI .
type ProveedorServiceI interface {
	ShowAllConContrato(idprod int, db *pg.DB) ([]models.Proveedor, error)
}

//ProveedorService .
type ProveedorService struct{}

//ShowAllConContrato .
func (s *ProveedorService) ShowAllConContrato(idprod int, db *pg.DB) ([]models.Proveedor, error) {
	var idsprov []int
	if err := db.Model().
		Table("kmr_contrato").
		Where("id_emp_prod = ?", idprod).
		ColumnExpr("array_agg(id_emp_prov)").
		Select(pg.Array(&idsprov)); err != nil {
		return nil, err
	}

	fmt.Println(idsprov)

	var provs []models.Proveedor
	if err := db.Model().
		Table("kmr_empresa_proveedora").
		Where("id in (?)", pg.In(idsprov)).
		Select(&provs); err != nil {
		return nil, err
	}

	return provs, nil

}
