INSERT INTO KMR_Escala_Eval
    VALUES
	('2020-07-20',NULL,1,10,1),
	('2020-07-10',NULL,1,10,1);


INSERT INTO KMR_Envio_Pais
    VALUES (1,147,10,'A',30);

INSERT INTO KMR_Contrato
    VALUES (1,1,1,FALSE,'2019-07-04',NULL,NULL);

INSERT INTO KMR_Contrato_Particulares
    VALUES (1,1,1,1,1,147,1,'descripcion');

INSERT INTO KMR_Ing_Contrato (id,id_contrato,id_emp_prov,id_ing_ifra)
    VALUES 
	(1,1,1,131766739),
	(2,1,1,14352615);

INSERT INTO KMR_EP_P
    VALUES
	(1,155),
	(1,89),
	(1,147),
	(1,126),
	(1,91),
	(2,44),
	(2,4),
	(2,68);
