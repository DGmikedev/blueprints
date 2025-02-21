use std::{cmp, vec};
mod tokens;
mod mc_format; // formato de impresion

// añadir al modulo de estadistica
// use std::collections::HashMap;

fn main(){


    // EJEMPLOS -- BORRAR AL FINAL DEL TEST
    //let token:String = tokens::genera_token(24);
    //println!("{token}");

    let vector: Vec<f32> = vec![ 1.0, 2.0, 3.0, 1.0, 1.5, 1.5, 1.5, 2.0, 3.0, 1.0, 3.0, 6.0, 5.5, 3.9, 0.2, 4.0, 4.0, 4.0, 4.0  ];

    let media = media(&vector);
    let mediana  = mediana(&vector);
    let modas:Vec<(usize, f32)> = moda(&vector);
    let desviacion_std:f32 = desv_std(&vector, &media);
    let varianzas: [f32;2] = varianza(&vector, &media);
    let rango:f32 = rango(&vector);
    let mayor:f32 = valor_maximo(&vector);
    let minimo:f32 = valor_minimo(&vector);


   pr_sep!();
   let msg: String = format!("Datos: {:?}\nMedia: {}\nMediana: {}\nModas: {:?}\nDeviacion_std: {}\nVarianzas: {:?}\nRango: {}\nValor mayor: {}\nValor minimo: {} \n", vector, media, mediana, modas, desviacion_std, varianzas,rango, mayor, minimo);
   pr_sep!(msg);
}


// SECCIÓN ESTADISTICA

#[doc=r"Devuelve el calculo de la media de un vector<f32> referenciado"]
fn media(vector :&Vec<f32>)->f32{ 
    let media = vector.iter().sum::<f32>() / vector.len() as f32; 
    media 
}
 
#[doc=r"Devuelve una copia de un vector<f32> referenciado acomodado en orden descendente "] 
fn vec_ord_dec(vector_i:&Vec<f32>)->Vec<f32>{
    let mut vector = vector_i.clone();
    vector.sort_by(|a,b| b.partial_cmp(a).unwrap() );
    vector
}

#[doc=r"Devuelve una copia de un vector<f32> referenciado acomodado en orden ascendente"] 
fn vec_ord_asc(vector_i:&Vec<f32>)->Vec<f32>{
    let mut vector = vector_i.clone();
    vector.sort_by(|a,b| a.partial_cmp(b).unwrap() );
    vector
}

#[doc=r"* Quita valores repetidos en un vector
* Devuelve una copia de un vector<f32> referenciado quitando los valores repetidos"]
fn vec_no_rptidos(vector_i: &Vec<f32>)->Vec<f32>{
    
    let mut vector: Vec<f32> = Vec::new();
    for &i in vector_i.iter(){
        if !vector.contains(&i){ vector.push(i) }
    }
    vector
}

#[doc = r"* devuelve un f32 con el calculo de la mediana de un vector<f32> referenmciado"]
fn mediana(vector_i: &Vec<f32>)-> f32{

    let mut med: f32 = 0.0;
    let vector: Vec<f32> = vec_ord_asc(vector_i);
    let n: usize = vector.len();
  
    if n%2 == 0 { med = (vector[( n/2 )-1] + vector[n/2]) / 2f32}
    else{ med = vector[( (n+1)/2 ) - 1] }

    med
}

#[doc=r"Devuelve un vector<(unisze, f32)>  con tuplas que describen el o los 
        números con mayor concurrencia [(concurrencias, valor)]"]
fn moda(vector_i:&Vec<f32>)-> Vec<(usize, f32)>{

    let mut vec_tuplas: Vec<(usize, f32)> = Vec::new();
    let mut vec_tuplas2: Vec<(usize, f32)> = Vec::new();
    let mut tupla:(usize, f32) = (0,0.0);
    let mut n: usize = vector_i.len(); 
    let mut acm: usize = 0;
    let mut repeticiones: usize = 0;
    let mut vector_tmp: Vec<f32> =  Vec::new();
    let mut mayor_menor = true;   
 
    for i in 0.. n{
        for j in acm.. n{
            if igualf32(vector_i[i], vector_i[j]){
                repeticiones = repeticiones + 1;
            }
        }

        if !vector_tmp.contains(&vector_i[i]) {
            vector_tmp.push(vector_i[i]);
            tupla.0 = repeticiones;
            tupla.1 = vector_i[i];
            vec_tuplas.push(tupla); 
        }

        repeticiones = 0;
        acm += 1;
    }

    acm = vec_tuplas[0].0;
    n = vec_tuplas.len();
    

    for j in 0.. n{
        mayor_menor = acm <= vec_tuplas[j].0 ;
        if mayor_menor { 
            if acm == vec_tuplas[j].0{
                vec_tuplas2.push(vec_tuplas[j]);
            }else if acm < vec_tuplas[j].0{ 
                vec_tuplas2.clear();
                vec_tuplas2.push(vec_tuplas[j]);
                
            }
        } 
    }
    vec_tuplas2

}

