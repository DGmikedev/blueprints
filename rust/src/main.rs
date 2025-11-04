use std::{cmp, vec};
mod tokens;
mod mc_format; // formato de impresion
mod estadistica;

use rand::{self, Rng};
mod seeder;
mod editor_text;

// añadir al modulo de estadistica
use std::collections::HashMap;

fn main(){

        // [
//     ["banco_central", "usuarios_registrados", "800"], 
//     ["Columna 2", "NAM", "0", "0"], 
//     ["Columna 3", "ADD", "0", "0"], 
//     ["Columna 4", "DEC", "10", "2"], 
//     ["Columna 5", "SMA", "0", "0"], 
//     ["Columna 6", "VAR", "0", "0"], 
//     ["Columna 7", "BIN", "26", "0"], 
//     ["Columna 8", "DAT", "0", "0"]
// ]



        // Insertador de regisros en tabla de ususarios
        // 200, 400, 800, 1000, 5000, 10000
      //   seeder::get_inserts("pltfrm_laravel".to_string(), "users".to_string(), 10);
        


    // EJEMPLOS -- BORRAR AL FINAL DEL TEST


    //let token:String = tokens::genera_token(24);
    //println!("{token}");

    // TODO: REVISAR LOS PERCENTILES
   
   /*
    let vector: Vec<f32> = vec![1.0, 2.0, 3.0, 1.0, 1.5, 1.5, 1.5, 2.0, 3.0, 1.0];
    let vector2: Vec<f32>  = vec![3.0, 3.0, 3.9, 4.0, 4.0, 4.0, 5.5, 6.0, 3.0, 4.0];
 
    let media:f32                =  estadistica::media(&vector);
    let mediana:f32              =  estadistica::mediana(&vector);
    let modas:Vec<(usize, f32)>  =  estadistica::moda(&vector);
    let desviacion_std:f32       =  estadistica::desv_std(&vector, &media);
    let varianzas: [f32;2]       =  estadistica::varianza(&vector, &media);
    let rango:f32                =  estadistica::rango(&vector);
    let mayor:f32                =  estadistica::valor_maximo(&vector);
    let minimo:f32               =  estadistica::valor_minimo(&vector);
    let cef_var:f32              =  estadistica::coef_variacion(&desviacion_std, &media);
    let qril: Vec<f32>           =  estadistica::quartiles(&vector);
    let conteo:Vec<(usize, f32)> =  estadistica::conteo_vec(&estadistica::vec_ord_asc(&vector) );
    let factor_corr_pearson:f32  =  estadistica::fctr_corrlcn_pearson(&vector, &vector2);
    let factor_corr_spearman:f32 =  estadistica::fctr_corrlcn_spearman(&vector, &vector2);

//    percentiles(&vector, &30.0);

        
        pr_sep!();
        let msg: String = format!("
Datos: {:?}
Conteo: {:?}
Quartiles: {:?}
Media: {}
Mediana: {}
Modas: {:?}
Deviacion_std: {}
Varianzas: {:?}
Rango: {}
Valor mayor: {}
Valor minimo: {}
Coeficiente de variación: {}
Factor de correlación pearson: {}
Factor de correlación spearman: {}
", 
vector, 
conteo,
qril,
media, 
mediana, 
modas, 
desviacion_std, 
varianzas,
rango, 
mayor, 
minimo, 
cef_var,
factor_corr_pearson,
factor_corr_spearman
);
        
        pr_sep!(msg);
*/


let datos: Vec<Vec<String>> = vec![
    vec!["banco_central".to_string(), "usuarios_registrados".to_string(), "800".to_string()], 
    vec!["Columna_2".to_string(), "NAM".to_string(), "0".to_string(), "0".to_string()], 
    vec!["Columna_3".to_string(), "ADD".to_string(), "0".to_string(), "0".to_string()], 
    vec!["Columna_4".to_string(), "DEC".to_string(), "10".to_string(), "2".to_string()], 
    vec!["Columna_5".to_string(), "SMA".to_string(), "0".to_string(), "0".to_string()], 
    vec!["Columna_6".to_string(), "VAR".to_string(), "0".to_string(), "0".to_string()], 
    vec!["Columna_7".to_string(), "BIN".to_string(), "26".to_string(), "0".to_string()], 
    vec!["Columna_8".to_string(), "DAT".to_string(), "0".to_string(), "0".to_string()]
];


rx_data(datos)


}



fn rx_data(data: Vec<Vec<String>>){
       
        let mut cols_names: Vec<String> = Vec::new();
        
        for i in 1..data.len(){ cols_names.push(data[i][0].clone()) }

        // crea las cabezeras       

        // "INSERT INTO banco_central.usuarios_registrados 
        //      (Columna_2,Columna_3,Columna_4,Columna_5,Columna_6,Columna_7,Columna_8) VALUES "

        let mut script: String = create_header_script(cols_names, data[0].clone());
        
        // Crear documento
        crear_txt(script.clone(), data[0].clone());

        
  // let salidatxt: Result<(), std::io::Error> = 
  // editor_text::edit_txt( name, path, text );

       // println!("{:?}",data);
} 


fn create_header_script(schema:Vec<String>, head_data: Vec<String>)->String{

        let array_cols_name = schema; 
        let bd: &String = &head_data[0];
        let table: &String = &head_data[1];
        let mut acm: usize = 0;
        let mut head_script = format!("INSERT INTO {}.{} ( ", bd, table); 
            
        for i in array_cols_name.iter(){
            acm += 1;
            head_script.push_str(i);
            if acm == array_cols_name.len(){ head_script.push_str("") }
            else{ head_script.push_str(",") }
        }
        head_script.push_str(" ) VALUES ");
        head_script
            
}


fn crear_txt(text: String, datos:Vec<String>){
    
   // let text: String =  String::from(text);
   let name: String =  format!("{}_{}.sql", &datos[0], &datos[1]);
   let path: String =  format!("{}/{}", &datos[0], &datos[1]);    
   let salidatxt: Result<(), std::io::Error> = 
        editor_text::edit_txt( name, path, text );
}




// SECCIÓN ESTADISTICA







// FUNCIONES USADAS
// | vec     |.sum()  -> suma
// | vAL     |.powi(2)  -> Exponente al cuadrado
// | mut vec |.sort_by(|a,b| a.partial_cmp(b).unwrap() ); -> ordena menor a mayor f32
// | mut vec |.sort_by(|a,b| b.partial_cmp(a).unwrap() ); -> ordena mayor a menor f32
// | respons |.unwrap() -> extrae el resultado, es un tipo Option
// | vec     |.contains -> contiene
// | vec     |let mut hashmap: HashMap<K, V>= HashMap::new(); -> declara u hashmap -> fn nombre_funcion<K, V> determinar el contexto
// | vec     |if let Some(&valor_maximo) = vec.iter().max_by(|a,b| a.partial_cmp(b).unwrap()){   -> Some es un contenedor de respuesta si es exitoso lo guarda en &valor_maximo del if
// |         |y si no lo guarda en el else,  ->max_by(|a,b| a.prtial_cmp(b)...)  obtiene el maximo comparando parcialmete a y b

