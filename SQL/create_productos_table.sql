/*
  ## Se crea tabla de productos  ##
    fecha de creación: 04/11/2025
    proyecto: tiendas principales
    desarrolló: MDG - dgmike.dev@gmail.com
    version Mysql 5.8
    [php.init already driver]
*/

create table productos (
id_articulo INT AUTO_INCREMENT PRIMARY KEY,
codigoarticulo  VARCHAR(10),
seccion VARCHAR(50), 
nombre_articulo	VARCHAR(100),
precio  FLOAT,
fecha DATE,
imporrtado BOOLEAN,
pais_origen VARCHAR(100),
foto BINARY
);

insert into productos(codigoarticulo,seccion,nombre_articulo,precio,fecha,imporrtado,pais_origen,foto) 
values (
    "ART1", 
    "CAFERTERIA",
    "CAFE COLOMBIANO",
    12.50,
    "2025-11-04",
    false,
    "MEXICO",
    ""
);

insert into productos(codigoarticulo,seccion,nombre_articulo,precio,fecha,imporrtado,pais_origen,foto) values ("AR01","FERRETERÍA","DESTORNILLADOR",6.63,        "2000-10-22",false, "ESPAÑA", "");
insert into productos(codigoarticulo,seccion,nombre_articulo,precio,fecha,imporrtado,pais_origen,foto) values ("AR02","CONFECCIÓN","TRAJE CABALLERO",284.58,     "2002-03-11",true, "ITALIA", "");
insert into productos(codigoarticulo,seccion,nombre_articulo,precio,fecha,imporrtado,pais_origen,foto) values ("AR03","JUGUETERÍA","COCHE TELEDIRIGIDO",159.45,  "2002-05-26",true, "MARRUECOS", "");
insert into productos(codigoarticulo,seccion,nombre_articulo,precio,fecha,imporrtado,pais_origen,foto) values ("AR04","DEPORTES","RAQUETA TENIS",93.47,          "2000-03-20",true, "USA", "");
insert into productos(codigoarticulo,seccion,nombre_articulo,precio,fecha,imporrtado,pais_origen,foto) values ("AR06","DEPORTES","MANCUERNAS",60.00,             "2000-09-13",true, "USA", "");
insert into productos(codigoarticulo,seccion,nombre_articulo,precio,fecha,imporrtado,pais_origen,foto) values ("AR07","CONFECCIÓN","SERRUCHO",30.20,             "2001-03-23",true, "FRANCIA", "");
insert into productos(codigoarticulo,seccion,nombre_articulo,precio,fecha,imporrtado,pais_origen,foto) values ("AR08","JUGUETERÍA","CORREPASILLOS",103.34,       "2000-04-11",true, "JAPÓN", "");
insert into productos(codigoarticulo,seccion,nombre_articulo,precio,fecha,imporrtado,pais_origen,foto) values ("AR09","CONFECCIÓN","PANTALÓN SEÑORA",174.23,     "2000-01-10",true, "MARRUECOS", "");
insert into productos(codigoarticulo,seccion,nombre_articulo,precio,fecha,imporrtado,pais_origen,foto) values ("AR10","JUGUETERÍA","CONSOLA VIDEO",442.54,       "2002-09-24",true, "USA", "");
insert into productos(codigoarticulo,seccion,nombre_articulo,precio,fecha,imporrtado,pais_origen,foto) values ("AR11","CERÁMICA","TUBOS",168.43,                 "2000-02-04",true, "CHINA", "");
insert into productos(codigoarticulo,seccion,nombre_articulo,precio,fecha,imporrtado,pais_origen,foto) values ("AR12","FERRETERÍA","LLAVE INGLESA",24.40,        "2001-05-23",true, "USA", "");
insert into productos(codigoarticulo,seccion,nombre_articulo,precio,fecha,imporrtado,pais_origen,foto) values ("AR13","CONFECCIÓN","CAMISA CABALLERO",67.13,     "2002-08-11",false, "ESPAÑA", "");
insert into productos(codigoarticulo,seccion,nombre_articulo,precio,fecha,imporrtado,pais_origen,foto) values ("AR14","JUGUETERÍA","TREN ELÉCTRICO",1505.38,     "2001-07-03",true, "JAPÓN", "");
insert into productos(codigoarticulo,seccion,nombre_articulo,precio,fecha,imporrtado,pais_origen,foto) values ("AR15","CERÁMICA","PLATO DECORATIVO",54.09,       "2000-06-07",true, "CHINA", "");
insert into productos(codigoarticulo,seccion,nombre_articulo,precio,fecha,imporrtado,pais_origen,foto) values ("AR16","FERRETERÍA","ALICATES",6.74,              "2000-04-17",true, "ITALIA", "");
insert into productos(codigoarticulo,seccion,nombre_articulo,precio,fecha,imporrtado,pais_origen,foto) values ("AR17","JUGUETERÍA","MUÑECA ANDADORA",105.06,     "2001-01-04",false, "ESPAÑA", "");
insert into productos(codigoarticulo,seccion,nombre_articulo,precio,fecha,imporrtado,pais_origen,foto) values ("AR18","DEPORTES","PISTOLA OLÍMPICA",46.73,       "2001-02-02",true, "SUECIA", "");
insert into productos(codigoarticulo,seccion,nombre_articulo,precio,fecha,imporrtado,pais_origen,foto) values ("AR19","CONFECCIÓN","BLUSA SRA.",101.06,          "2000-03-18",true, "CHINA", "");
insert into productos(codigoarticulo,seccion,nombre_articulo,precio,fecha,imporrtado,pais_origen,foto) values ("AR20","CERÁMICA","JUEGO DE TE",43.27,            "2001-01-15",true, "CHINA", "");
insert into productos(codigoarticulo,seccion,nombre_articulo,precio,fecha,imporrtado,pais_origen,foto) values ("AR21","CERÁMICA","CENICERO",19.75,               "2001-07-02",true, "JAPÓN", "");
insert into productos(codigoarticulo,seccion,nombre_articulo,precio,fecha,imporrtado,pais_origen,foto) values ("AR22","FERRETERÍA","MARTILLO",11.40,             "2001-09-04",false, "ESPAÑA", "");
insert into productos(codigoarticulo,seccion,nombre_articulo,precio,fecha,imporrtado,pais_origen,foto) values ("AR23","CONFECCIÓN","CAZADORA PIEL",522.69,       "2001-07-10",true, "ITALIA", "");
insert into productos(codigoarticulo,seccion,nombre_articulo,precio,fecha,imporrtado,pais_origen,foto) values ("AR24","DEPORTES","BALÓN RUGBY",111.64,           "2000-11-11",true, "USA", "");
insert into productos(codigoarticulo,seccion,nombre_articulo,precio,fecha,imporrtado,pais_origen,foto) values ("AR25","DEPORTES","BALÓN BALONCESTO",75.27,       "2001-06-25",true, "JAPÓN", "");
insert into productos(codigoarticulo,seccion,nombre_articulo,precio,fecha,imporrtado,pais_origen,foto) values ("AR26","JUGUETERÍA","FUERTE DE SOLDADOS",143.70,  "2000-11-25",true, "JAPÓN", "");
insert into productos(codigoarticulo,seccion,nombre_articulo,precio,fecha,imporrtado,pais_origen,foto) values ("AR27","CONFECCIÓN","ABRIGO CABALLERO",500000.00, "2002-04-05",true, "ITALIA", "");
insert into productos(codigoarticulo,seccion,nombre_articulo,precio,fecha,imporrtado,pais_origen,foto) values ("AR28","DEPORTES","BALÓN FÚTBOL",43.91,           "2002-07-04",false, "ESPAÑA", "");
insert into productos(codigoarticulo,seccion,nombre_articulo,precio,fecha,imporrtado,pais_origen,foto) values ("AR29","CONFECCIÓN","ABRIGO SRA",360.07,          "2001-05-03",true, "MARRUECOS", "");
insert into productos(codigoarticulo,seccion,nombre_articulo,precio,fecha,imporrtado,pais_origen,foto) values ("AR30","FERRETERÍA","DESTORNILLADOR",9.06,        "2002-02-20",true, "FRANCIA", "");
insert into productos(codigoarticulo,seccion,nombre_articulo,precio,fecha,imporrtado,pais_origen,foto) values ("AR31","JUGUETERÍA","PISTOLA CON SONIDOS",57.25,  "2001-04-15",false, "ESPAÑA", "");
insert into productos(codigoarticulo,seccion,nombre_articulo,precio,fecha,imporrtado,pais_origen,foto) values ("AR32","DEPORTES","CRONÓMETRO",439.18,            "2002-01-03",true, "USA", "");
insert into productos(codigoarticulo,seccion,nombre_articulo,precio,fecha,imporrtado,pais_origen,foto) values ("AR33","CERÁMICA","MACETA",29.04,                 "2000-02-23",false, "ESPAÑA", "");
insert into productos(codigoarticulo,seccion,nombre_articulo,precio,fecha,imporrtado,pais_origen,foto) values ("AR34","OFICINA","PIE DE LÁMPARA",39.76,          "2001-05-27",true, "TURQUÍA", "");
insert into productos(codigoarticulo,seccion,nombre_articulo,precio,fecha,imporrtado,pais_origen,foto) values ("AR35","FERRETERÍA","LIMA GRANDE",22.07,          "2002-08-10",false, "ESPAÑA", "");
insert into productos(codigoarticulo,seccion,nombre_articulo,precio,fecha,imporrtado,pais_origen,foto) values ("AR36","FERRETERÍA","JUEGO DE BROCAS",15.10,      "2002-07-04",true, "TAIWÁN", "");
insert into productos(codigoarticulo,seccion,nombre_articulo,precio,fecha,imporrtado,pais_origen,foto) values ("AR37","CONFECCIÓN","CINTURÓN DE PIEL",4.33,      "2002-05-12",false, "ESPAÑA", "");
insert into productos(codigoarticulo,seccion,nombre_articulo,precio,fecha,imporrtado,pais_origen,foto) values ("AR38","DEPORTES","CAÑA DE PESCA",270.00,         "2000-02-14",true, "USA", "");
insert into productos(codigoarticulo,seccion,nombre_articulo,precio,fecha,imporrtado,pais_origen,foto) values ("AR39","CERÁMICA","JARRA CHINA",127.77,           "2002-09-02",true, "CHINA", "");
insert into productos(codigoarticulo,seccion,nombre_articulo,precio,fecha,imporrtado,pais_origen,foto) values ("AR40","DEPORTES","BOTA ALPINISMO",144.00,        "2002-05-05",false, "ESPAÑA", "");
insert into productos(codigoarticulo,seccion,nombre_articulo,precio,fecha,imporrtado,pais_origen,foto) values ("AR41","DEPORTES","PALAS DE PING PONG",21.60,     "2002-02-02",false, "ESPAÑA", "");
