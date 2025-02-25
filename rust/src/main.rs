use std::{cmp, vec};
mod tokens;
mod mc_format; // formato de impresion

// añadir al modulo de estadistica
// use std::collections::HashMap;

fn main(){

    // EJEMPLOS -- BORRAR AL FINAL DEL TEST
    //let token:String = tokens::genera_token(24);
    //println!("{token}");


    // TODO: REVISAR LOS PERCENTILES
   

    let vector: Vec<f32>  = vec![3.0, 3.0, 3.0, 3.9, 4.0, 4.0, 4.0, 4.0, 5.5, 6.0];
    let vector2: Vec<f32> = vec![1.0, 2.0, 3.0, 1.0, 1.5, 1.5, 1.5, 2.0, 3.0, 1.0];
    // [0.2, 1.0, 1.0, 1.0, 1.5, 1.5, 1.5, 2.0, 2.0, 3.0, 0.1];
    // [2.2, 3.0, 4.0, 5.0, 2.5, 3.5, 4.5, 5.0, 3.0, 4.0, 0.2];
    
   // [0.2, 1.0, 1.0, 1.0, 1.5, 1.5, 1.5, 2.0, 2.0, 3.0, 
   //  3.0, 3.0, 3.0, 3.9, 4.0, 4.0, 4.0, 4.0, 5.5, 6.0, 
   //  1.0, 2.0, 3.0, 1.0, 1.5, 1.5, 1.5, 2.0, 3.0, 1.0, 
   //  3.0, 6.0, 5.5, 3.9, 0.2, 4.0, 4.0, 3.0, 4.0, 4.0
   // ];



    // vec![0.2, 1.0, 1.0, 1.0, 1.5, 1.5, 1.5, 2.0, 2.0, 3.0, 3.0, 3.0, 3.0];
    // vec![16.000, 18.000, 18.000, 20.000, 21.000, 21.000, 22.000, 23.000, 24.000, 24.000, 25.000, 26.000, 28.000];
    // 16.000, 18.000, 18.000, 20.000, 21.000, 21.000, 22.000, 23.000, 24.000, 24.000, 25.000, 26.000, 28.000
    // vec![0.2, 1.0, 1.0, 1.0, 1.5, 1.5, 1.5, 2.0, 2.0, 3.0, 3.0, 3.0, 3.0, 3.9, 4.0, 4.0, 4.0, 4.0, 5.5, 6.0, 1.0, 2.0, 3.0, 1.0, 1.5, 1.5, 1.5, 2.0, 3.0, 1.0, 3.0, 6.0, 5.5, 3.9, 0.2, 4.0, 4.0, 3.0, 4.0, 4.0, 3.0  ];
                                
 
    let media = media(&vector);
    let mediana  = mediana(&vector);
    let modas:Vec<(usize, f32)> = moda(&vector);
    let desviacion_std:f32 = desv_std(&vector, &media);
    let varianzas: [f32;2] = varianza(&vector, &media);
    let rango:f32 = rango(&vector);
    let mayor:f32 = valor_maximo(&vector);
    let minimo:f32 = valor_minimo(&vector);
    let cef_var:f32 = coef_variacion(&desviacion_std, &media);
    let qril: Vec<f32> =  quartiles(&vector);
    let conteo:Vec<(usize, f32)> =  conteo_vec(&vec_ord_asc(&vector) );
    let factor_corr_pearson:f32 = fctr_corrlcn_pearson(&vector, &vector2);

    let factor_corr_spearman:f32 = fctr_corrlcn_spearman(&vector, &vector2);


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
Coeficiente de desviación: {}
Factor de correlación: {}
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
factor_corr_pearson
);
        
        // pr_sep!(msg);

}

// SECCIÓN ESTADISTICA


