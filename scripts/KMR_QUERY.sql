SELECT if.cas_number, ic.id as ingrediente_id
FROM kmr_ing_contrato ic
JOIN kmr_ifra_ingrediente if ON if.cas_number = ic.id_ing_ifra AND if.id_emp_prov = 1 

UNION

SELECT io.ipc, ic.id as ingrediente_id
FROM kmr_ing_contrato ic
JOIN kmr_ingrediente_otros io ON io.ipc = ic.id_ing_otros AND io.id_emp_prov =  1
