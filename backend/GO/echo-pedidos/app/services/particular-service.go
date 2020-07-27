package services

import (
	"echo-pedidos/app/models"
	"fmt"

	"github.com/go-pg/pg"
)

//ParticularServiceI .
type ParticularServiceI interface {
	FindEnviosParticulares(idProd int, idProv int, db *pg.DB) ([]models.EnvioParticular, error)
	FindPagosParticulares(idProd int, idProv int, db *pg.DB) ([]interface{}, error)
}

//ParticularService .
type ParticularService struct{}

//FindEnviosParticulares .
func (s *ParticularService) FindEnviosParticulares(idProd int, idProv int, db *pg.DB) ([]models.EnvioParticular, error) {
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
	enviosParticulares := ([]models.EnvioParticular{})
	if err := db.Model().
		Table("kmr_contrato_particulares").
		ColumnExpr("kmr_contrato_particulares.id, kmr_contrato_particulares.id_envio_pais, id_envio_pais_prov").
		ColumnExpr("kmr_pais.nombre").
		Join("JOIN kmr_pais ON kmr_pais.id = kmr_contrato_particulares.id_envio_pais").
		Where("kmr_contrato_particulares.id_contrato = ?", contrato).
		Select(&enviosParticulares); err != nil {
		return nil, err
	}

	//MUESTRA
	var a []int
	_, err := db.Query(pg.Scan(&a), `SELECT id FROM kmr_empresa_productora`)
	if err != nil {
		fmt.Println(err)
		fmt.Println(a)
	}

	return enviosParticulares, nil
}

//FindPagosParticulares .
func (s *ParticularService) FindPagosParticulares(idProd int, idProv int, db *pg.DB) ([]models.PagoParticular, error) {
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
	pagosParticulares := ([]models.PagoParticular{})
	if err := db.Model().
		Table("kmr_contrato_particulares").
		ColumnExpr("kmr_contrato_particulares.id, kmr_contrato_particulares.id_cond_pago, kmr_contrato_particulares.id_cond_pago_prov").
		ColumnExpr("kmr_condiciones_pago.tipo").
		Join("JOIN kmr_condiciones_pago ON kmr_condiciones_pago.id = kmr_contrato_particulares.id_cond_pago").
		Where("kmr_contrato_particulares.id_contrato = ?", contrato).
		Select(&pagosParticulares); err != nil {
		return nil, err
	}
	return pagosParticulares, nil
}
