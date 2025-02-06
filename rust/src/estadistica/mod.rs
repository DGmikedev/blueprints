    pub fn media(_val: &[f32; 5]) -> f32 {
        let suma = _val.into_iter().fold(0f32, |x: f32, y: &f32| x + y);
        suma / _val.len() as f32
    }

    pub fn varianza(val: &[f32; 5], med: &f32) -> [f32; 2] {
        let mut vec: Vec<f32> = Vec::new();

        val.into_iter()
            .for_each(|x: &f32| vec.push((x - med).powi(2)));

        let sum = vec.into_iter().fold(0f32, |x: f32, y: f32| x + y);

        [sum / val.len() as f32, sum / val.len() as f32 - 1f32]
    }
    
    pub fn desviacion_estd(var_pob:&f32)->f32{
     var_pob.sqrt()
    }

    pub fn factor_correlacion(val:&[f32;5], val2:&[f32;5]) -> f32 {
    
        let mut vec_x: Vec<f32> = Vec::new();
        let mut vec_y: Vec<f32> = Vec::new();
        
        let mut sum_xe2: f32 = 0.0;
        let mut sum_ye2: f32 = 0.0;
        
        let mut xy:f32 = 0.0;
        
        // # pares de datos
        let n = val.len() as f32;
        
        // media de los vectores
        let med_x:f32 =  val.into_iter().fold(0f32,|x:f32, y:&f32|x+y) / n;
        let med_y:f32 = val2.into_iter().fold(0f32,|x:f32, y:&f32|x+y) / n;
        
        // diferencias en vector
        for i in val.into_iter(){ vec_x.push(i - med_x) }
        for i in val2.into_iter(){ vec_y.push(i - med_y ) }
        
        // sum del producto de vectores  € (x-x^p)(y-y^p)
        for i in 0..n as i32{ xy += vec_x[i as usize] * vec_y[i as usize] }
        
        // sumatoria de cuadrados
        vec_x.clone().into_iter().for_each(|x:f32|{
            sum_xe2 += x.powi(2)
        });
        
        vec_y.clone().into_iter().for_each(|y:f32|{
            sum_ye2 += y.powi(2)
        });
        
        // aplicando formula
        let res = xy / (sum_xe2 * sum_ye2).sqrt();
        
        res
        
        }
