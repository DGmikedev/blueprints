use std::{cmp, vec};
mod tokens;
mod mc_format; // formato de impresion

// añadir al modulo de estadistica
use std::collections::HashMap;



fn main(){


    // EJEMPLOS -- BORRAR AL FINAL DEL TEST
    //let token:String = tokens::genera_token(24);
    //println!("{token}");

    let vector: Vec<f32> = vec![21.0, 1.0,1.0,12.0, 3.0, 7.2, 11.0, 1.0, 3.0, 6.3, 2.0, 3.0,4.0,2.0, 3.2, 5.1,5.0, 6.0, 2.0, 8.0, 6.0, 9.0];

    let media = media(&vector);
    let mediana  = mediana(&vector);


    moda::<usize, f32>(&vector);

   // pr_sep!();
   // let msg: String = format!("Datos: {:?}\nMedia: {}\nMediana: {}", vector, media, mediana);
   // pr_sep!(msg);
}


// SECCIÓN ESTADISTICA

fn media(vector :&Vec<f32>)->f32{ 
    let media = vector.iter().sum::<f32>() / vector.len() as f32; 
    media 
}
 
fn vec_ord_dec(vector_i:&Vec<f32>)->Vec<f32>{
    let mut vector = vector_i.clone();
    vector.sort_by(|a,b| b.partial_cmp(a).unwrap() );
    vector
}

fn vec_ord_asc(vector_i:&Vec<f32>)->Vec<f32>{
    let mut vector = vector_i.clone();
    vector.sort_by(|a,b| a.partial_cmp(b).unwrap() );
    vector
}


fn vec_no_rptidos(vector_i: &Vec<f32>)->Vec<f32>{
    
    let mut vector: Vec<f32> = Vec::new();
    for &i in vector_i.iter(){
        if !vector.contains(&i){ vector.push(i) }
    }
    vector
}

fn mediana(vector_i: &Vec<f32>)-> f32{

    let mut med: f32 = 0.0;
    let vector: Vec<f32> = vec_ord_asc(vector_i);
    let n: usize = vector.len();
  
    if n%2 == 0 { med = (vector[( n/2 )-1] + vector[n/2]) / 2f32}
    else{ med = vector[( (n+1)/2 ) - 1] }

    med
}

fn moda<K,V>(vector_i:&Vec<f32>){

    let mut hashmapvec: HashMap<String, usize>  = HashMap::new();

    let mut vec_tuplas: Vec<(usize, f32)> = Vec::new();
    let mut tupla:(usize, f32) = (0,0.0);

    let n: usize = vector_i.len(); 
    let mut acm: usize = 0;
    let mut repeticiones: usize = 0;
    
    let clave: i32 = 0;
    let val = 3f32;
    // hashmapvec.insert(1,vector_i[0]);
    pr_v!(hashmapvec);
 
    for i in 0.. n{
        for j in acm.. n{

            if igualf32(vector_i[i], vector_i[j]){
                repeticiones = repeticiones + 1;
            }
        }

        tupla.1 = vector_i[i];
        tupla.0 = repeticiones;
        vec_tuplas.push(tupla);        

        // if !hashmapvec.contains_key(&vector_i[i].to_string()){
        //     hashmapvec.insert(vector_i[i].to_string(), repeticiones);
        // }
        repeticiones = 0;
        acm += 1;
    }

    //pr_v!(hashmapvec);
    pr_v!(vec_tuplas);


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





 