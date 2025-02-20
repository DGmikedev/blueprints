use std::{cmp, vec};
mod tokens;
mod mc_format; // formato de impresion


fn main(){


    // EJEMPLOS -- BORRAR AL FINAL DEL TEST
    //let token:String = tokens::genera_token(24);
    //println!("{token}");

    let vector: Vec<f32> = vec![12.0, 3.0, 7.2, 11.0, 1.0, 3.0, 6.3, 2.0, 3.0,4.0,2.0, 3.2, 5.1,5.0, 6.0, 2.0, 8.0, 6.0, 9.0];

    let media = media(&vector);
    let mediana  = mediana(&vector);





    pr_sep!();
    let msg: String = format!("Datos: {:?}\nMedia: {}\nMediana: {}", vector, media, mediana);
    pr_sep!(msg);
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


// DATA
// | vec     |.sum()  -> suma
// | mut vec |.sort_by(|a,b| a.partial_cmp(b).unwrap() ); -> ordena menor a mayor f32
// | mut vec |.sort_by(|a,b| b.partial_cmp(a).unwrap() ); -> ordena mayor a menor f32
// | respons |.unwrap() -> extrae el resultado, es un tipo Option
// | vec     |.contains -> contiene
//





