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
