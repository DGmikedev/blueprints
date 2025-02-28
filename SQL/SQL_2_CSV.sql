/* MY_SQL */
SELECT id, nombre, edad
FROM usuarios
INTO OUTFILE 'C:/Users/PC_PERSONAL_NAME/Documents/datos_usuarios.csv'
FIELDS TERMINATED BY ','  -- Separador de columnas
ENCLOSED BY '"'           -- Opcional, para envolver los valores en comillas dobles
LINES TERMINATED BY '\n'; -- Fin de línea

/* POSTGRESQL */
COPY (SELECT id, nombre, edad FROM usuarios)
TO 'C:/Users/PC_PERSONAL_NAME/Documents/datos_usuarios.csv'
WITH CSV HEADER;


-- > PARA LA LINEA DE COMANDOS DEL CLIENTE
\COPY (SELECT id, nombre, edad FROM usuarios) TO 'C:/Users/PC_PERSONAL_NAME/Documents/datos_usuarios.csv' WITH CSV HEADER;


/* SQL SERVER */

bcp "SELECT id, nombre, edad FROM usuarios" queryout "C:\ruta\a\tu\archivo.csv" -c -t, -S nombre_servidor -U usuario -P contraseña

-- >  -c: Especifica que los datos se exporten como caracteres.
-- >  -t,: Define la coma (,) como el delimitador entre los campos.
-- >  queryout: Indica que se va a exportar el resultado de la consulta a un archivo.
-- >  -S: El nombre del servidor de base de datos.
-- >  -U: El nombre de usuario para acceder a la base de datos.
-- >  -P: La contraseña del usuario

/*  ORACLE  */

SET COLSEP ,                    -- Establece la coma como separador de columnas
SET PAGESIZE 0                  -- No muestra la paginación
SET LINESIZE 32767              -- Ajusta el tamaño de la línea para evitar cortes en los datos
SET FEEDBACK OFF                -- Desactiva la retroalimentación
SET TRIMSPOOL ON                -- Elimina espacios adicionales al final de los datos
  
SPOOL /ruta/a/tu/archivo.csv    --Abre el archivo donde se escribirán los resultados.

SELECT id || ',' || nombre || ',' || edad
FROM usuarios;

SPOOL OFF;
