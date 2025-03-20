use std::fs::File;
use std::io::Error;
mod editor_txt;

fn main() {

let datos = [
     vec!["banco_central", "usuarios_registrados", "800"], 
     vec!["Columna 2", "NAM", "0", "0"], 
     vec!["Columna 3", "ADD", "0", "0"], 
     vec!["Columna 4", "DEC", "10", "2"], 
     vec!["Columna 5", "SMA", "0", "0"], 
     vec!["Columna 6", "VAR", "0", "0"], 
     vec!["Columna 7", "BIN", "26", "0"], 
     vec!["Columna 8", "DAT", "0", "0"]
 ];
/*
    let path: String =  format!("{}/{}/", datos[0][0], datos[0][1]); 
    let name: String =  format!("{}.sql", datos[0][1]);
    let full_name: String = format!("{path}{name}");
    let text: String = String::from("Gloucester: Yo hago el mal, y no soy el primero en empezar a regañar. Las maldades secretas que preparo, las pongo a cuenta de otros, como culpa suya. A Clarence, a quien, desde luego, he puesto yo en la tiniebla, ahora le lamento delante de muchos simples bobos; esto es, ante Hastings, Stanley y Buckingham; y digo que son la Reina y sus aliados quienes mueven al Rey contra mi hermano el Duque. Ahora se lo creen; y a la vez me dejan vengarme de Rivers, Vaughan y Grey: pero entonces suspiro y, con un trozo de la Escritura, les digo que Dios nos manda hacer bien por mal, revistiendo así mi desnuda villanía con retazos viejos robados de la Santa Biblia; parezco un santo cuando más hago el diablo. ");

// 1) =>  Se crea el directorio
    let path_directory: bool =  editor_txt::create_directory(path.clone());

    // RETURN SI FALLA LA CREACIÓN DEL DIRECTORIO
    if !&path_directory { println!("No se pudo crear el directorio indicado: {}", &path) }

// 2) => se crea el archvio
    let mut document: Result<File, Error> = editor_txt::create_file(path.clone(), name);

    // RETURN SI FALLA LA CREACIÓN DEL ARCHIVO
    match &document{
        Ok(v) => println!("Archivo creado"),
        _ => println!("Error al crear el archivo indicado: {}", &full_name),
    }

    // iNSERTAR REGNGLONES
    // for i in 0..1{
    //     let _ = editor_txt::insert_txt_by_ln(full_name.clone(),text.clone());
    // }
*/
    println!("1 -> {}",seeder::get_int_usize(1));
    println!("2 -> {}",seeder::get_int_usize(2));
    println!("3 -> {}",seeder::get_int_usize(3));
    println!("4 -> {}",seeder::get_int_usize(4));
    println!("5 -> {}",seeder::get_int_usize(5));
    println!("6 -> {}",seeder::get_double_or_float(0));
    println!("7 -> {}",seeder::get_double_or_float(1));
    println!("8 -> {}",seeder::get_num_dec(10, 5));
    println!("8 -> {}",seeder::get_num_dec(38, 5));

}


