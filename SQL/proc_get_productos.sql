/*
 ESTE PROCEDIMIENTO TRAE LOS PRODUCTOS SEGÚN LAS OPCIONRD ENVIADAS:

    cod = '0'  ó  '1,2,3,4 .... n' OBTIENE TODOS O LOS REGISTROS SELECCIONADOS

    op_order = 'asc' Ó 'desc'      ASCENDENTE O DESCENDENTE EL ORDEN DE LOS REGISTROS

    CREADO: 11/11/2025

    USR_CREADOR: dgmike.dev@gmail.com
*/

DELIMITER $$

CREATE PROCEDURE get_productos_by_code(
    IN cod  VARCHAR(100),
    IN op_order VARCHAR(4)
)
BEGIN

    SELECT * FROM productos 
    WHERE (cod = '0') OR FIND_IN_SET(id_articulo, cod)
    ORDER BY 
        CASE WHEN op_order = 'asc' THEN id_articulo  END ASC,
        CASE WHEN op_order = 'desc' THEN id_articulo END DESC;
END $$

DELIMITER ; 