// CALCULOS DESCRIPTIVOS

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

    let vec_tuplas: Vec<(usize, f32)> = conteo_vec(vector_i);
    let mut vec_tuplas2: Vec<(usize, f32)> = Vec::new();
    let mut n: usize = vec_tuplas.len();
    let mut acm: usize = vec_tuplas[0].0;
    let mut mayor_menor = true;   
    
    for j in 0.. n{
        mayor_menor = acm <= vec_tuplas[j].0 ;
        if mayor_menor { 
            if acm == vec_tuplas[j].0{
                vec_tuplas2.push(vec_tuplas[j]);
            }else if acm < vec_tuplas[j].0{
                vec_tuplas2.clear();
                vec_tuplas2.push(vec_tuplas[j]);
                acm = vec_tuplas[j].0;
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
    } else{ return minimo }
}
#[doc=r"Devuelve el rango el mayor<f32> - el menor<f32> de un vector<f32> referenciado"]
fn rango(vector:&Vec<f32>)-> f32{
    let maximo = valor_maximo(vector);
    let minimo = valor_minimo(vector);
    return maximo - minimo
}


#[doc=r"Devuelve le calculo del coefeicinte de variacón en un f32"]
pub fn coef_variacion(desv_estan:&f32, media:&f32)->f32{
    (desv_estan / media) * 100.0
}

#[doc=r"Retorn un Vector<[f32;3]> con tres f32 que representan v[0] = q1, v[1]= q2, v[2]= q3 los qurtiles en el vector referenciado ordenado acendentemente"]
fn quartiles(vector:&Vec<f32>)->Vec<f32>{

    // Q1, Q2, Q3
    let mut qurtiles:Vec<f32> = vec![0.0, 0.0, 0.0];
    let vec_no_repetidos: Vec<f32> = vec_no_rptidos(vector);
    let mut vec_temp1:Vec<f32> = Vec::new();
    let mut vec_temp3:Vec<f32> = Vec::new();

    qurtiles[1] = mediana(&vec_no_repetidos);

    for i in vector.iter(){
        if i < &qurtiles[1]{ vec_temp1.push(*i) }
        else if i > &qurtiles[1]{ vec_temp3.push(*i)  }

    }

    qurtiles[0] = mediana(&vec_temp1);
    qurtiles[2] = mediana(&vec_temp3);

    // ------------|---------|---------|------------
    // [0.2, 1.0, 1.5, 2.0, 3.0, 3.9, 4.0, 5.5, 6.0]
    qurtiles
}

fn percentiles(vector_i: &Vec<f32>, perc:&f32){

    

    let vector = vec_ord_asc(vector_i);
    let pos_calculada: f32 =  90f32/100f32 * ( (vector.len() as f32 - 1f32)  + 1f32 ) ;
    let mut promedio:f32 = 0.0;


    println!("{}", igualf32(pos_calculada % 2f32, 0f32) );

    if igualf32(pos_calculada % 2f32, 0f32){
        promedio = pos_calculada / 2f32;
        promedio =  vector[promedio as usize];
    }else{
        let valor_posicion_1:f32 = vector[pos_calculada.floor() as usize];
        let valor_posicion_2:f32 = vector[pos_calculada.floor() as usize + 1];
        let posicion_1:usize = pos_calculada.floor() as usize;  
        let posicion_2:usize = pos_calculada.floor() as usize + 1; 
        promedio =   valor_posicion_1 + ( pos_calculada - posicion_1 as f32) * ( (valor_posicion_2 - valor_posicion_1 ) / (posicion_2 as f32 - posicion_1 as f32)  );
    }

    println!("{:?}", vector);
    println!("{pos_calculada} - {promedio}");
}


#[doc=r"Retorna un vectorV<tuplas(unsize, f32)> de tuplas con los valores en orden asc y el numero de aparciones en el vector de datos"]
fn conteo_vec(vector: &Vec<f32>)-> Vec<(usize, f32)>{

    let mut vec_tuplas: Vec<(usize, f32)> = Vec::new();
    let mut tupla:(usize, f32) = (0,0.0);
    let n: usize = vector.len(); 
    let mut acm: usize = 0;
    let mut repeticiones: usize = 0;
    let mut vector_tmp: Vec<f32> =  Vec::new();

    for i in 0.. n{
        for j in acm.. n{
            if igualf32(vector[i], vector[j]){
                repeticiones = repeticiones + 1;
            }
        }

        if !vector_tmp.contains(&vector[i]) {
            vector_tmp.push(vector[i]);
            tupla.0 = repeticiones;
            tupla.1 = vector[i];
            vec_tuplas.push(tupla); 
        }

        repeticiones = 0;
        acm += 1;
    }
   
    vec_tuplas
}


// ANALSIS DE CORRELACIÓN

#[doc=r"Devuelve el coeficiente de correlació de Pearson en un f32 de dos vectores<f32> referenciados"]
pub fn fctr_corrlcn_pearson(val:&Vec<f32>, val2:&Vec<f32>) -> f32 {
    
    let mut vec_x: Vec<f32> = Vec::new();
    let mut vec_y: Vec<f32> = Vec::new();
    
    let mut sum_xe2: f32 = 0.0;
    let mut sum_ye2: f32 = 0.0;
    
    let mut xy:f32 = 0.0;
    
    // # pares de datos
    let n = val.len() as f32;
    
    // media de los vectores
    let med_x:f32 =  val.iter().fold(0f32,|x:f32, y:&f32|x+y) / n;
    let med_y:f32 = val2.iter().fold(0f32,|x:f32, y:&f32|x+y) / n;
    
    // diferencias en vector
    for i in val.iter(){ vec_x.push(i - med_x) }
    for i in val2.iter(){ vec_y.push(i - med_y ) }
    
    // sum del producto de vectores  € (x-x^p)(y-y^p)
    for i in 0..n as i32{ xy += vec_x[i as usize] * vec_y[i as usize] }
    
    // sumatoria de cuadrados
    vec_x.clone().into_iter().for_each(|x:f32|{ sum_xe2 += x.powi(2) });
    vec_y.clone().into_iter().for_each(|y:f32|{ sum_ye2 += y.powi(2) });
    
    // aplicando formula
    let res = xy / (sum_xe2 * sum_ye2).sqrt();
    
    res
    
    }

    #[doc=r"Devuelve el coeficiente de correlació de Spearman en un f32 de dos vectores<f32> referenciados"]
pub fn fctr_corrlcn_spearman(val:&Vec<f32>, val2:&Vec<f32>) -> f32 {

    let mut acm: f32 = 1.0;
    let mut acm_tmp: f32 = 0.0;
    let mut vec_pro:  Vec<(usize, f32)> = Vec::new();
    let mut vec_pro2: Vec<(usize, f32)> = Vec::new();
    let mut rangos: Vec<f32> = Vec::new();
    let vec_1: Vec<(usize, f32)> = conteo_vec( &vec_ord_asc(val ) );
    let vec_2: Vec<(usize, f32)> = conteo_vec( &vec_ord_asc(val2 ) );
    
    let mut vec_cn_rangos:  Vec<(usize,f32)> = Vec::new();
    let mut vec_cn_rangos2: Vec<(usize,f32)> = Vec::new();

    for i in 0..val.len(){
        vec_cn_rangos.push((i, val[i]));
        vec_cn_rangos2.push((i, val2[i]));
    }

    println!("{vec_cn_rangos:?} \n {vec_cn_rangos2:?}");


/*
    for i in 0..vec_1.len(){
        if vec_1[i].0 != 1 {
            for j in 0..vec_1[i].0{
                acm_tmp += acm;
                acm += 1f32;
            }
            for j in 0..vec_1[i].0{ vec_pro.push((i,acm_tmp as f32/vec_1[i].0 as f32) ); }
            acm_tmp = 0.0;
            continue
        }
        vec_pro.push((i,acm));
        acm += 1f32;
    }

    acm = 1.0;
    acm_tmp = 0.0;

    for i in 0..vec_2.len(){
        if vec_2[i].0 != 1 {
            for j in 0..vec_2[i].0{
                acm_tmp += acm;
                acm += 1f32;
            }
            for j in 0..vec_2[i].0{ vec_pro2.push((i, acm_tmp as f32/vec_2[i].0 as f32) ) }
            acm_tmp = 0.0;
            continue
        }
        vec_pro2.push((i,acm));
        acm += 1f32;
    }

    acm = 0.0;
    let n: f32 = val.len() as f32;

    for i in 0..vec_pro.len(){
        rangos.push(vec_pro[i].1 - vec_pro2[i].1);
        // acm += (vec_pro[i] - vec_pro2[i]).powi(2);
        // println!("{} {} {acm}", vec_pro[i], vec_pro2[i]);
    }

    println!("----\n{rangos:?}\n-----");
    // println!("{}", ( 6f32 * acm ));

    acm = 1f32 - ( ( 6f32 * acm ) / (n * ( n.powi(2) - 1f32 ) ) );
    
    println!("{}", 1f32-acm);
    // println!("{}", (n * ( n.powi(2) - 1f32 ) ) );
    // acm_tmp = 1f32 - ( ( 6f32 * acm ) / (n * ( n.powi(2) - 1f32 ) ) );
  
     println!("{:?}",vec_pro);
     println!("{:?}",vec_pro2);
// 
// 
   //  print!("--{acm_tmp}");
// 
*/
    3.0
    }











// UTILSS
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