/*
 <option value="N/A">SELECT</option>
            <option disabled   >-- Fast data ----</option>
            <option value="NAM">NAME</option>
            <option value="NMF">NAME FULL</option>
            <option value="APP">MIDDLE-LAST NAME</option>
            <option value="ADD">ADDRESS</option>
            <option value="MAI">EMAIL</option>
            <option value="PHO">PHONE</option>
            <option value="CON">COUNTRY</option>
            <option value="CNT">CONTINENT</option>
            <option value="SLF">SELL FLOAT</option>
            <option value="SLI">SELL INT</option>
            <option disable>---------------------</option>
            <option value="TIN">TINYINT</option>
            <option value="SMA">SMALLINT</option>
            <option value="MED">MEDIUMINT</option>
            <option value="INT">INT</option>
            |<option value="BIG">BIGINT</option>
            <option disable>---------------------</option>

            <option value="FLO">FLOAT     ~-3.4E+38    ~3.4E+38    3.141592 -12345.6789
            <option value="DOU">DOUBLE    ~-1.7E+308   ~1.7E+308 
            <option value="DEC">DECIMAL (P, S)</option> DECIMAL(10, 0))
            <option value="NUM">NUMERIC (P, S)</option> NUMERIC(38, 0))
	        <option disabled   >-------------</option>

            
            <option value="CHA">CHAR (N)</option>
            <option value="VAR">VARCHAR (N)</option>
            <option value="TEX">TEXT</option>
            <option disabled   >-------------</option>



            <option value="DAT">DATE</option>
            <option value="TIM">TIME</option>
            <option value="DAT">DATETIME</option>
            <option value="TMS">TIMESTAMP</option>
            <option value="YEA">YEAR</option>
            <option disabled   >-------------</option>

            <option value="BIN">BINARY (N)</option>
            <option value="VRB">VARBINARY (N)</option>
            <option disabled   >-------------</option>


            <option value="BOO">BOOLEAN</option>
            <option value="JSO">JSON</option>
            <option value="ENU">ENUM</option>
            <option value="SET">SET</option>

*/
mod seeder {

    use rand::{ self, Rng };   //{self, Rng};

    // use std::env;
    // use std::process::Command;
    // use std::process::exit;
    
     /// RETORNA UN VALOR DEL TIPO STRING <br>
    /// QUE REPRESENTA UN VALOR DEL TIPO NUMERIC O DECIMAL <br>
    /// ```
    /// OPCIONES
    /// opc- num  cantidad de numeros de la cifra entera
    /// opc- dec  cantidad de numeros de la cifra decimal
    /// ```
    /// 

    pub fn get_num_dec(num: usize, dec: usize)->String{

        let mut rng: rand::prelude::ThreadRng = rand::rng();
        let mut str = format!("");
        let mut str_dec = format!("");
        for i in 0 ..num{
            str.push_str( &rng.random_range(0 .. 9).to_string() )
        }
        for i in 0 ..dec{
            str_dec.push_str( &rng.random_range(0 .. 9).to_string() )
        }
        let resp = format!("{}.{}",str, str_dec);
        resp
    }

    
     /// RETORNA UN VALOR DEL TIPO STRING <br>
    /// QUE REPRESENTA UN VALOR DEL TIPO FLOAT O DOUBLE <br>
    /// DE RANGO DESDE -1.7^308 HASTA 1.7^308
    /// ```
    /// OPCIONES
    /// opc- 0 "FLO">FLOAT    -3.4E+38   3.4E+38
    /// opc- 1 "DOU">DOUBLE   -1.7E+308  1.7E+308
    /// ```
    /// 
    pub fn get_double_or_float(flo_num: usize)->String{

        let mut rng: rand::prelude::ThreadRng = rand::rng();
        match flo_num{
            0 =>{
                let indx = rng.random_range(-3.4f32.powf(37.5)..3.4f32.powf(38.0));
                return indx.to_string()
            },
            1 =>{
                let indx  = rng.random_range(-1.7f64.powf(308.0)..1.7f64.powf(308.0));
                return indx.to_string()
            },
            _ => return format!("x")
        }
    }


