/// COMPARACIÓN DE DOS NUMEROS DE PUNTO FLOTANTE
///  
/// # EJEMPLO
/// ```rust
///  let is_equal: bool = igualf32(3.0,325);
///  assert_eq!(is_equal, true);
/// ```
/// # PARAMETROS
///  * `f32` número a comparar
///  * `f32` número a comparar
/// 
/// # RETORNO
/// * `boolean`
/// 
/// # PANORAMICA
/// 
/// Esta función compara dos números de punto flotante y retorna tru o false.

pub fn igualf32(a: f32, b: f32) -> bool {
    let epsilon = f32::EPSILON * 1000.0; // Ajusta este valor si es necesario
    (a - b).abs() < epsilon
}

/// AGRUPA LA FRECUENCIA Y VALOR QUE APARECE EN UN VECTOR
///  
/// # EJEMPLO
/// ```rust
/// 
/// let vector: Vec<f32> = vec![1.0, 2.0, 3.0, 1.0, 1.5, 1.5, 1.5, 2.0, 3.0, 1.0];
/// 
/// let cont: Vec<(usize, f32)> = conteo_vec(&vector);
/// 
/// println!("{cont:?}");  // 
/// 
/// ```
/// ```bash
/// [(3, 1.0), (2, 2.0), (2, 3.0), (3, 1.5)]
/// ```
/// # PARAMETROS
///  * `&Vec<f32>` Referencia a el vector a analisar 
///  
/// # RETORNO
/// * `Vec<(usize, f32)>`
/// * usize: numero de apariciones en el vector
/// * f32:   valor que es búscado
///
/// # PANORAMICA
/// 
/// Retorna un vector del tipo `Vec<(unsize, f32)>` con tuplas con 
/// el numero de apariciones y el valor que es contado
/// 
pub fn conteo_vec(vector: &Vec<f32>)-> Vec<(usize, f32)>{

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


// CALCULOS DESCRIPTIVOS

/// DEVUELVE LA MEDIA DE UN VECTOR
///  
/// # EJEMPLO
/// ```rust
/// 
/// let vector: Vec<f32> = vec![1.0, 2.0, 3.0, 1.0, 1.5, 1.5, 1.5, 2.0, 3.0, 1.0];
/// 
/// let est: f32 = media(&vector);
/// 
/// println!("{est}");
///
/// ```
/// ```bash
/// 1.75
/// ```
/// # PARAMETROS
///  * `&Vec<f32>` Referencia a el vector a analisar 
///  
/// # RETORNO
/// * `f32`  media del vector
/// 
/// # PANORAMICA
/// 
/// Devuelve el calculo de la media de un vector<f32> referenciado
/// 
pub fn media(vector :&Vec<f32>)->f32{ 
    let media = vector.iter().sum::<f32>() / vector.len() as f32; 
    media 
}

/// DEVUELVE LA MEDIA DE UN VECTOR
///  
/// # EJEMPLO
/// ```rust
/// 
/// let vector: Vec<f32> = vec![1.0, 2.0, 3.0, 1.0, 1.5, 1.5, 1.5, 2.0, 3.0, 1.0];
/// 
/// let est: f32 = media(&vector);
/// 
/// println!("{est}");
///
/// ```
/// ```bash
/// 1.75
/// ```
/// # PARAMETROS
///  * `&Vec<f32>` Referencia a el vector a analisar 
///  
/// # RETORNO
/// * `f32`  media del vector
/// 
/// # PANORAMICA
/// 
/// Devuelve una copia de un vector<f32> referenciado acomodado en orden descendente
/// 
pub fn vec_ord_dec(vector_i:&Vec<f32>)->Vec<f32>{
    let mut vector = vector_i.clone();
    vector.sort_by(|a,b| b.partial_cmp(a).unwrap() );
    vector
}

/// DEVUELVE UN VECTOR ORDENADO EN FORMA ASCENDENTE
///  
/// # EJEMPLO
/// ```rust
/// 
/// let vector: Vec<f32> = vec![1.0, 2.0, 3.0, 1.0, 1.5, 1.5, 1.5, 2.0, 3.0, 1.0];
/// 
/// let vec_asc: Vec<f32> = vec_ord_asc(&vector);
/// 
/// println!("{vec_asc:?}");
///
/// ```
/// ```bash
/// [1.0, 1.0, 1.0, 1.5, 1.5, 1.5, 2.0, 2.0, 3.0, 3.0]
/// ```
/// # PARAMETROS
///  * `&vector` Referencia a el vector a ordenar
///  
/// # RETORNO
/// * `&Vec<f32>` Vector ordenado de forma ascendente
/// 
/// # PANORAMICA
/// 
/// Devuelve un vector<f32> referenciado ordenado en orden ascendente
/// 

pub fn vec_ord_asc(vector_i:&Vec<f32>)->Vec<f32>{
    let mut vector = vector_i.clone();
    vector.sort_by(|a,b| { a.partial_cmp(b).unwrap() });
    vector
}

/// DEVUELVE UNA COPIA DE UN VECTOR REFERENCIADO SIN VALORES DUPLICADOS
///  
/// # EJEMPLO
/// ```rust
/// 
/// let vector: Vec<f32> = vec![1.0, 2.0, 3.0, 1.0, 1.5, 1.5, 1.5, 2.0, 3.0, 1.0];
/// 
/// let no_rptdos: Vec<f32> = vec_no_rptidos(&vector);
/// 
/// println!("{no_rptdos:?}");
///
/// ```
/// ```bash
/// [1.0, 2.0, 3.0, 1.5]
/// ```
/// # PARAMETROS
///  * `&vector` Referencia a el vector a copiar y limpiar de duplicados
///  
/// # RETORNO
/// * `&Vec<f32>` Copia de vector referenciado pero sin valores duplicados
/// 
/// # PANORAMICA
/// 
/// * Quita valores repetidos en un vector
/// * Devuelve un vector<f32> que es una copia de un vector referenciado pero sin valores duplicados
/// 

pub fn vec_no_rptidos(vector_i: &Vec<f32>)->Vec<f32>{
    
    let mut vector: Vec<f32> = Vec::new();
    for &i in vector_i.iter(){
        if !vector.contains(&i){ vector.push(i) }
    }
    vector
}

/// DEVUELVE EL CALCULO DE LA MEDIANA DE UN VECTOR
///  
/// # EJEMPLO
/// ```rust
/// 
/// let vector: Vec<f32> = vec![1.0, 2.0, 3.0, 1.0, 1.5, 1.5, 1.5, 2.0, 3.0, 1.0];
/// 
/// let _mediana: f32 = mediana(&vector);
/// 
/// println!("{_mediana:?}");
///
/// ```
/// ```bash
/// 1.5
/// ```
/// # PARAMETROS
///  * `&Vec<f32>` Referencia al vector al cual se le desea calcular la mediana
///  
/// # RETORNO
/// * `f32` Copia de vector referenciado pero sin valores duplicados
/// 
/// # PANORAMICA
/// 
/// * Devuelve un f32 con el calculo de la mediana de un vector<f32> referenmciado
/// 

pub fn mediana(vector_i: &Vec<f32>)-> f32{

    let mut med: f32 = 0.0;
    let vector: Vec<f32> = vec_ord_asc(vector_i);
    let n: usize = vector.len();
    if n%2 == 0 { med = (vector[( n/2 )-1] + vector[n/2]) / 2f32}
    else{ med = vector[( (n+1)/2 ) - 1] }

    med
}


/// DEVUELVE LOS VALORES CONMAYOR CONCURRENCIA DE UN VECTOR
///  
/// # EJEMPLO
/// ```rust
/// 
/// let vector: Vec<f32> = vec![1.0, 2.0, 3.0, 1.0, 1.5, 1.5, 1.5, 2.0, 3.0, 1.0];
/// 
/// let _moda: Vec<(usize,f32)> = moda(&vector);
/// 
/// println!("{_moda:?}");
///
/// ```
/// ```bash
/// [(3, 1.0), (3, 1.5)]
/// ```
/// # PARAMETROS
///  * `&Vec<f32>` Referencia al vector al cual se le desea calcular la moda
///  
/// # RETORNO
/// * `Vec<(usize,f32)>` vector con tuplas de concurrencías y valores
/// 
/// # PANORAMICA
/// 
/// * Devuelve un vector<(unisze, f32)> con tuplas que describen el o los 
///   números con mayor concurrencia Vec<(concurrencias, valor)>
/// 

pub fn moda(vector_i:&Vec<f32>)-> Vec<(usize, f32)>{

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

/// DEVUELVE EL CALCULO DE LA DESVIACIÓN ESTANDAR DE UN VECTOR REFERENCIADO Y SU MEDIA
///  
/// # EJEMPLO
/// ```rust
/// 
///  let vector: Vec<f32> = vec![1.0, 2.0, 3.0, 1.0, 1.5, 1.5, 1.5, 2.0, 3.0, 1.0];
/// 
///  let _media: f32 = media(&vector);
/// 
///  let _desv_std: f32 = desv_std(&vector, &_media);
/// 
///  println!("{_desv_std:?}");
/// 
/// ```
/// ```bash
/// 0.7546154
/// ```
/// # PARAMETROS
///  * `&Vec<f32>` Referencia al vector al cual se le desea calcular la desviación estandar
///  * `&f32`      Referencia al valor de la media del vector referenciado
/// 
/// # RETORNO
/// * `Vec<(usize,f32)>` vector con tuplas de concurrencías y valores
/// 
/// # PANORAMICA
/// 
/// * Devuelve el calculo de la desviación estandar en un f32 de un vector<f32> referenciado
///   

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

/// DEVUELVE EL CALCULO DE LAS VARIANZAS POBLACIONAL Y MUESTRAL
///  
/// # EJEMPLO
/// ```rust
///  let vector: Vec<f32> = vec![1.0, 2.0, 3.0, 1.0, 1.5, 1.5, 1.5, 2.0, 3.0, 1.0];
///  
///  let _media: f32 = media(&vector); 
///  
///  let _varianza: [f32;2] = varianza(&vector, &_media);
///  
///  println!("{_varianza:?}");
/// 
/// 
/// ```
/// ```bash
/// [0.5125, 0.5694444]
/// ```
/// # PARAMETROS
///  * `&Vec<f32>` Referencia al vector al cual se le desea calcular la varianza
///  * `&f32`      Referencia al valor de la media del vector referenciada
/// 
/// # RETORNO
/// * `[f32, f32]` vector con dos valores f32 `[poblacional , muestral]`
/// 
/// # PANORAMICA
/// 
/// * Devuleve un vector<f32,f32> de un vector referenciado<f32> y la media<f32> ambos referenciados
///   

pub fn varianza(val:&Vec<f32> , med: &f32) -> [f32; 2] {

    let mut var:f32 = 0.0;
    val.into_iter()
        .for_each(|x: &f32|{ 
            var += (x - med).powi(2);
         }); 
    [var /(val.len() as f32), var /(val.len() as f32 - 1f32)]
}

/// DEVUELVE EL VALOR DE MAYOR MAGNITUD DE UN VECTOR
///  
/// # EJEMPLO
/// ```rust
///  
///  let vector: Vec<f32> = vec![1.0, 2.0, 3.0, 1.0, 1.5, 1.5, 1.5, 2.0, 3.0, 1.0];
///      
///  let _maximo: f32 = valor_maximo(&vector);
///     
///  println!("{_maximo:?}");
/// 
/// ```
/// ```bash
/// 3.0
/// ```
/// # PARAMETROS
///  * `&Vec<f32>` Referencia al vector al cual se le desea calcular el valor maximo
/// 
/// # RETORNO
/// * `f32` Valor de mayor magnitud en el vector referenciado
/// 
/// # PANORAMICA
/// 
/// * Devuelve el valor maximo<f32> contenido en un vector<f32> referenciado
///  

pub fn valor_maximo(vec:&Vec<f32>)-> f32{
    let mut maximo: f32 = 0.0;
    if let Some(&valor_maximo) = vec.iter()
            .max_by(|a,b| a.partial_cmp(b).unwrap()){
        maximo = valor_maximo.clone();
        return maximo
    }else{ 
        return maximo
    }
}

/// DEVUELVE EL VALOR DE MENOR MAGNITUD DE UN VECTOR
///  
/// # EJEMPLO
/// ```rust
///  
///  let vector: Vec<f32> = vec![1.0, 2.0, 3.0, 1.0, 1.5, 1.5, 1.5, 2.0, 3.0, 1.0];
///      
///  let _maximo: f32 = valor_minimo(&vector);
///      
///  println!("{_maximo:?}");
/// 
/// ```
/// ```bash
/// 1.0
/// ```
/// # PARAMETROS
///  * `&Vec<f32>` Referencia al vector al cual se le desea calcular el valor mínimo
/// 
/// # RETORNO
/// * `f32` Valor de mayor magnitud en el vector referenciado
/// 
/// # PANORAMICA
/// 
/// * Devuelve el valor mínimo<f32> contenido en un vector<f32> referenciado
///  

pub fn valor_minimo(vec:&Vec<f32>)-> f32{
    let mut minimo: f32 = 0.0;
    if let Some(&valor_minimo) = vec.iter()
            .min_by(|a,b| a.partial_cmp(b).unwrap()){
        minimo = valor_minimo.clone();
        return minimo
    } else{ return minimo }
}

/// DEVUELVE EL CALCULO DEL RANGO DE UN VECTOR 
///  
/// # EJEMPLO
/// ```rust
///  
///  let vector: Vec<f32> = vec![1.0, 2.0, 3.0, 1.0, 1.5, 1.5, 1.5, 2.0, 3.0, 1.0];
///      
///  let _rango: f32 = rango(&vector);
///      
///  println!("{_rango:?}");
/// 
/// ```
/// ```bash
/// 2.0
/// ```
/// # PARAMETROS
///  * `&Vec<f32>` Referencia al vector al cual se le desea calcular el rango
/// 
/// # RETORNO
/// * `f32` valor de la diferencia entre el valor mayor y el valor mínimo
/// 
/// # PANORAMICA
/// 
/// * Devuelve el rango el mayor<f32> - el menor<f32> de un vector<f32> referenciado
///  

pub fn rango(vector:&Vec<f32>)-> f32{
    let maximo = valor_maximo(vector);
    let minimo = valor_minimo(vector);
    return maximo - minimo
}

/// DEVUELVE EL COEFICIENTE DE VARIACIÓN DE UN VECTOR
///  
/// # EJEMPLO
/// ```rust
///  
///  let vector: Vec<f32> = vec![1.0, 2.0, 3.0, 1.0, 1.5, 1.5, 1.5, 2.0, 3.0, 1.0];
///      
///  let _media:f32 = media(&vector);
///  
///  let _dsv_std: f32 = desv_std(&vector, &_media);
///      
///  let _cft_variacioon: f32 = coef_variacion(&_dsv_std, &_media);
///      
///  println!("{_cft_variacioon:?}");
///  
/// ```
/// ```bash
/// 43.12088
/// ```
/// # PARAMETROS
///  * `&f32` Referencia a un f32 con valor de la desviación estandar
///  * `&f32` Referencia a un f32 con valor de la media de un vector
/// 
/// # RETORNO
/// * `f32`   valor del coefciente de variación del par de valores desviacion estandar y media
/// 
/// # PANORAMICA
/// 
/// * Devuelve le calculo del coefeicinte de variacón en un f32
///  

pub fn coef_variacion(desv_estan:&f32, media:&f32)->f32{
    (desv_estan / media) * 100.0
}

/// CALCULA LOS QUARTILES DE UN VECTOR
///  
/// # EJEMPLO
/// ```rust
/// 
///  let vector: Vec<f32> = vec![1.0, 2.0, 3.0, 1.0, 1.5, 1.5, 1.5, 2.0, 3.0, 1.0];
    
///  let _quartiles:Vec<f32> = estadistica::quartiles(&vector);

///  println!("{_quartiles:?}");
///  
/// ```
/// ```bash
/// [1.25, 1.75, 2.5]
/// ```
/// # PARAMETROS
///  *&Vec<f32>  Rferecnia al vector que se le desea calcular los quartiles

/// 
/// # RETORNO
/// *Vec<f32>  Vector con las pociciones del calulo de los quertiles en el vector referenciado
/// // ------------|---------|---------|------------
///               Q1        Q2        Q3   
/// # PANORAMICA
/// 
/// * Retorn un Vector<[f32;3]> con tres f32 que representan v[0] = q1, v[1]= q2, v[2]= q3 los qurtiles en el vector referenciado ordenado acendentemente
///  

pub fn quartiles(vector:&Vec<f32>)->Vec<f32>{

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

    qurtiles
}


/*
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

*/

/// DEVUELVE EL COEFICIENTE DE CORRELACIÓN DE PEARSON
///  
/// # EJEMPLO
/// ```rust
///  
///  let vector: Vec<f32> = vec![1.0, 2.0, 3.0, 1.0, 1.5, 1.5, 1.5, 2.0, 3.0, 1.0];
///
///  let vector2: Vec<f32>  = vec![3.0, 3.0, 3.9, 4.0, 4.0, 4.0, 5.5, 6.0, 3.0, 4.0];
///    
///  let _cft_corr_pearson:f32 = estadistica::fctr_corrlcn_pearson(&vector, &vector2);
///
///  println!("{_cft_corr_pearson:?}");
///
/// ```
/// ```bash
/// -0.10896431
/// ```
/// # PARAMETROS
///  * `&Vec<f32>` Referencia a un vector al cual se le quiera calcular junto con otro el coeficiente de correlación de pearson
///  * `&Vec<f32>` Referencia a un vector al cual se le quiera calcular junto con otro el coeficiente de correlación de pearson
/// 
/// # RETORNO
/// * `f32`        f32 con el valor del coefciente de correlación de pearson
/// 
/// # PANORAMICA
/// 
/// * Devuelve el coeficiente de correlació de Pearson en un f32 de dos vectores<f32> referenciados
///  

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

pub fn vec_ord_asc_usize(vector: &Vec<usize>)->Vec<usize>{

    let mut vec_c= vector.clone();
    vec_c.sort_by(|a,b| a.cmp(b) );

    vec_c
}

#[doc=r"Funcion que devuelve un vector<f32> con el listado de rangos f32, esta funcion es solo para apoyo de la función 'fctr_corrlcn_spearman'"]
fn ajusta_rango(vec_l:&Vec<(usize, f32)>)->Vec<f32>{

    let mut acm: f32 = 1.0;
    let mut acm_tmp: f32 = 0.0;
    let mut vec_pro: Vec<f32> = Vec::new();

    // con un vector que contiene tupoas [(veces que aparace el valor, valor)...()]
    //                                    [(5, 2.3)] aparece 5 veces el numero 3.2
    for i in 0..vec_l.len(){
        
        if vec_l[i].0 != 1 {  // Si tiene más de un valor hace el promedio
            for j in 0..vec_l[i].0{
                acm_tmp += acm;
                acm += 1f32;
            }
            // una vez el promedio hecho lo guarda en orden de numeros repetidos
            for j in 0..vec_l[i].0{ vec_pro.push( acm_tmp as f32/vec_l[i].0 as f32 ) }
            acm_tmp = 0.0;
            continue
        }
        // si no hay más de un elemento guarda el rango que le corresponde
        vec_pro.push(acm);
        acm += 1f32;
    }
    vec_pro
}

/// DEVUELVE EL COEFICIENTE DE CORRELACIÓN DE SPEARMAN
///  
/// # EJEMPLO
/// ```rust
///  
///  let vector: Vec<f32> = vec![1.0, 2.0, 3.0, 1.0, 1.5, 1.5, 1.5, 2.0, 3.0, 1.0];
///
///  let vector2: Vec<f32>  = vec![3.0, 3.0, 3.9, 4.0, 4.0, 4.0, 5.5, 6.0, 3.0, 4.0];
///    
///  let _cft_corr_spearman:f32 = estadistica::fctr_corrlcn_spearman(&vector, &vector2);
///
///  println!("{_cft_corr_pearson:?}");
///
/// ```
/// ```bash
/// -0.115151525
/// ```
/// # PARAMETROS
///  * `&Vec<f32>` Referencia a un vector al cual se le quiera calcular junto con otro el coeficiente de correlación de spearman
///  * `&Vec<f32>` Referencia a un vector al cual se le quiera calcular junto con otro el coeficiente de correlación de spearman
/// 
/// # RETORNO
/// * `f32`        f32 con el valor del coefciente de correlación de spearman
/// 
/// # PANORAMICA
/// 
/// * Devuelve el coeficiente de correlació de Spearman en un f32 de dos vectores<f32> referenciados
///  

pub fn fctr_corrlcn_spearman(val:&Vec<f32>, val2:&Vec<f32>) -> f32 {
    
    // contiene tuplas con # de apariciones y el valor ordenadas de menor a mayor
    // [(5,1.0),...()] 
    let vec_1: Vec<(usize, f32)> = conteo_vec( &vec_ord_asc(val ) );
    let vec_2: Vec<(usize, f32)> = conteo_vec( &vec_ord_asc(val2 ) );

    // Aqui se obteiene los rangos en orden menor a mayor
    let mut vec_pro: Vec<f32> = ajusta_rango(&vec_1);
    let mut vec_pro2: Vec<f32> = ajusta_rango(&vec_2);
    
    // se obtienen los indices del 0 a n
    let mut sorted_indices: Vec<usize> = (0..val.len()).collect();
    let mut sorted_indices2: Vec<usize> = (0..val2.len()).collect();
    
    // vectores que guardarán los rangos que s erestarán al final
    let mut vector_valores = vec![0.0; vec_pro.len()];
    let mut vector_valores2 = vec![0.0; vec_pro.len()];

    // Se ordenan los Indices, dependiendo del valor que indican, se califican los valores
    // y se guradan ese orden los indices
    sorted_indices.sort_by(|&a, &b| val[a].partial_cmp(&val[b]).unwrap());
    sorted_indices2.sort_by(|&a, &b| val2[a].partial_cmp(&val2[b]).unwrap());



    for i in 0..sorted_indices2.len(){
        vector_valores[sorted_indices[i]] = vec_pro[i];
        vector_valores2[sorted_indices2[i]] = vec_pro2[i];
    }

    let mut valor: f32 = 0.0;
    for i in 0..vector_valores.len(){
        valor += (vector_valores[i] - vector_valores2[i]).powi(2);
    }

    let n= val.len();
    1.0 - (6.0 * valor) / ((n as f32) * ((n as f32).powi(2) - 1.0))
    
    }


// UTILSS

/// DEVUELVE LA MATRIZ TRANSPUESTA DE UNA MATRIZ DE n x n
///  
/// # EJEMPLO
/// ```rust
///  
///  let matriz = vec![
///     vec![1.0, 2.0, 3.0],
///     vec![4.0, 5.0, 6.0],
///     vec![7.0, 8.0, 9.0] ];
///      
///  let _transpuesta:Vec<Vec<f32>> = estadistica::transpuestafn(&matriz);
///  
///  for vector in _transpuesta.iter(){ println!("{vector:?}") }
///
/// ```
/// ```bash
/// [1.0, 4.0, 7.0]
/// [2.0, 5.0, 8.0]
/// [3.0, 6.0, 9.0]
/// ```
/// # PARAMETROS
///  * `&Vec<Vec<f32>>` Referencia a una matriz de vectores al cual se le quiere sacar su transpuesta
/// 
/// # RETORNO
///  * `&Vec<Vec<f32>>` Vector de vectores acomodados de forma transpuesta con referencia a la matriz ingresada
/// 
/// # PANORAMICA
/// 
/// * Devuelve un vector<Vec<Vec<f32>> de vectores referenciado, asemejando una matriz, esta matriz devuleta será transpuesta
///  
#[doc=r""]
pub fn transpuestafn( arr:&Vec<Vec<f32>> )->Vec<Vec<f32>>{

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


// [0.2, 1.0, 1.0, 1.0, 1.5, 1.5, 1.5, 2.0, 2.0, 3.0, 0.1];
// [2.2, 3.0, 4.0, 5.0, 2.5, 3.5, 4.5, 5.0, 3.0, 4.0, 0.2];
// [0.2, 1.0, 1.0, 1.0, 1.5, 1.5, 1.5, 2.0, 2.0, 3.0, 
//  3.0, 3.0, 3.0, 3.9, 4.0, 4.0, 4.0, 4.0, 5.5, 6.0, 
//  1.0, 2.0, 3.0, 1.0, 1.5, 1.5, 1.5, 2.0, 3.0, 1.0, 
//  3.0, 6.0, 5.5, 3.9, 0.2, 4.0, 4.0, 3.0, 4.0, 4.0
// vec![0.2, 1.0, 1.0, 1.0, 1.5, 1.5, 1.5, 2.0, 2.0, 3.0, 3.0, 3.0, 3.0];
// vec![16.000, 18.000, 18.000, 20.000, 21.000, 21.000, 22.000, 23.000, 24.000, 24.000, 25.000, 26.000, 28.000];
// 16.000, 18.000, 18.000, 20.000, 21.000, 21.000, 22.000, 23.000, 24.000, 24.000, 25.000, 26.000, 28.000
// vec![0.2, 1.0, 1.0, 1.0, 1.5, 1.5, 1.5, 2.0, 2.0, 3.0, 3.0, 3.0, 3.0, 3.9, 4.0, 4.0, 4.0, 4.0, 5.5, 6.0, 1.0, 2.0, 3.0, 1.0, 1.5, 1.5, 1.5, 2.0, 3.0, 1.0, 3.0, 6.0, 5.5, 3.9, 0.2, 4.0, 4.0, 3.0, 4.0, 4.0, 3.0  ];