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

   pr_sep!();
   let msg: String = format!("Datos: {:?}\nMedia: {}\nMediana: {}\nModas: {:?}", vector, media, mediana, modas);
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





 