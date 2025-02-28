/* DEMO-SQL->JSON */

SELECT 
		JSON_OBJECT(
		'id_empleado', matricula, 
		'nombre', nombre, 
		'apellido', apellido, 
		'mail', 'e-mail', 
		'f_ingreso', f_ingreso, 
		'dep_inscripto', carreras_idcarrera ) AS datos_user 
	FROM alumnos;