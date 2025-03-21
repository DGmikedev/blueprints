use std::fs::File;
use std::io::Error;

mod get_fker_data;
mod editor_txt;
mod mkr_fke_data;
mod make_script_sql;

fn main() {

    let datos = [
         vec!["local".to_string(), "users".to_string(), "5".to_string()], 
         vec!["email_verified_at".to_string(), "TMS".to_string(), "0".to_string(), "0".to_string()], 
         vec!["created_at".to_string(), "TMS".to_string(), "0".to_string(), "0".to_string()], 
         vec!["updated_at".to_string(), "TMS".to_string(), "0".to_string(), "0".to_string()], 
         vec!["remember_token".to_string(), "VAR".to_string(), "10".to_string(), "0".to_string()], 
         vec!["name".to_string(), "NMF".to_string(), "255".to_string(), "0".to_string()], 
         vec!["email".to_string(), "MAI".to_string(), "0".to_string(), "0".to_string()], 
         vec!["password".to_string(), "VAR".to_string(), "250".to_string(), "0".to_string()]
     ];

    // Se parsea el numero de rgistro deseados desde los datos de entrada  en la poscicion datos[0][2].  
    let num_de_registros = datos[0][2].
        parse::<usize>().
        expect("Ha ocurrido un error");

    let mut cols_names: Vec<String> = Vec::new();

    // Se recaban los nombres de las columnas para hacer el insert en el paso 3
        for i in 1..datos.len(){ cols_names.push(datos[i][0].clone()) }


     // SE RECIBEN LOS DATOS  Y SE CREAN LOS DIRECTORIOS Y EL ARCHIVO 

        let path: String =  format!("{}/{}/", datos[0][0], datos[0][1]); 
        let name: String =  format!("{}.sql", datos[0][1]);
        let full_name: String = format!("{path}{name}");

    // 1) =>  Se crea el directorio
        let path_directory: bool =  editor_txt::create_directory(path.clone());

        // RETURN SI FALLA LA CREACIÓN DEL DIRECTORIO
        if !&path_directory { println!("No se pudo crear el directorio indicado: {}", &path) }

    // 2) => se crea el archvio
        let document: Result<File, Error> = editor_txt::create_file(path.clone(), name);

        // RETURN SI FALLA LA CREACIÓN DEL ARCHIVO
        match &document{
            Ok(v) => println!("Archivo creado"),
            _ => println!("Error al crear el archivo indicado: {}", &full_name),
        }

    // 3) => Se Inicia el insert

    // Se obtiene el Header - nombre de tabla insert into (columns)
        let header_script = 
        make_script_sql::create_header_insert_script(cols_names, datos[0].clone());
    
    // Se  inserta en el documento
        let _ = 
        editor_txt::insert_txt_by_ln(full_name.clone(),header_script.clone());


    // 4) => Se generan los rows a insertar

        for i in 0..num_de_registros{

            let mut row = format!("(");

            for i in 1..datos.len(){
                row.push_str("\'");
                row.push_str(&get_fker_data::get_fake_data(&datos[i][1], &datos[i][2], &datos[i][3]));
                if i == datos.len() - 1 {
                    row.push_str("\'");
                }else{
                    row.push_str("\',");
                }
            }
            if i == num_de_registros - 1{ 
                row.push_str(");"); 
            }else{
                row.push_str("),");
            }
            let _ = editor_txt::insert_txt_by_ln(full_name.clone(),row.clone());
        }

    /*
    let commando = Command::new("mysql")
            .arg("-u")
            .arg("root")
            .arg("-e")
            .arg(stringtmp)
            .output()
            .expect("Falló al ejecutar el comando");

            // success?
            if commando.status.success() {
                // Si la ejecución fue exitosa, mostramos la salida
                println!("{}", String::from_utf8_lossy(&commando.stdout));
            } else {
                // Si ocurrió un error, mostramos el error
                eprintln!("Error al ejecutar el comando MySQL:");
                eprintln!("{}", String::from_utf8_lossy(&commando.stderr));
                exit(1); // Salir con código de error 1
            }
     */    



}

