    pub fn media(_val: &[f32; 8]) -> f32 {
        let suma = _val.into_iter().fold(0f32, |x: f32, y: &f32| x + y);
        suma / _val.len() as f32
    }

    pub fn varianza(val: &[f32; 8], med: &f32) -> [f32; 2] {

        let mut var:f32 = 0.0;

        val.into_iter()
            .for_each(|x: &f32| var += (x - med).powi(2) ); 

        [var / val.len() as f32, var / val.len() as f32 - 1f32]
    }
    
    pub fn desviacion_estd(var_pob:&f32)->f32{
     var_pob.sqrt()
    }

    pub fn factor_correlacion(val:&[f32;8], val2:&[f32;8]) -> f32 {
    
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

        pub fn coeficiente_de_variacion(desv_estan:&f32, media:&f32)->f32{
            (desv_estan / media) * 100.0
        }

        pub fn correlacion_matriz(matrx:&Vec<Vec<f32>>)->Vec<Vec<f32>>{          //Vec<Vec<f32>>{

            let n = matrx.len();
            let mut transposed = vec![vec![0.0; n]; n];
        
            for i in 0..n {
                for j in 0..n {
                    transposed[j][i] = matrx[i][j];
                }
            }

            
            transposed
            /* let mut columnas:Vec<f32> = Vec::new();
            let mut _contendor:Vec<Vec<f32>> = Vec::new();
            let mut acm:usize = 0;
            let mut acm2:usize = 0;

            for i in matrx.clone().into_iter(){

                for j in i.into_iter(){
                    _contendor[acm][acm2].push(j);
                    acm2 += 1;
                }
                acm += 1;
                
                
            } */

        }


        /*

La matriz de correlación es una tabla cuadrada donde cada celda muestra el coeficiente de correlación de Pearson (ρρ) entre dos variables. Se usa para analizar la relación lineal entre múltiples variables.
Pasos para calcular una matriz de correlación 8×8 manualmente
1. Construir la matriz de datos (XX)

Supongamos que tenemos 8 variables (X1,X2,...,X8X1​,X2​,...,X8​), y cada una tiene 8 valores (observaciones).

Ejemplo de matriz de datos (XX):
X=[10151220182225301218142219242833913101815202327111613211723263181291714192226131714232025293414191524212730361014111916212429]
X=
​10129118131410​1518131612171914​121410139141511​2022182117232419​1819151714202116​2224202319252721​2528232622293024​3033273126343629​
​

Cada columna representa una variable diferente, y cada fila representa una observación.
2. Calcular la media (μμ) de cada variable

Para cada columna (variable XiXi​), sumamos los valores y dividimos entre 8 (número de observaciones).

Ejemplo para X1X1​:
μX1=10+12+9+11+8+13+14+108=878=10.875
μX1​​=810+12+9+11+8+13+14+10​=887​=10.875

Repetimos para todas las columnas.
3. Calcular la desviación estándar (σσ) de cada variable

La fórmula es:
σXi=∑(Xi−μXi)2N
σXi​​=N∑(Xi​−μXi​​)2​
​

Ejemplo para X1X1​:
σX1=(10−10.875)2+(12−10.875)2+...+(10−10.875)28
σX1​​=8(10−10.875)2+(12−10.875)2+...+(10−10.875)2​
​

Se repite para cada variable.
4. Calcular el coeficiente de correlación (ρρ) entre cada par de variables

El coeficiente de correlación de Pearson entre dos variables XiXi​ y XjXj​ se define como:
ρXi,Xj=∑(Xi−μXi)(Xj−μXj)N⋅σXi⋅σXj
ρXi​,Xj​​=N⋅σXi​​⋅σXj​​∑(Xi​−μXi​​)(Xj​−μXj​​)​

Ejemplo para la correlación entre X1X1​ y X2X2​:
ρX1,X2=(10−10.875)(15−16.625)+(12−10.875)(18−16.625)+…8⋅σX1⋅σX2
ρX1​,X2​​=8⋅σX1​​⋅σX2​​(10−10.875)(15−16.625)+(12−10.875)(18−16.625)+…​

Este proceso se repite para todas las combinaciones de pares de variables.
5. Construcción de la matriz de correlación RR

La matriz de correlación 8×8 final tiene la forma:
R=[1ρ1,2ρ1,3ρ1,4ρ1,5ρ1,6ρ1,7ρ1,8ρ2,11ρ2,3ρ2,4ρ2,5ρ2,6ρ2,7ρ2,8ρ3,1ρ3,21ρ3,4ρ3,5ρ3,6ρ3,7ρ3,8ρ4,1ρ4,2ρ4,31ρ4,5ρ4,6ρ4,7ρ4,8ρ5,1ρ5,2ρ5,3ρ5,41ρ5,6ρ5,7ρ5,8ρ6,1ρ6,2ρ6,3ρ6,4ρ6,51ρ6,7ρ6,8ρ7,1ρ7,2ρ7,3ρ7,4ρ7,5ρ7,61ρ7,8ρ8,1ρ8,2ρ8,3ρ8,4ρ8,5ρ8,6ρ8,71]
R=
​1ρ2,1​ρ3,1​ρ4,1​ρ5,1​ρ6,1​ρ7,1​ρ8,1​​ρ1,2​1ρ3,2​ρ4,2​ρ5,2​ρ6,2​ρ7,2​ρ8,2​​ρ1,3​ρ2,3​1ρ4,3​ρ5,3​ρ6,3​ρ7,3​ρ8,3​​ρ1,4​ρ2,4​ρ3,4​1ρ5,4​ρ6,4​ρ7,4​ρ8,4​​ρ1,5​ρ2,5​ρ3,5​ρ4,5​1ρ6,5​ρ7,5​ρ8,5​​ρ1,6​ρ2,6​ρ3,6​ρ4,6​ρ5,6​1ρ7,6​ρ8,6​​ρ1,7​ρ2,7​ρ3,7​ρ4,7​ρ5,7​ρ6,7​1ρ8,7​​ρ1,8​ρ2,8​ρ3,8​ρ4,8​ρ5,8​ρ6,8​ρ7,8​1​
​

    Diagonal principal: Siempre tiene valores de 11, porque la correlación de una variable consigo misma es 1.
    Valores fuera de la diagonal: Son los coeficientes de correlación entre pares de variables.
    Matriz simétrica: ρi,j=ρj,iρi,j​=ρj,i​, porque la correlación es la misma en ambas direcciones.

Ejemplo de matriz de correlación con valores hipotéticos

Supongamos que los cálculos nos dieron estos valores:
R=[10.850.720.650.800.900.500.600.8510.780.680.820.880.550.620.720.7810.750.790.850.480.580.650.680.7510.720.780.450.520.800.820.790.7210.860.530.640.900.880.850.780.8610.570.670.500.550.480.450.530.5710.750.600.620.580.520.640.670.751]
R=
​10.850.720.650.800.900.500.60​0.8510.780.680.820.880.550.62​0.720.7810.750.790.850.480.58​0.650.680.7510.720.780.450.52​0.800.820.790.7210.860.530.64​0.900.880.850.780.8610.570.67​0.500.550.480.450.530.5710.75​0.600.620.580.520.640.670.751​
​
Conclusión

Hacer una matriz de correlación 8×8 manualmente implica:

    Calcular la media de cada variable.
    Calcular la desviación estándar de cada variable.
    Calcular los coeficientes de correlación entre cada par de variables.
    Organizar los valores en la matriz de correlación.

🔹 Aplicaciones: Se usa en estadísticas, ciencia de datos, finanzas, psicología, y más.

¿Necesitas más detalles o ejemplos? 😊
         * 
         */