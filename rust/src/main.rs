mod estadistica;
mod tokens;
mod vectorial;

fn main() {

    /*** Vectorial  */

    // vector 2x2
    let _eq: [[f64; 2]; 2] = [[-1.0, 7.0], [-5.20, 10.0]];
    let determinante = vectorial::det_arr2x2(_eq);

    // inversa de array 2x2 [[-1.0, 7.0], [-5.20, 10.0]]
    let arr_inversa: [[f64; 2]; 2] = vectorial::inv_arr2x2(_eq, 26.4);

    // vetor 2d -> n(5.0) x vector [1.0, 6.0, 9.0, 8.0, 6.0]
    let vecn: Vec<f64> = vec![1.0, 6.0, 9.0, 8.0, 6.0];
    let vect_x_n: Vec<f64> = vectorial::n_x_vec2d(vecn, 5.0);

    // es inversa de una array 2x2  [[0.378, -0.265], [0.196, -0.0378]] es inversa de [[-1.0, 7.0], [-5.20, 10.0]] ?
    let is_invs: [[f64; 2]; 2] = vectorial::is_inv_arr2x2(_eq, arr_inversa);

    // Determinante de array de 3x3 [[1.0, 6.0, 9.7], [4.6, 6.1, 9.0], [1.0, 5.0, 7.0]]
    let array_3x3: [[f64; 3]; 3] = [[1.0, 6.0, 9.7], [4.6, 6.1, 9.0], [1.0, 5.0, 7.0]];
    let det_arr3x3:f64 = vectorial::det_arr3x3(array_3x3);

    println!("\nVECTORIAL:::::::::::::
    Con estos datos: {_eq:?}
    Determinante 2x2: {determinante}
    Vector 2d multiplicado: [1.0, 6.0, 9.0, 8.0, 6.0] x 5.0 = {vect_x_n:?}
    Inversa 2x2: {arr_inversa:?}
    Es inversa? : {is_invs:?}
    Determinante de 3x3: {det_arr3x3:?}");
    


    // ***Token  **************************************************************

    let longitud:usize =18; // tamaño del token
    let token: String = tokens::genera_token(longitud);
    println!("\nTOKEN:::::::::::::::::::::::::::::::::::::
    Token: {token}");

    // ******* Estadistica ***************************************************** 
 
    let valores:Vec<f32>  = vec![12.0, 15.0, 22.0, 29.0, 34.0, 40.0, 18.0, 25.0];  // [f32; 8] = [12.0, 15.0, 22.0, 29.0, 34.0, 40.0, 18.0, 25.0];
    let valores2:Vec<f32> = vec![13.0, 4.24, 420.99, 5.0, 451.0, 65.0, 65.0, 320.0];// [f32; 8] = [13.0, 4.24, 420.99, 5.0, 451.0, 65.0, 65.0, 320.0];

    // media
    let media: f32 = estadistica::media(&valores); 

    // varianzas [poblacional, muestral]
    // let varianzas: [f32; 2] = estadistica::varianza(&valores, &media);
    
    // desviacion estandar
//   let desvi_estan:f32 = estadistica::desviacion_estd(&varianzas[0]);

    // coeficinete de variacion
    // CV < 10% → Baja variabilidad (los datos son relativamente homogéneos).
    // 10% ≤ CV ≤ 30% → Variabilidad moderada.
    // CV > 30% → Alta variabilidad (los datos son muy dispersos).
//    let coeficiente_de_variacion: f32 = estadistica::coeficiente_de_variacion(&desvi_estan, &media);

    // factor de correlación
    // let factor_correlacion: f32 = estadistica::factor_correlacion(&valores, &valores2);


    // factor de correlación matricial

    let mut matriz: Vec<Vec<f32>> = vec![
        vec![ 3.0, 7.0,  11.0 ],
        vec![ 5.0, 9.0,  15.0 ],
        vec![ 7.0, 13.0, 19.0 ]
    ];

/*
    let mut matriz: Vec<Vec<f32>> = vec![
        vec![10.0, 12.0, 91.0, 18.0, 13.0, 14.0, 10.0, 15.0],
        vec![18.0, 13.0, 16.0, 12.0, 17.0, 19.0, 14.0, 12.0],
        vec![14.0, 10.0, 13.0, 91.0, 41.0, 51.0, 12.0, 02.0],
        vec![21.0, 82.0, 11.0, 72.0, 32.0, 41.0, 09.0, 18.0],
        vec![19.0, 15.0, 17.0, 01.0, 42.0, 02.0, 11.0, 62.0],
        vec![02.0, 24.0, 20.0, 23.0, 19.0, 25.0, 02.0, 72.0],
        vec![12.0, 52.0, 82.0, 32.0, 62.0, 22.0, 93.0, 02.0],
        vec![04.0, 30.0, 33.0, 27.0, 31.0, 26.0, 34.0, 36.29]
        //  [12.5, 29.7, 35.3, 34.5, 32.1, 25.0, 23.1, 27.41]  // media
        //  [12.5  29.7, 35.3, 34.5, 32.1, 25.0, 23.1, 27.41]  <-- comprobacion OK
*/
        //  [42.0, 557, 914,  836,  229,  203.5, 771,  629]   // varianza
        //  [42.0, 557, 914,  836,  229,  203.5, 771,  629]   <--  comprobacion OK

        // [6.48, 23.6, 30.2, 28.9, 15.1, 14.2, 27.7, 25.0]   // desviacion_estandar
        // [6.48, 23.6, 30.2, 28.9, 15.1, 14.2, 27.7, 25.0]    <--  comprobacion OK


//         [1.0,          0.25972697,   0.20279862,   0.21203844,   0.40477872,   0.42996222,   0.22080746,   0.2444893]
//         [-0.08024583,  1.0,         -0.017194899, -0.017978325, -0.034320395, -0.036455654, -0.018721834, -0.02072977]
//         [-1.254099,   -0.3441605,    1.0,         -0.2809691,   -0.53636646,  -0.5697368,   -0.29258883,  -0.3239693]
//         [0.60444295,   0.16587636,   0.12951869,   1.0,          0.25851464,   0.27459824,   0.14102016,   0.15614474]
//         [0.26985246,   0.0740552,    0.05782339,   0.06045792,   1.0,          0.122593895,  0.0629582,    0.06971054]
//         [-0.31909147, -0.087567784, -0.06837421,  -0.071489446, -0.13647245,   1.0,         -0.07444595,  -0.082430355]
//         [-1.6117705,  -0.44231576,  -0.3453666,   -0.36110207,  -0.6893393,   -0.73222685,   1.0,         -0.41636598]
//         [-0.34405333, -0.094418034, -0.07372299,  -0.077081926, -0.14714842,  -0.15630332,  -0.08026971,   1.0       ]






        //  [-2.5, -17.75, 55.625, -16.5, -19.125, -11.0, -13.125, -12.411251]
        //  [5.5, -16.75, -19.375, -22.5, -15.125, -6.0, -9.125, -15.411251]
        //  [1.5, -19.75, -22.375, 56.5, 8.875, 26.0, -11.125, -25.411251]
        //  [8.5, 52.25, -24.375, 37.5, -0.125, 16.0, -14.125, -9.411251]
        //  [6.5, -14.75, -18.375, -33.5, 9.875, -23.0, -12.125, 34.58875]
        //  [-10.5, -5.75, -15.375, -11.5, -13.125, 0.0, -21.125, 44.58875]
        //  [-0.5, 22.25, 46.625, -2.5, 29.875, -3.0, 69.875, -25.411251]
        //  [-8.5, 0.25, -2.375, -7.5, -1.125, 1.0, 10.875, 8.87875]




//        ];

        let matriz_5: Vec<Vec<f32>> = vec! [
            vec![10.0, 12.0, 91.0, 18.0, 13.0],
            vec![18.0, 13.0, 16.0, 12.0, 17.0],
            vec![14.0, 10.0, 13.0, 91.0, 41.0],
            vec![21.0, 82.0, 11.0, 72.0, 32.0],
            vec![19.0, 15.0, 17.0, 01.0, 42.0]
        ];



         let transp: Vec<Vec<f32>> = estadistica:: correlacion_matriz(&matriz_5);
    //  let transp: Vec<Vec<f32>> = estadistica:: correlacion_matriz(&matriz);
    // for i in transp.clone().into_iter(){
    //     println!("{i:?}")
    // }
// 
    // println!("{transp:?}");
/*
 X  1           2         3            4          5
 1  X         336.19998  -493.2       19.399994  102.0
 2  336.19998   X        -1320.2001
 3  -493.2   -1320.2001   X   
 4
 5


  1 y 2        2 y 3         3 y 4        4 y 5
 336.19998,   -1320.2001,   -1920.4001,   889.0

1 y 3          2 y 4         3 y 5
[-493.2,      2079.4,      -1238.0001]

 1 y 4        2 y 5
 19.399994,   212.99998

1 y 5
102.0




[   X  -    336.19998, -1320.2001, -1920.4001, 889.0 ]
 336.19998,     X
-1320.2001, 
-1920.4001, 
 889.0 







    println!("\nESTADISTICA::::::::::::::::::::::::::
           Datos: {valores:?} , {valores2:?}
           Media (μ): {media}
           Varianza (σ^2): {varianzas:?}
           Coeficiente de variación (CV): {coeficiente_de_variacion}
           Desviación estándar (σ): {desvi_estan}
           Coeficiente de correlación (ρ): {factor_correlacion}");
    

    // ************************************************************************
*/
}

