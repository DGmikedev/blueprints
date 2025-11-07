SELECT JSON_OBJECT(
'codigo', codigocliente,
'empresa', empresa,
'direcion', direccion,
'poblacion', poblacion,
'telefono', telefono,
'responsable',responsable
) AS salida 
FROM clientes; 
