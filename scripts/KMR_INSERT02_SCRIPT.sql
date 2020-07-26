INSERT INTO KMR_Criterio_Eval
    VALUES
	('2020-07-20',NULL,'I', 33, 1,1),
	('2020-07-10',NULL,'I', 67, 1,2),
	('2020-07-10',NULL,'E', 100, 1,3),
    ('2020-07-20',NULL,'I', 33, 2, 1),
	('2020-07-10',NULL,'I', 67, 2,2),
	('2020-07-10',NULL,'E', 100, 2,3),
    ('2020-07-20',NULL,'I', 33, 3,1),
	('2020-07-10',NULL,'I', 67, 3,2),
	('2020-07-10',NULL,'E', 100, 3,3),
	('2020-07-20',NULL,'I', 33, 4,1),
	('2020-07-10',NULL,'I', 67, 4,2),
	('2020-07-10',NULL,'E', 100, 4,3);
	('2020-07-20',NULL,'I', 33, 5,1),
	('2020-07-10',NULL,'I', 67, 5,2),
	('2020-07-10',NULL,'E', 100, 5,3);
    
INSERT INTO KMR_Escala_Eval
    VALUES
	('2020-07-20',NULL,1,10,1),
	('2020-07-10',NULL,1,5,1);

INSERT INTO KMR_Condiciones_Pago
    VALUES (1,'AP',3);


INSERT INTO KMR_Envio_Pais
    VALUES (1,147,10,'A',30);

INSERT INTO KMR_Contrato
    VALUES
    (1,1,1,FALSE,'2019-07-04',NULL,NULL),
    (2,1,2,TRUE,'2020-07-24',NULL,NULL),
    (3,2,3,FALSE,'2020-06-15',NULL,NULL),
    (4,2,4,FALSE,'2018-05-16',NULL,NULL),
    (5,3,1,FALSE,'2020-11-24',NULL,NULL),
    (6,3,3,FALSE,'2019-02-19',NULL,NULL),
    (7,4,2,FALSE,'2020-12-25',NULL,NULL),
    (8,5,2,FALSE,'2020-01-24',NULL,NULL),
    (9,5,4,TRUE,'2020-03-23',NULL,NULL);

INSERT INTO KMR_Contrato_Particulares
    VALUES
    (1,1,1,NULL,NULL,147,1,'Iff contrato a Keva con envio a Panaama'),
    (2,1,1,1,1,NULL,NULL,'Iff contrato a Keva con condicion de pago aplazado con 3 cuotas'),
    (3,2,2,2,2,NULL,NULL, 'Iff contrato a essence con condicion de pago Aplazado con 2 cuotas'),
    (4,2,2,NULL,NULL,91,2, 'Iff contrato a essence con envio a la India'),
    (5,3,3,NULL,NULL,68,3, 'Firmenich contrato a Privi con envio a America'),
    (6,3,3,7,3, NULL, NULL, 'Firmenich contrato a Privi con condicion de pago contado sin cuotas'),
    (7,4,4,NULL,NULL,44,4, 'Firmenich contrato a Perfumer''s Aprentice con envio a Chile'),
    (8,4,4,4,4, NULL, NULL, 'Firmenich contrato a Perfumer''s Aprentice con condicion de pago aplazado con 2 cuotas'),
    (9,5,1,NULL,NULL,89,1, 'Aromex contrato a Keva con envio a India'),   
    (10,5,1,5,1, NULL, NULL, 'Aromex contrato a Keva con condicion de pago contado sin cuotas'),
    (11,6,3,NULL,NULL,126,3, 'Aromex contrato a Privi con envio a Mexico'),   
    (12,6,3,7,3, NULL, NULL, 'Aromex contrato a Privi con condicion de pago contado sin cuotas'),         
    (13,7,2,NULL,NULL,4,2, 'Sephora contrato a essence con envio a Alemania'),   
    (14,7,2,6,2, NULL, NULL, 'Sephora contrato a essence con condicion de pago contado sin cuotas'),   
    (15,8,2,NULL,NULL,14,2, 'fourchem contrato a essence con envio a Australia'),   
    (16,8,2,2,2, NULL, NULL, 'Sephora contrato a essence con condicion de pago aplazado con 2 cuotas'),    
    (17,9,4,NULL,NULL,14,4, 'fourchem contrato a essence con envio a Australia'),   
    (18,9,4,8,4, NULL, NULL, 'Sephora contrato a essence con condicion de pago contado con sin cuotas');


INSERT INTO KMR_Ing_Contrato
    (id,id_contrato,id_emp_prov,id_ing_ifra)
    VALUES 
	(1,1,1,131766739),
	(2,1,1,14352615),
    (3,2,2,80546),
    (5,3,3,18479577),
    (6,4,4,111273),
    (7,4,4,542461),
    (8,5,1,14352615),
    (9,5,1,93049),
    (10,6,3,28897214),
    (11,7,2,106241),
    (12,7,2,121335),
    (13,8,2,121335),
    (14,9,4,504632);
    
INSERT INTO KMR_EP_P
    VALUES
	(1, 155),
	(1, 89),
	(1, 147),
	(1, 126),
	(1, 91),
	(2, 44),
	(2, 4),
	(2, 68),
    (3, 126),
    (3, 89),
    (3, 14),
    (4, 126),
    (4, 91),
    (4, 4),
    (5, 14),
    (5, 91);
