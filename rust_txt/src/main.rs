use std::fs::File;
use std::io::Error;


fn main() {

    let path: String = String::from("test/archivo1/");
    let name: String = String::from("informe.txt");
    let full_name: String = format!("{path}{name}");
    let text: String = String::from("Gloucester: Yo hago el mal, y no soy el primero en empezar a regañar. Las maldades secretas que preparo, las pongo a cuenta de otros, como culpa suya. A Clarence, a quien, desde luego, he puesto yo en la tiniebla, ahora le lamento delante de muchos simples bobos; esto es, ante Hastings, Stanley y Buckingham; y digo que son la Reina y sus aliados quienes mueven al Rey contra mi hermano el Duque. Ahora se lo creen; y a la vez me dejan vengarme de Rivers, Vaughan y Grey: pero entonces suspiro y, con un trozo de la Escritura, les digo que Dios nos manda hacer bien por mal, revistiendo así mi desnuda villanía con retazos viejos robados de la Santa Biblia; parezco un santo cuando más hago el diablo. ");

// 1) =>  Se crea el directorio
    let path_directory: bool =  editor_txt::create_directory(path.clone());

    // RETURN SI FALLA LA CREACIÓN DEL DIRECTORIO
    if !&path_directory { println!("No se pudo crear el directorio indicado: {}", &path) }

// 2) => se crea el archvio
    let mut document: Result<File, Error> = editor_txt::create_file(path.clone(), name);

    // RETURN SI FALLA LA CREACIÓN DEL DIRECTORIO
    match &document{
        Ok(v) => println!("Archivo creado"),
        _ => println!("Error al crear el archivo indicado: {}", &full_name),
    }

    for i in 0..10000{
        let _ = editor_txt::insert_txt_by_ln(full_name.clone(),text.clone());
    }

}

mod editor_txt{

 
    use std::os::windows::fs::OpenOptionsExt;
    use std::path::{ PathBuf, Path };
    use std::fs::create_dir_all;
    use std::fs::{ File, OpenOptions };
    use std::io::Error;
    use std::io::{self, BufWriter, Write};


    pub fn create_directory(path:String)->bool{

        let mut _path: PathBuf = PathBuf::from(path);
        let dir: Result<(), Error> = create_dir_all(&_path);
        match dir {
            Ok(v) => return true,
            _ =>  return false,
        }
    }

    pub fn create_file(path: String, name: String)->Result<File, Error>{

        let path_full: String = format!("{}{}",path, name);
        let doc: Result<File, Error> = File::create(path_full);
        match doc{
            Ok(v) => return Ok(v),
            _ => return doc,
        }
    }

    pub fn insert_txt_by_ln(document: String, line: String) -> std::io::Result<()> { 

        let path = Path::new(&document);
        let _file = OpenOptions::new()
        .create(false)
        .append(true)
        .open(path)?;
        let mut writer = BufWriter::new(_file);
        writeln!(writer, "{}", line)?;
        Ok(())
    }



}
