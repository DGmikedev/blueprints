/*
  ## Se crea tabla de clinetes codigocliente es tomado como id principal ##
    fecha de creación: 04/11/2025
    proyecto: tiendas principales
    desarrolló: MDG - dgmike.dev@gmail.com
    version Mysql 5.8
    [php.init already driver]
*/
CREATE TABLE clientes (
codigocliente INT AUTO_INCREMENT PRIMARY KEY,
empresa VARCHAR(50),
direccion VARCHAR(50),
poblacion VARCHAR(50),
telefono VARCHAR(50),
responsable VARCHAR(50),
historial  VARCHAR(200)
);

SELECT * FROM clientes

INSERT INTO clientes ( empresa,direccion,poblacion,telefono,responsable,historial ) VALUES 
(
	"Empresa No1",
	"Calle falsa 123",
	"Poblacion Oriente",
	"123456789",
	"Jacinto Moreno",
	"Buenas practicas, exzcelente servicio"
); 



INSERT INTO clientes ( empresa,direccion,poblacion,telefono,responsable,historial ) VALUES ("BELTRÁN E HIJOS","LAS FUENTES 78","MADRID","914456435","ANGEL MARTÍNEZ"," no aplica");
INSERT INTO clientes ( empresa,direccion,poblacion,telefono,responsable,historial ) VALUES ("LA MODERNA","LA PALOMA 123","OVIEDO","985323434","JUAN GARCÍA"," no aplica");
INSERT INTO clientes ( empresa,direccion,poblacion,telefono,responsable,historial ) VALUES ("EL ESPAÑOLITO","MOTORES 34","BARCELONA","934565343","ANA FERNÁNDEZ"," no aplica");
INSERT INTO clientes ( empresa,direccion,poblacion,telefono,responsable,historial ) VALUES ("EXPORTASA","VALLECAS 34","MADRID","913452378","ELVIRA GÓMEZ"," no aplica");
INSERT INTO clientes ( empresa,direccion,poblacion,telefono,responsable,historial ) VALUES ("CONFECCIONES AMPARO","LOS MOROS 23","GIJÓN","985754332","LUÍS ÁLVAREZ"," no aplica");
INSERT INTO clientes ( empresa,direccion,poblacion,telefono,responsable,historial ) VALUES ("LA CASA DEL JUGUETE","AMÉRICA 45","MADRID","912649987","ELÍAS PÉREZ"," no aplica");
INSERT INTO clientes ( empresa,direccion,poblacion,telefono,responsable,historial ) VALUES ("JUGUETERÍA SUÁREZ","PARIS 123","BARCELONA","933457866","JUAN GARCÍA"," no aplica");
INSERT INTO clientes ( empresa,direccion,poblacion,telefono,responsable,historial ) VALUES ("ALMACÉN POPULAR","LAS FUENTES 124","BILBAO","942347127","JOSÉ ÁLVAREZ"," no aplica");
INSERT INTO clientes ( empresa,direccion,poblacion,telefono,responsable,historial ) VALUES ("FERETERÍA EL CLAVO","PASEO DE ÁLAMOS 78","MADRID","914354866","MANUEL MENÉNDEZ"," no aplica");
INSERT INTO clientes ( empresa,direccion,poblacion,telefono,responsable,historial ) VALUES ("JUGUETES MARTÍNEZ","VIA LAYETANA 245","BARCELONA","936628554","FRANCISCO CUEVAS"," no aplica");
INSERT INTO clientes ( empresa,direccion,poblacion,telefono,responsable,historial ) VALUES ("FERNÁNDEZ SL","PASEO DEL MAR 45","SANTANDER","942049586","ELISA COLLADO"," no aplica");
INSERT INTO clientes ( empresa,direccion,poblacion,telefono,responsable,historial ) VALUES ("CONFECCIONES ARTÍMEZ","GENERAL PERÓN 45","A CORUÑA","981345239","ESTEBAN PASCUAL"," no aplica");
INSERT INTO clientes ( empresa,direccion,poblacion,telefono,responsable,historial ) VALUES ("DEPORTES GARCÍA","GUZMÁN EL BUENO 45","MADRID","913299475","ANA JIMÉNEZ"," no aplica");
INSERT INTO clientes ( empresa,direccion,poblacion,telefono,responsable,historial ) VALUES ("EXCLUSIVAS FERNÁNDEZ","LLOBREGAT 250","BARCELONA","939558365","LUISA FERNÁNDEZ"," no aplica");
INSERT INTO clientes ( empresa,direccion,poblacion,telefono,responsable,historial ) VALUES ("DEPORTES MORÁN","AUTONOMÍA 45","LUGO","982986944","JOSÉ MANZANO"," no aplica");
INSERT INTO clientes ( empresa,direccion,poblacion,telefono,responsable,historial ) VALUES ("BAZAR FRANCISCO","CARMEN 45","ZAMORA","980495288","CARLOS BELTRÁN"," no aplica");
INSERT INTO clientes ( empresa,direccion,poblacion,telefono,responsable,historial ) VALUES ("JUGUETES LA SONRISA","LA BAÑEZA 67","LEÓN","987945368","FAUSTINO PÉREZ"," no aplica");
INSERT INTO clientes ( empresa,direccion,poblacion,telefono,responsable,historial ) VALUES ("CONFECCIONES GALÁN","FUENCARRAL 78","MADRID","913859234","JUAN GARCÍA"," no aplica");
INSERT INTO clientes ( empresa,direccion,poblacion,telefono,responsable,historial ) VALUES ("LA CURTIDORA","OLIVARES 3","MÁLAGA","953756259","MARÍA GÓMEZ"," no aplica");
INSERT INTO clientes ( empresa,direccion,poblacion,telefono,responsable,historial ) VALUES ("LÍNEA JOVEN","SIERPES 78","SEVILLA","953452567","ASUNCIÓN SALADO"," no aplica");
INSERT INTO clientes ( empresa,direccion,poblacion,telefono,responsable,historial ) VALUES ("BAZAR EL BARAT","DIAGONAL 56","BARCELONA","936692866","ELISA DAPENA"," no aplica");
INSERT INTO clientes ( empresa,direccion,poblacion,telefono,responsable,historial ) VALUES ("EL PALACIO DE LA MODA","ORTEGA Y GASSET 129","MADRID","927785235","LAURA CARRASCO"," no aplica");
INSERT INTO clientes ( empresa,direccion,poblacion,telefono,responsable,historial ) VALUES ("SÁEZ Y CÍA","INFANTA MERCEDS 23","SEVILLA","954869234","MANUEL GUERRA"," no aplica");
INSERT INTO clientes ( empresa,direccion,poblacion,telefono,responsable,historial ) VALUES ("DEPORTES EL MADRILEÑO","CASTILLA 345","ZARAGOZA","976388934","CARLOS GONZÁLEZ"," no aplica");
INSERT INTO clientes ( empresa,direccion,poblacion,telefono,responsable,historial ) VALUES ("FERRETERÍA LA ESCOBA","ORENSE 7","MADRID","918459346","JOSÉ GARCÍA"," no aplica");
INSERT INTO clientes ( empresa,direccion,poblacion,telefono,responsable,historial ) VALUES ("JUGUETES EL BARATO","VÍA AUGUSTA 245","BARCELONA","933486984","ELVIRA IGLESIAS"," no aplica");
INSERT INTO clientes ( empresa,direccion,poblacion,telefono,responsable,historial ) VALUES ("CONFECCIONES HERMINIA","CORRIDA 345","GIJÓN","985597315","ABEL GONZÁLEZ"," no aplica");
INSERT INTO clientes ( empresa,direccion,poblacion,telefono,responsable,historial ) VALUES ("BAZAR EL ARGENTINO","ATOCHA 55","MADRID","912495973","ADRIÁN ÁLVAREZ"," no aplica");
INSERT INTO clientes ( empresa,direccion,poblacion,telefono,responsable,historial ) VALUES ("LA TIENDA ELEGANTE","EL COMENDADOR 67","ZARAGOZA","975694035","JOSÉ PASCUAL"," no aplica");
INSERT INTO clientes ( empresa,direccion,poblacion,telefono,responsable,historial ) VALUES ("DEPORTES NAUTICOS GARCÍA","JUAN FERNÁNDEZ 89","ÁVILA","920268648","JUAN CONRADO"," no aplica");
INSERT INTO clientes ( empresa,direccion,poblacion,telefono,responsable,historial ) VALUES ("CONFECCIONES RUIZ","LLOBREGAT 345","BARCELONA","934587615","CARLOS SANZ"," no aplica");
INSERT INTO clientes ( empresa,direccion,poblacion,telefono,responsable,historial ) VALUES ("BAZAR LA FARAONA","CASTILLA Y LEÓN 34","MADRID","915483627","ANGEL SANTAMARÍA"," no aplica");
INSERT INTO clientes ( empresa,direccion,poblacion,telefono,responsable,historial ) VALUES ("FERRETERÍA EL MARTILLO","CASTELLANOS 205","SALAMANCA","923548965","JOAQUÍN FERNANDEZ"," no aplica");
INSERT INTO clientes ( empresa,direccion,poblacion,telefono,responsable,historial ) VALUES ("JUGUETES EDUCATIVOS SANZ","ORENSE 89","MADRID","916872354","PEDRO IGLESIAS"," no aplica");
INSERT INTO clientes ( empresa,direccion,poblacion,telefono,responsable,historial ) VALUES ("ALMACENES FERNANDEZ","ANTÓN 67","TERUEL","978564025","MARIA ARDANZA"," no aplica");
INSERT INTO clientes ( empresa,direccion,poblacion,telefono,responsable,historial ) VALUES ("CONFECCIONES MÓNICA","MOTORES 67","BARCELONA","935681245","PEDRO SERRANO"," no aplica");
INSERT INTO clientes ( empresa,direccion,poblacion,telefono,responsable,historial ) VALUES ("FERRETERÍA LIMA","VALLECAS 45","MADRID","913532785","LUIS GARCÍA"," no aplica");
INSERT INTO clientes ( empresa,direccion,poblacion,telefono,responsable,historial ) VALUES ("DEPORTES EL BRASILEÑO","ABEL MARTÍNEZ 67","SALAMANCA","921548762","CARLOS GÓMEZ"," no aplica");