    /// RETORNA UN VALOR DEL TIPO STRING <br>
    /// QUE REPRESENTA UN VALOR DEL TIPO INT <br>
    /// DE RANGO DESDE 0 HASTA 2^64 - 2 
    /// ```
    /// OPCIONES
    /// opc- 1 "TIN">TINYINT    1 byte   -128         0  127         max 255
    /// opc- 2 "SMA">SMALLINT   2 byte  -32768        0  32767       max 65535
    /// opc- 3 "MED">MEDIUMINT  3 byte  -8388608      0  8388607     max 16777215
    /// opc- 4 "INT">INT        4 byte  -2147483648   0  2147483647  max 4294967295
    /// opc- 5 "BIG">BIGINT     8 byte  -2^63 	      0  2^63 - 1    max 2^64 - 1
    /// ```
    /// 
    pub fn get_int_usize(tipo: usize)-> String{

        let mut rng: rand::prelude::ThreadRng = rand::rng();
        match tipo{
            1 => { let indx: i32 = rng.random_range(-128..127);  
                   return indx.to_string();    },
            2 => { let indx: i32 = rng.random_range(-32768..32767); 
                   return indx.to_string();    },
            3 => { let indx: i32 = rng.random_range(-8388608..8388607); 
                   return indx.to_string();    },
            4 => { let indx: i64 = rng.random_range(-2147483648..2147483647); 
                   return indx.to_string();    },
            5 => { let indx: i64 = rng.random_range(-2i64.pow(60)..2i64.pow(60) - 2); 
                   return indx.to_string();    },
            _ => format!("x"),
        }
    }


    pub fn get_token(lon: usize, chars_especials: bool )->String{

        let mut clve: String = String::from("");
        let mut rng: rand::prelude::ThreadRng = rand::rng();
        
        let characters: [&str;67] =[
        "A","B","C","D","E","F","G","H","I","J",
        "K","L","M","N","O","P","Q","R","S","T",
        "U","V","W","X","Y","Z","a","b","c","d",
        "e","f","g","h","i","j","k","l","m","n",
        "o","p","q","r","s","t","u","v","w","x",
        "y","z","0","1","2","3","4","5","6","7",
        "8","9","!","@","#","$","%"
        ];
        
        for _i in 0..= lon {
                if chars_especials{
                let indx = rng.random_range(0..=66);
            clve.push_str(characters[indx]);
            }else{
                let indx = rng.random_range(0..=52);
            clve.push_str(characters[indx]);
            }
        }
        clve
        
    }

    pub fn get_name()->String{
        let mut rng: rand::prelude::ThreadRng = rand::rng();
        let mut cadena:String = String::from("");
        let names: Vec<&str> = vec![
                "Juan", "James", "María", "John", "Maria", "Emily", "Juan", "Emily","Daniel", 
                "Carlos", "Michael", "Ana", "Michael", "Ana", "Sarah", "David", "Ashley",
                "Luis", "David", "Laura", "Christopher","Pedro", "Christopher", "Sofía", "Tatiana",
                "Sofia", "Jessica", "Michael", "Jessica", "Matthew", "Laura", "Ashley", "Amanda",
                "Emily", "Sarah", "Miguel", "Joshua", "Carlos", "David", "Isabel", "Megan",
                "Akira", "Mei","Hiroshi","Yumi","Suki","Taro","Jiro","Ling","Kenji","Sakura",
                "Iván","Anastasia","Dmitri","Ekaterina","Alexéi","Natalia","Vladimir","Olga","Mijaíl"
        ];
    
        let random = rng.random_range(0..names.len());
        cadena.push_str(names[random]);
        cadena
    }
    
    pub fn get_mdl_lst_name()->String{
        let mut rng: rand::prelude::ThreadRng = rand::rng();
        let mut cadena:String = String::from("");
        let mdl_lst_name: Vec<&str> = vec![
                "Pérez", "Smith", "García", "Smith", "Rodríguez", "Johnson", "Hernández", "Thompson",
                "Williams", "Martínez", "Brown", "Martínez", "Jones", "González", "Davis","López",
                "Smith", "Miller", "Ramírez", "Martinez", "González", "Davis", "Martínez", "White",
                "Anderson", "Williams", "Rodriguez", "Jiménez", "Taylor", "Pérez", "Anderson", "Miller",
                "Hernández", "Thomas", "Jones", "Jackson", "García", "Jackson", "Sánchez","Lee",
                "Gómez", "Johnson", "López", "Johnson", "López", "Williams", "Martínez", "Brown",
                "Hernández", "Jones", "González", "Davis", "Sánchez", "Miller", "Johnson", "Wilson",
                "Torres", "Garcia", "Pérez", "Garcia", "Díaz", "Martinez", "Brown", "Martinez",
                "Cruz", "Hernandez", "Sánchez", "Thomas", "Mora", "Moore", "Davis", "White", "Vasiliev",
                "Fernández", "Martin", "Ramírez", "Moore", "Ramírez", "Lee", "Wilson", "Clark",
                "Wang","Kim","Li","Zhang","Tanaka","Yamamoto","Lee","Sato","Cho","Park","Morozov",
                "Ivánov","Petrova","Smirnov","Romanov","Kuznetsov","Fedorov","Mikhailov","Alexandrov"
        ];  
        let random = rng.random_range(0..mdl_lst_name.len());
        cadena.push_str(mdl_lst_name[random]);
        cadena
    }

