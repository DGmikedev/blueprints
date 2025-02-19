mod tokens;


fn main(){


    // EJEMPLOS -- BORRAR AL FINAL DEL TEST
    let token:String = tokens::genera_token(24);
    println!("{token}");

    let vector: Vec<f32> = vec![1.0, 2.0, 3.0];
    let g = media(&vector);
   
    println!("{g}");

}


// SECCIÓN ESTADISTICA

fn media(vector:&[f32])->f32{ 
    let media = vector.iter().sum::<f32>() / vector.len() as f32; 
    media 
}

