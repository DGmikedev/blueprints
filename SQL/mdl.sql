CREATE TABLE USER(
  id int auto_increment primary key,
  activated bool NOT NULL,
  name varchar(30) NOT NULL,
  middle_name varchar(30) NOT NULL,
  last_name varchar(100) NOT NULL,
  age tinyint(3) NOT NULL,
  date_born date NOT NULL,
  date_sign date NOT NULL,
  mail varchar(150) NOT NULL
  )
  
  
  
  
 /*        
  DATOS NUMÉRICOS 
             DESDE                      HASTA
  BIT	       1         	                0
  TINYINT	   0	                        255
  SMALLINT	-32.768	                    32.767
  INT	      -2.147.483.648	            2.147.483.647
  BIGINT	  -9.223.372.036.854.775.808	9.223.372.036.854.775.807
  DECIMAL	  -10³⁸ + 1                  	10³⁸ – 1
  FLOAT	    -1.79E+308	                1.79E+308
  REAL	    -3.40E+38	                  3.40E+38


FECHA - HORA
DATE	      Almacena la fecha en el formato YYYY-MM-DD.
TIME	      Almacena la hora en el formato HH:MM:SS.
DATETIME	  Combina fecha y hora en el formato YYYY-MM-DD HH:MM:SS.
TIMESTAMP 	Almacena el número de segundos desde el Unix Epoch (1970).
YEAR	      Almacena el año en formato de 2 o 4 dígitos (e.g., 1970)

TIPOS DE DATO DE CADENA

CHAR	        Longitud fija, con un máximo de 8.000 caracteres.
VARCHAR	      Longitud variable, con un máximo de 8.000 caracteres.
VARCHAR(MAX)	Longitud variable, permite almacenar grandes cadenas de texto (no en MySQL).
TEXT	        Longitud variable, con un tamaño máximo de 2 GB.





*/