#[doc=r"Devuelve el calculo de la desviación estandar en un f32 de un vector<f32> referenciado"]
pub fn desv_std(val:&Vec<f32> , med: &f32) -> f32 {

    let mut var:f32 = 0.0;
    val.iter()
        .for_each(|x: &f32|{ 
            var = var + (x - med).powi(2);
        }); 
        var = var/(val.len() as f32 - 1f32);
        var = var.sqrt();
        var
}

#[doc=r"Devuleve un vector<f32,f32> con un vector referenciado<f32> y la media<f32> ambos referenciados"]
pub fn varianza(val:&Vec<f32> , med: &f32) -> [f32; 2] {

    let mut var:f32 = 0.0;
    val.into_iter()
        .for_each(|x: &f32|{ 
            var += (x - med).powi(2);
         }); 
    [var /(val.len() as f32), var /(val.len() as f32 - 1f32)]
}

#[doc=r"Devuelve el valor maximo<f32> contenido en un vector<f32> referenciado"]
fn valor_maximo(vec:&Vec<f32>)-> f32{
    let mut maximo: f32 = 0.0;
    if let Some(&valor_maximo) = vec.iter()
            .max_by(|a,b| a.partial_cmp(b).unwrap()){
        maximo = valor_maximo.clone();
        return maximo
    }else{ 
        return maximo
    }
}

#[doc=r"Devuelve el valor minimo<f32> contenido en un vector<f32> referenciado"]
fn valor_minimo(vec:&Vec<f32>)-> f32{
    let mut minimo: f32 = 0.0;
    if let Some(&valor_minimo) = vec.iter()
            .min_by(|a,b| a.partial_cmp(b).unwrap()){
        minimo = valor_minimo.clone();
        return minimo
    }else{ 
        return minimo
    }
}
#[doc=r"Devuelve el rango el mayor<f32> - el menor<f32> de un vector<f32> referenciado"]
fn rango(vector:&Vec<f32>)-> f32{
    let maximo = valor_maximo(vector);
    let minimo = valor_minimo(vector);
    return maximo - minimo
}

/*
fn main() {
    let vec: Vec<f32> = vec![10.5, 23.3, 5.7, 17.2, 42.9];

    // Obtener el valor máximo
    if let Some(&max_value) = vec.iter().max_by(|a, b| a.partial_cmp(b).unwrap()) {
        println!("Valor máximo: {}", max_value);
    } else {
        println!("El vector está vacío");
    }

    // Obtener el valor mínimo
    if let Some(&min_value) = vec.iter().min_by(|a, b| a.partial_cmp(b).unwrap()) {
        println!("Valor mínimo: {}", min_value);
    } else {
        println!("El vector está vacío");
    }
}


*/


#[doc=r"Devuelve un vector<V<V<f32>> de vectores referenciado, asemejando una matriz, esta matriz devuleta será transpuesta"]
fn transpuestafn( arr:&Vec<Vec<f32>> )->Vec<Vec<f32>>{

    let high:usize = arr.len();
    let with:usize = arr[0].len();
    let mut arr_return: Vec<Vec<f32>> = vec![vec![0.0; high]; with];
        for (i, vect) in arr.into_iter().enumerate(){ 
            for j in 0..vect.len(){
                arr_return[j][i] = arr[i][j]; 
            }
        }
    arr_return
}

fn igualf32(a: f32, b: f32) -> bool {
    let epsilon = f32::EPSILON * 1000.0; // Ajusta este valor si es necesario
    (a - b).abs() < epsilon
}


// DATA
// | vec     |.sum()  -> suma
// | mut vec |.sort_by(|a,b| a.partial_cmp(b).unwrap() ); -> ordena menor a mayor f32
// | mut vec |.sort_by(|a,b| b.partial_cmp(a).unwrap() ); -> ordena mayor a menor f32
// | respons |.unwrap() -> extrae el resultado, es un tipo Option
// | vec     |.contains -> contiene
// | vec     |let mut hashmap: HashMap<K, V>= HashMap::new(); -> declara u hashmap -> fn nombre_funcion<K, V> determinar el contexto
// | vec     |if let Some(&valor_maximo) = vec.iter().max_by(|a,b| a.partial_cmp(b).unwrap()){   -> Some es un contenedor de respuesta si es exitoso lo guarda en &valor_maximo del if
// |         |y si no lo guarda en el else,  ->max_by(|a,b| a.prtial_cmp(b)...)  obtiene el maximo comparando parcialmete a y b






 