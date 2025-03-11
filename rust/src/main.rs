use std::{cmp, vec};
mod tokens;
mod mc_format; // formato de impresion
mod estadistica;

use rand::{self, Rng};
mod seeder;


// añadir al modulo de estadistica
use std::collections::HashMap;

fn main(){

        // Insertador de regisros en tabla de ususarios
        seeder::get_inserts("pltfrm_laravel".to_string(), "users".to_string(), 200);
        


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

