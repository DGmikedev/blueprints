/*
    ESTE PROCEDIMIENTO TRAE LOS CLIENTES SEGÚN LAS OPCIONRD ENVIADAS:

    id = '0'  ó  '1,2,3,4 .... n' OBTIENE TODOS O LOS REGISTROS SELECCIONADOS

    orden = 'asc' Ó 'desc'        ASCENDENTE O DESCENDENTE EL ORDEN DE LOS REGISTROS

    CREADO: 11/11/2025

    USR_CREADOR: dgmike.dev@gmail.com

*/


DELIMITER $$

CREATE PROCEDURE get_clientes(
    IN id VARCHAR(100), 
    IN orden VARCHAR(4)
)

BEGIN
    SELECT * FROM clientes
    WHERE (id = '0') OR FIND_IN_SET(codigocliente, id)
    ORDER BY
        CASE WHEN orden = 'asc'  THEN codigocliente end ASC,
        CASE WHEN orden = 'desc' THEN codigocliente end DESC;
END $$

DELIMITER ;
