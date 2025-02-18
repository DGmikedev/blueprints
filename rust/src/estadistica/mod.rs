use std::{result, vec};

macro_rules! pr_Vv{ ($vec:expr)=>{ for i in $vec.into_iter(){ println!("{:?}", i) }  } }
macro_rules! pr_v{  ($vec:expr)=>{  println!("{:?}", $vec)  }  }
macro_rules! pr_separador{  ($txt:expr)=>{ println!("\n-- {} ----------------------------------\n", $txt) } }



    pub fn media(_val: &Vec<f32>) -> f32 {
        let suma = _val.into_iter().fold(0f32, |x: f32, y: &f32| x + y);
        suma / _val.len() as f32
    }

    pub fn varianza(val:&Vec<f32> , med: &f32) -> [f32; 2] {

        let mut var:f32 = 0.0;
        val.into_iter()
            .for_each(|x: &f32|{ 
                var += (x - med).powi(2);
             }); 
        [var /(val.len() as f32), var /(val.len() as f32 - 1f32)]
    }

    pub fn desvisacion_estd_por_vector(val:&Vec<f32> , med: &f32) -> f32 {

        let mut var:f32 = 0.0;
        val.into_iter()
            .for_each(|x: &f32|{ 
                var = var + (x - med).powi(2);
            }); 
            var = var/(val.len() as f32 - 1f32);
            var = var.sqrt();
            var
        
    }
    
    pub fn desviacion_estd(var_pob:&f32)->f32{
     var_pob.sqrt()
    }

    pub fn factor_correlacion(val:&Vec<f32>, val2:&Vec<f32>) -> f32 {
    
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


        fn transpuestafn( arr:&Vec<Vec<f32>> )->Vec<Vec<f32>>{

            let high:usize = arr.len();
            let with:usize = arr[0].len();
            let mut arr_return: Vec<Vec<f32>> = vec![vec![0.0; high]; with];
                for (i, vect) in arr.into_iter().enumerate(){ 
                    for j in 0..vect.len(){
                        arr_return[j][i] = arr[i][j]; 
                    }
                }
            //for i in arr_return.into_iter(){ println!("{:?}", i) }
            arr_return
        }



        pub fn correlacion_matriz(matrx:&Vec<Vec<f32>>)->Vec<Vec<f32>>{

            let mut vector_medias: Vec<f32> = Vec::new();
            let mut vector_varianzas:Vec<f32> = Vec::new();
            let mut vector_desviaciones_std:Vec<f32> = Vec::new();
            let mut vector_temporal:Vec<f32> = Vec::new();
            let mut vector_temporal2: Vec<f32> = Vec::new();
            let mut matriz_covarianzas_por_par:Vec<Vec<f32>> = Vec::new();
            let mut matriz_de_correlacion:Vec<Vec<f32>> = Vec::new();
            let mut matriz_de_correlacion2:Vec<Vec<f32>> = Vec::new();
            let mut vector_covarianzas_por_par:Vec<f32> = Vec::new();
            let mut matriz_temporal:Vec<Vec<f32>> = Vec::new();
            let mut matriz_temporal2:Vec<Vec<f32>> = Vec::new();

            let mut acm:usize = 0;
            let mut skp:usize = 0;
            let mut acm_f32: f32 = 0.0;
            let mut acm2_f32: f32 = 0.0;
            let mut tupla_tmp_f32:[f32;2] = [0.0;2];

            // transpuesta de la original 
            // [ [][][][] ]
            let transpuesta:Vec<Vec<f32>> = transpuestafn(matrx);

            // Vector de medias
            transpuesta.clone().into_iter().for_each(|vec:Vec<f32>|{
                vector_medias.push( media(&vec));
            });

            // desviacion estandar inmediata - sin usar la función 
            // para calcular previamente la varianza
            acm = 0;
            transpuesta.clone().into_iter().for_each(|x:Vec<f32>|{
                acm2_f32 = desvisacion_estd_por_vector(&x, &vector_medias[acm]);
                vector_desviaciones_std.push(acm2_f32);
                acm += 1;
            });


//          acm = 0;
//             // vector de varianzas
//             transpuesta.clone().into_iter().for_each(|vec: Vec<f32>|{
//                 tupla_tmp_f32 = varianza(&vec, &vector_medias[acm]);
//                 vector_varianzas.push(tupla_tmp_f32[1]);
//                 acm += 1;
//             });

            
            // Vector de desviaciones estandar
            vector_varianzas.clone().into_iter().for_each(|varianza|{
                vector_desviaciones_std.push( desviacion_estd(&varianza) );
            });


           
            

            
            acm = 0;
            
            // Calcular el Vector de covarianzas por par de columnas
            // col 1,2 - 2,3 - 3,4 - 4,5
            // col 1,3 - 2,4 - 3,5 - 4,6
            // cal 1,4 - 2,5 - 3,6 - 4,7

            // 1.- restar la media al valor original
            matrx.clone().into_iter().for_each(|x:Vec<f32>|{
                x.into_iter().for_each(|y:f32|{
                   //  tmp_i = y - media_v[acm];
                    vector_temporal.push(y - vector_medias[acm]);
                    acm += 1;
                });
                acm = 0;
                matriz_temporal.push(vector_temporal.clone());
                vector_temporal.clear()
            });



            // 2.- multiplicar clumnas


            pr_separador!("vector de medias");
            pr_v!(vector_medias.clone());

            pr_separador!("matriz_temporal");
            pr_Vv!(matriz_temporal.clone());  

            pr_separador!("----");
            skp = 1;




            for i in 0 .. matriz_temporal.len() - 1 {
                for k in 0 .. matriz_temporal.len(){
                    for j in 0 .. matriz_temporal.len() - skp {
                        if j >= matriz_temporal.len() { continue } 
                        //println!("[i{i}-k{k}-j{j}]");
                        // println!("[i{i}-k{k}-j{j}] - {}*{} = {}", matriz_temporal[k][j] , matriz_temporal[k][j + skp], matriz_temporal[k][j] * matriz_temporal[k][j + skp]);
                        vector_temporal.push(matriz_temporal[k][j] * matriz_temporal[k][j + skp]);
                        }
                        matriz_temporal2.push(vector_temporal.clone());
                        vector_temporal.clear();
                        // pr_separador!("----");
                }

                acm_f32 = 0.0;

                pr_separador!("Vector temporal previo");
                pr_v!(vector_temporal);
                pr_separador!("matriz temporal");

                pr_Vv!(matriz_temporal2.clone());

                pr_separador!("matriz temporal transpuesta");
                matriz_temporal2 = transpuestafn(&matriz_temporal2);
                pr_Vv!(matriz_temporal2.clone());

                matriz_temporal2.clone().into_iter().for_each(|x:Vec<f32>|{
                    acm_f32 = acm_f32 + x.into_iter().fold(0f32, |x:f32, y:f32| x + y ); // ) / (matrx.len() as f32 - 1.0); 
                    vector_temporal.push(acm_f32.clone());
                    acm_f32 = 0.0;
                });

                acm_f32 = 0.0;

                pr_separador!("... v-tmp");

                pr_v!(vector_temporal);

                matriz_de_correlacion.push(vector_temporal.clone());

                pr_separador!("...");


                matriz_temporal2.clear();
                vector_temporal.clear();

                skp = skp + 1;


            }


            pr_Vv!(matriz_de_correlacion.clone());
            println!(".");
            pr_Vv!( transpuestafn(&matriz_de_correlacion) );
            println!(".");
            pr_v!(vector_desviaciones_std);
            
            matriz_de_correlacion = transpuestafn(&matriz_de_correlacion);
            
/*
[336.19998, -493.2, 19.399994, 102.0]
[-1320.2001, 2079.4, 212.99998, 0.0]
[-1920.4001, -1238.0001, 0.0, 0.0]
[889.0, 0.0, 0.0, 0.0]
.
[16.4, 26.4, 29.6, 38.8, 29.0]

336.19998        -493.2
---------  ,  -------------, 
16.4 x26.4     16.4 x 29.6




*/



            return matriz_covarianzas_por_par;  



            skp = 1;

            for i in 0 .. matriz_temporal.len() - 1 {
                for k in 0 .. matriz_temporal.len(){
                    for j in 0 .. matriz_temporal.len() - skp {
                        if j >= matriz_temporal.len() { continue } 
                            vector_temporal.push( matriz_temporal[k][j] * matriz_temporal[k][j + skp] );
                        }
                    matriz_covarianzas_por_par.push( vector_temporal.clone() );
                    vector_temporal.clear();
                }

                acm_f32 = 0.0;

                for (index, i) in matriz_covarianzas_por_par.clone().into_iter().enumerate(){
                    for (index2,j) in i.into_iter().enumerate(){
                        if index2 > 0 { continue }
                        acm_f32 = acm_f32 + matriz_covarianzas_por_par[index][index2];
                    }
                }

                vector_temporal2.push(acm_f32);
                
                // 3.- tranponer 
                matriz_covarianzas_por_par = transpuestafn(&matriz_covarianzas_por_par.clone());

//                pr_separador!("Matriz de matriz_covarianzas_por_par: ");
//                pr_Vv!(matriz_covarianzas_por_par.clone());


                for i in matriz_covarianzas_por_par.clone().into_iter(){
                    acm2_f32 = i.into_iter().fold(0f32, |x:f32, y:f32| x + y);  // 4.- sumar columnas
                    // pr_separador!(acm2_f32);
                    vector_covarianzas_por_par.push(acm2_f32 / matrx.len() as f32  - 1.0  );    // 5.- dividir entre largo de la matriz original
                }
                matriz_covarianzas_por_par.clear();
                skp += 1;
            }   

            
            

            vector_temporal.clear();

            for i in 0..matrx.len(){
                for j in 0..matrx.len(){
                    if i == j { 
                        vector_temporal.push(1.0); 
                        continue
                    }

                   // println!("{} / {} x {}", vector_covarianzas_por_par[i],vector_desviaciones_std[i], vector_desviaciones_std[j] );
                    vector_temporal.push( vector_covarianzas_por_par[i] / ( vector_desviaciones_std[i] * vector_desviaciones_std[j] ) );
                }

                matriz_de_correlacion.push(vector_temporal.clone());
                vector_temporal.clear();
            }

//             pr_separador!("vector de covarianzas");
//             pr_v!(vector_covarianzas_por_par);
// 
//             pr_separador!("vector de desviaciones estandar");
//             pr_v!(vector_desviaciones_std);

           

            
        //     matriz_de_correlac




             pr_separador!("vector de medias");
             pr_v!(vector_medias);
//             pr_separador!();
//             pr_v!(vector_varianzas);
             pr_separador!("desviacion estandar");
             pr_v!(vector_desviaciones_std);
//             pr_separador!();
//             pr_v!(acm);
//              pr_separador!("Vector temporal = Con la resta de la media por variable a plicado a las columnas"); 
//              pr_Vv!(matriz_temporal);

             // pr_separador!("matriz_cov_por_par");
             // pr_Vv!(matriz_covarianzas_por_par);
        //      pr_separador!("Vector de covarianzas por par de columnas");
        //         pr_Vv!(vector_covarianzas_por_par);
            // let n = matrx.len();
            // let mut transpuesta: Vec<Vec<f32>> = vec![vec![0.0; n]; n];
            // let mut media_v:Vec<f32> = Vec::new(); 
            // let mut varianza_v:Vec<f32> = Vec::new();
            // let mut covarianza_par_var: Vec<Vec<f32>> = Vec::new();
            // let mut acm: usize = 0;
            // let mut acm_f32: f32 = 0.0;
            // let mut desviacion_std_v:Vec<f32> = Vec::new();
            // let mut resultado: Vec<Vec<f32>> = Vec::new();
            // let mut tmp_v: Vec<f32> = Vec::new();
            // let mut tmp_v2: Vec<f32> = Vec::new();
            // let mut tmp_i:f32 = 0.0;
            // let mut tmp_tpl:[f32;2] = [0.0,0.0];
            // let mut vec_varianzas:Vec<f32> = Vec::new();
            // let mut step:usize = 0;


            // test array
            // let mtrx2:Vec<Vec<f32>> = vec![
            //     vec![10.0, 18.0, 14.0, 21.0, 19.0, 2.0,  12.0, 4.0], vec![12.0, 13.0, 10.0, 82.0, 15.0, 24.0, 52.0, 30.0],
            //     vec![10.0, 18.0, 14.0, 21.0, 19.0, 2.0,  12.0, 4.0], vec![12.0, 13.0, 10.0, 82.0, 15.0, 24.0, 52.0, 30.0]
            // ];


/*
            // transpuesta
            for i in 0..n {
                for j in 0..n {
                    transpuesta[j][i] = matrx[i][j];
                }
            }

            
            
            // media
            for i in transpuesta.clone().into_iter(){
                media_v.push( media(&i) );
            }

            // varianza
            for (indice, valor) in transpuesta.clone().into_iter().enumerate(){
                tmp_tpl =  varianza( &valor, &media_v[indice]);
                varianza_v.push(tmp_tpl[0])
            }

            // desviación estandar
            for i in varianza_v.clone().into_iter(){
                desviacion_std_v.push( desviacion_estd(&i) );
            }

            // num - su_media_por_columna
            
            matrx.clone().into_iter().for_each(|x:Vec<f32>|{
                x.into_iter().for_each(|y:f32|{
                    tmp_i = y - media_v[acm];
                    tmp_v.push(tmp_i);
                    acm += 1;
                });
                acm = 0;
                resultado.push(tmp_v.clone());
                tmp_v.clear()
            });


            let mut skp:usize = 1;

            for i in 0 .. resultado.len() - 1 {

                for k in 0 .. resultado.len(){
                    
                    for j in 0 .. resultado.len() - skp {

                        if j >= resultado.len() { continue } 

                    //    // println!("i:{i} k:{k} j:{j} skp:{skp} skp2{skp_2} ");
                    //      println!("{:?} x {:?} = {:?}",resultado[k][j], 
                    //                                               resultado[k][j + skp], 
                    //                                               resultado[k][j] * resultado[k][j + skp]);
                        tmp_v.push( resultado[k][j] * resultado[k][j + skp] );
                       
                    }

                    covarianza_par_var.push( tmp_v.clone() );

                    tmp_v.clear();
                    acm_f32 = 0.0;
                }

                
                acm_f32 = 0.0;
                for (index, i) in covarianza_par_var.clone().into_iter().enumerate(){
            // 'v   
                    for (index2,j) in i.into_iter().enumerate(){
                
                        if index2 > 0 { continue } // ->
                        acm_f32 = acm_f32 + covarianza_par_var[index][index2];
                    //    println!("** {} - {}", covarianza_par_var[index][index2], acm_f32);
                    }
                    // tmp_v2.push(acm_f32);
                   // acm_f32 = 0.0;
                }
            //    println!("push -> {acm_f32}");
                tmp_v2.push(acm_f32);
                
                tmp_v.clear();
                
                covarianza_par_var.push( tmp_v.clone() );

                skp += 1;

            }
*/
        
        //     for i  in covarianza_par_var.into_iter(){
        //         println!(":{:?}", i)
        //     }
// 
        //     for i  in tmp_v2.into_iter(){
        //         println!("--{:?}", i)
        //     }


// EN_USO
 /*

            let vecr:[f32;8] = [-987.34375,
            324.53125,
            441.90625,
            -1273.5938,
            271.03125,
            88.40625,
            1037.4063,
            -0.59375];

            let v:f32 = vecr.into_iter().fold(0f32,|x:f32, y:f32| x + y );
        //    println!("SUMA:::: {v}");
*/

  
            return transpuesta;



}


       //      for i in covarianza_par_var.clone().into_iter(){
        //          println!("{i:?}");
        //     }
            

/*


:[44.375,  -987.34375, -917.8125, 315.5625, 210.375, 144.375, 162.89767]    
:[-92.125, 324.53125, 435.9375, 340.3125, 90.75, 54.75, 140.62767]
:[-29.625, 441.90625, -1264.1875, 501.4375, 230.75, -289.25, 282.70016]    
:[444.125, -1273.5938, -914.0625, -4.6875, -2.0, -226.0, 132.93391]        
:[-95.875, 271.03125, 615.5625, -330.8125, -227.125, 278.875, -419.38858]  
:[60.375,  88.40625, 176.8125, 150.9375, -0.0, -0.0, -941.9373]
:[-11.125, 1037.4063, -116.5625, -74.6875, -89.625, -209.625, -1775.6112]  
:[-2.125,  -0.59375, 17.8125, 8.4375, -1.125, 10.875, 96.556404]
:[]
:[-139.0625, 292.875, -1063.8281, 181.5, 251.01563, 136.52376]
:[-106.5625, 376.875, 293.04688, 135.0, 138.01563, 92.46751]
:[-33.5625, -1115.875, -198.57813, 1469.0, -98.734375, -660.6925]
:[-207.1875, 1959.375, 3.046875, 600.0, 1.765625, -150.58002]
:[-119.4375, 494.125, -181.45313, 770.5, -119.734375, -795.5412]
:[161.4375, 66.125, 201.79688, -0.0, 277.26563, 0.0]
:[-23.3125, -55.625, 1392.9219, 7.5, 2087.5156, 76.23375]
:[20.1875, -1.875, 2.671875, -7.5, -12.234375, 8.87875]
:[]
:[41.25, 339.46875, -611.875, 216.5625, 237.36517]
:[-123.75, 253.34375, 116.25, 205.3125, 233.09517]
:[84.75, -175.28125, -581.75, -628.5625, -225.52486]
:[318.75, -6.53125, -390.0, -529.6875, 1.1764064]
:[-217.75, -145.65625, 422.625, 406.1875, 341.5639]
:[120.75, 75.46875, -0.0, 242.9375, -585.22736]
:[1.25, 664.71875, -139.875, -174.6875, -759.16113]
:[63.75, -0.28125, -2.375, -81.5625, -9.988594]
:[]
:[47.8125, 195.25, -730.0781, 204.78564]
:[-83.1875, 100.5, 176.79688, 346.75314]
:[13.3125, -513.5, 248.92188, -1435.7357]
:[-1.0625, 836.0, 344.29688, -352.9219]
:[64.1875, 339.25, 222.79688, -1158.7231]
:[137.8125, -0.0, 324.79688, -512.7706]
:[-14.9375, -66.75, 3257.9219, 63.52813]
:[9.5625, 0.25, -25.828125, -66.59062]
:[]
:[27.5, 232.96875, -690.37585]
:[-33.0, 152.84375, 298.593]
:[39.0, 219.71875, 568.5767]
:[136.0, -738.03125, 229.39925]
:[-149.5, 178.84375, -635.56824]
:[-0.0, 121.46875, -685.552]
:[1.5, 1554.7188, -1184.7996]
:[-8.5, 2.71875, -21.08703]
:[]
:[32.8125, 220.29971]
:[-50.1875, 258.13846]
:[-16.6875, 501.87222]
:[-120.0625, -491.73785]
:[-78.8125, -510.18405]
:[221.8125, -256.3853]
:[-34.9375, -565.4003]
:[-92.4375, 2.2196875]
:[]
:[31.028128]
:[-84.76188]
:[-38.116875]
:[-79.995636]
:[224.82687]
:[-468.18185]
:[12.705626]
:[-75.469376]
:[]


*/




/*

-2.5 x -17.75 = 44.375       - 01    5.5 x -16.75 = -92.125            1.5 x -19.75 = -29.625            8.5 x 52.25 = 444.125
-17.75 x 55.625 = -987.34375 - 12   -16.75 x -19.375 = 324.53125       -19.75 x -22.375 = 441.90625      52.25 x -24.375 = -1273.5938
55.625 x -16.5 = -917.8125   - 23   -19.375 x -22.5 = 435.9375         -22.375 x 56.5 = -1264.1875       -24.375 x 37.5 = -914.0625
-16.5 x -19.125 = 315.5625   - 34   -22.5 x -15.125 = 340.3125         56.5 x 8.875 = 501.4375           37.5 x -0.125 = -4.6875
-19.125 x -11.0 = 210.375    - 45   -15.125 x -6.0 = 90.75             8.875 x 26.0 = 230.75             -0.125 x 16.0 = -2.0
-11.0 x -13.125 = 144.375    - 56   -6.0 x -9.125 = 54.75              26.0 x -11.125 = -289.25          16.0 x -14.125 = -226.0
-13.125 x -12.41 = 162.89767 - 67   -9.125 x -15.411251 = 140.62767    -11.125 x -25.411251 = 282.70016  -14.125 x -9.411251 = 132.93391

6.5 x -14.75 = -95.875              -10.5 x -5.75 = 60.375           -0.5 x 22.25 = -11.125             -8.5 x 0.25 = -2.125
-14.75 x -18.375 = 271.03125       -5.75 x -15.375 = 88.40625        22.25 x 46.625 = 1037.4063         0.25 x -2.375 = -0.59375
-18.375 x -33.5 = 615.5625         -15.375 x -11.5 = 176.8125        46.625 x -2.5 = -116.5625          -2.375 x -7.5 = 17.8125
-33.5 x 9.875 = -330.8125          -11.5 x -13.125 = 150.9375        -2.5 x 29.875 = -74.6875           -7.5 x -1.125 = 8.4375
9.875 x -23.0 = -227.125           -13.125 x 0.0 = -0.0              29.875 x -3.0 = -89.625            -1.125 x 1.0 = -1.125
-23.0 x -12.125 = 278.875          0.0 x -21.125 = -0.0              -3.0 x 69.875 = -209.625           1.0 x 10.875 = 10.875
-12.125 x 34.58875 = -419.38858    -21.125 x 44.58875 = -941.9373    69.875 x -25.411251 = -1775.6112   10.875 x 8.87875 = 96.556404

[44.375, -987.34375, -917.8125, 315.5625, 210.375, 144.375, 162.89767]
[-139.0625, 292.875, -1063.8281, 181.5, 251.01563, 136.52376]
[41.25, 339.46875, -611.875, 216.5625, 237.36517]
[47.8125, 195.25, -730.0781, 204.78564]
[27.5, 232.96875, -690.37585]
[32.8125, 220.29971]
[31.028128]
[-92.125, 324.53125, 435.9375, 340.3125, 90.75, 54.75, 140.62767]
[-106.5625, 376.875, 293.04688, 135.0, 138.01563, 92.46751]
[-123.75, 253.34375, 116.25, 205.3125, 233.09517]
[-83.1875, 100.5, 176.79688, 346.75314]
[-33.0, 152.84375, 298.593]
[-50.1875, 258.13846]
[-84.76188]
[-29.625, 441.90625, -1264.1875, 501.4375, 230.75, -289.25, 282.70016]
[-33.5625, -1115.875, -198.57813, 1469.0, -98.734375, -660.6925]
[84.75, -175.28125, -581.75, -628.5625, -225.52486]
[13.3125, -513.5, 248.92188, -1435.7357]
[39.0, 219.71875, 568.5767]
[-16.6875, 501.87222]
[-38.116875]
[444.125, -1273.5938, -914.0625, -4.6875, -2.0, -226.0, 132.93391]
[-207.1875, 1959.375, 3.046875, 600.0, 1.765625, -150.58002]
[318.75, -6.53125, -390.0, -529.6875, 1.1764064]
[-1.0625, 836.0, 344.29688, -352.9219]
[136.0, -738.03125, 229.39925]
[-120.0625, -491.73785]
[-79.995636]
[-95.875, 271.03125, 615.5625, -330.8125, -227.125, 278.875, -419.38858]
[-119.4375, 494.125, -181.45313, 770.5, -119.734375, -795.5412]
[-217.75, -145.65625, 422.625, 406.1875, 341.5639]
[64.1875, 339.25, 222.79688, -1158.7231]
[-149.5, 178.84375, -635.56824]
[-78.8125, -510.18405]
[224.82687]
[60.375, 88.40625, 176.8125, 150.9375, -0.0, -0.0, -941.9373]
[161.4375, 66.125, 201.79688, -0.0, 277.26563, 0.0]
[120.75, 75.46875, -0.0, 242.9375, -585.22736]
[137.8125, -0.0, 324.79688, -512.7706]
[-0.0, 121.46875, -685.552]
[221.8125, -256.3853]
[-468.18185]
[-11.125, 1037.4063, -116.5625, -74.6875, -89.625, -209.625, -1775.6112]
[-23.3125, -55.625, 1392.9219, 7.5, 2087.5156, 76.23375]
[1.25, 664.71875, -139.875, -174.6875, -759.16113]
[-14.9375, -66.75, 3257.9219, 63.52813]
[1.5, 1554.7188, -1184.7996]
[12.705626]
[-2.125, -0.59375, 17.8125, 8.4375, -1.125, 10.875, 96.556404]
[20.1875, -1.875, 2.671875, -7.5, -12.234375, 8.87875]
[63.75, -0.28125, -2.375, -81.5625, -9.988594]
[9.5625, 0.25, -25.828125, -66.59062]
[-8.5, 2.71875, -21.08703]
[-92.4375, 2.2196875]
[-75.469376]



*/


      