    pub fn get_addres()->String{
        let mut rng: rand::prelude::ThreadRng = rand::rng();
        let mut cadena:String = String::from("");
        let address: Vec<&str> = vec![
                "Avenida Reforma 123, Ciudad de México, CDMX", "123 Main Street, New York, NY","789 Pine Avenue, Chicago, IL",
                "Calle Principal 123, Ciudad de México, México","123 Main St, Anytown, CA, USA","Av. Reforma 456, Guadalajara, México",
                "1234 Elm Street, Los Ángeles, CA", "456 Oak Road, Los Angeles, CA","123 Main St, Los Angeles, CA, USA",
                "456 Oak Ave, Springfield, IL, USA","Calle 5 de Febrero 456, Monterrey, Nuevo León", "789 Pine Ln, Riverside, TX, USA",
                "Carretera Panamericana 789, Guadalajara, Jalisco", "321 Maple Lane, Houston, TX","456 Oak Ave, New York, NY, USA",
                "101 Elm Rd, Greenville, NY, USA","Av. de los Insurgentes Sur 101, Ciudad de México, CDMX","123 Oak Avenue, Houston, TX",
                "654 Cedar Drive, Phoenix, AZ","Calle Hidalgo 789, Monterrey, México","202 Maple Dr, Sunnyvale, WA, USA",
                "987 Birch Street, Miami, FL","789 Pine Ln, Chicago, IL, USA", "Calle Hidalgo 789, Puebla, Puebla", "123 Elm Street, San Francisco, CA",
                "303 Birch St, Pleasantville, FL, USA", "Av. Juárez 101, Puebla, México", "404 Cedar Ave, Harmonyville, MI, USA",
               "Callejón del Sol 345, Monterrey, Nuevo León", "Boulevard Juárez 678, Tijuana, Baja California", "707 Sequoia Dr, Tranquil Town, OR, USA",
                "234 Pinecrest Road, Dallas, TX","101 Elm Rd, Houston, TX, USA","505 Willow Ln, Peaceful Pines, AZ, USA","890 Rosewood Avenue, Austin, TX","202 Maple Dr, Phoenix, AZ, USA",
                "567 Maple Lane, Seattle, WA","Calle Morelos 202, Tijuana, México","606 Redwood Rd, Serenity Springs, CO, USA","8th Avenue 234, New York, NY",
        ]; 
        let random = rng.random_range(0..address.len());
        cadena.push_str(address[random]);
        cadena
    }        
        
    pub fn get_movil()->String{
        let mut rng: rand::prelude::ThreadRng = rand::rng();
        let mut cadena:String = String::from("");
        let movil: Vec<&str> = vec![
            "5551234",     "2125551234",  "525512345678",   "15551234567",
            "3235678",     "3235552345",  "12135551212",    "15559876543",
            "8189012",     "3125553456",  "523398765432",   "15551122334",
            "3322334455",  "7135554567",  "12125551212",    "15554455667",
            "5512345678",  "6025555678",  "528123456789",   "15557788990",
            "7139087",     "3055556789",  "13125551212",    "15552233445",
            "2223344",     "4155557890",  "522221234567",   "15555566778",
            "8182345",     "2145558901",  "17135551212",    "15558899001",
            "6643456",     "2065559012",  "526649876543",   "15553344556",
            "2125678",     "5125550123",  "16025551212",    "15556677889"
        ];
        let random = rng.random_range(0..movil.len());
        cadena.push_str(movil[random]);
        cadena
    }  
    
    pub fn get_email()->String{
        let mut rng: rand::prelude::ThreadRng = rand::rng();
        let mut cadena:String = String::from("");
        let email: Vec<&str> = vec![
                "juan.perez@email.com", "maria.garcia@example.com", "james.smith@email.com", "john.smith@example.com",
                "maria.rodriguez@email.com", "juan.hernandez@example.com", "emily.johnson@email.com", "emily.williams@example.com",
                "carlos.martinez@email.com", "ana.martinez@example.com", "michael.brown@email.com", "michael.jones@example.com",
                "ana.gonzalez@email.com", "david.smith@example.com", "sarah.davis@email.com", "ashley.miller@example.com",
                "luis.ramirez@email.com", "laura.gonzalez@example.com", "david.martinez@email.com", "christopher.davis@example.com",
                "sofia.martinez@email.com", "michael.williams@example.com", "jessica.anderson@email.com", "jessica.rodriguez@example.com",
                "pedro.jimenez@email.com", "sofia.perez@example.com", "christopher.taylor@email.com", "matthew.anderson@example.com",
                "laura.hernandez@email.com", "emily.jones@example.com", "ashley.thomas@email.com", "sarah.jackson@example.com",
                "miguel.garcia@email.com", "carlos.sanchez@example.com", "joshua.jackson@email.com", "david.thompson@example.com",
                "isabel.lopez@email.com", "daniel.miller@example.com", "megan.white@email.com", "amanda.lee@example.com"
        ];
        let random = rng.random_range(0..email.len());
        cadena.push_str( &( get_token(5, false) + email[random] ) );
        cadena
    }  
    
    pub fn get_full_date(separador: String)->String{
    
        let mut rng: rand::prelude::ThreadRng = rand::rng();
        let mut cadena:String = String::from("");
        let sep = separador;
        let year: Vec<&str> = vec![ "2020", "2021", "2022", "2023", "2024", "2025"];
        let mont: Vec<&str> = vec![ "01","02","03","04","05","06","07","08","09","10","11","12" ];
        let day:  Vec<&str> = vec![ "01","02","03","04","05","06","08","09","10","11","12",
                                    "13","14","15","16","17","18","19","20","21","22","23",
                                    "24","25","26","27","28","29","30" ];
        
        let mut random = rng.random_range(0..year.len());
        cadena.push_str(year[random]);
        cadena.push_str(&sep);
    
        random = rng.random_range(0..mont.len());
        cadena.push_str(mont[random]);
        cadena.push_str(&sep);
    
        if random == 1{
            random = rng.random_range(0..25);    
        }else{
            random = rng.random_range(0..day.len());
        }
        
        cadena.push_str(day[random]);
    
        cadena
    }  
    
    pub fn get_rand_words()->String{
        let mut rng: rand::prelude::ThreadRng = rand::rng();
        let mut cadena:String = String::from("");
        let word: Vec<&str> = vec!["sol", "estrella", "luna", "gato", "perro", "rojo", "flor", 
            "montaña", "playa", "viento", "piedra", "agua", "camino", "noche", "cielo", "amigo", 
            "familia", "feliz", "árbol", "río", "solución", "fuego", "café", "bici", "piedra", 
            "sueño", "música", "carro", "viaje", "paz", "salud", "luz", "libro", "puente", "ciudad", 
            "fútbol", "balón", "guitarra", "verde", "rojo", "amor", "nieve", "hielo", "flor", "viento", 
            "pasión", "sombra", "amigo", "brisa", "cielo", "mar", "risa"];
        let random = rng.random_range(0..word.len());
        cadena.push_str(word[random]);
        cadena
    }


}



























