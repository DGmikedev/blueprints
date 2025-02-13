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
            
        //  [42.0, 557, 914,  836,  229,  203.5, 771,  629]   // varianza
        //  [42.0, 557, 914,  836,  229,  203.5, 771,  629]   <--  comprobacion OK

        // [6.48, 23.6, 30.2, 28.9, 15.1, 14.2, 27.7, 25.0]   // desviacion_estandar
        // [6.48, 23.6, 30.2, 28.9, 15.1, 14.2, 27.7, 25.0]    <--  comprobacion OK




        //  [-2.5, -17.75, 55.625, -16.5, -19.125, -11.0, -13.125, -12.411251]
        //  [5.5, -16.75, -19.375, -22.5, -15.125, -6.0, -9.125, -15.411251]
        //  [1.5, -19.75, -22.375, 56.5, 8.875, 26.0, -11.125, -25.411251]
        //  [8.5, 52.25, -24.375, 37.5, -0.125, 16.0, -14.125, -9.411251]
        //  [6.5, -14.75, -18.375, -33.5, 9.875, -23.0, -12.125, 34.58875]
        //  [-10.5, -5.75, -15.375, -11.5, -13.125, 0.0, -21.125, 44.58875]
        //  [-0.5, 22.25, 46.625, -2.5, 29.875, -3.0, 69.875, -25.411251]
        //  [-8.5, 0.25, -2.375, -7.5, -1.125, 1.0, 10.875, 8.87875]




        ];

        let matriz_5: Vec<Vec<f32>> = vec! [
            vec![10.0, 12.0, 91.0, 18.0, 13.0],
            vec![18.0, 13.0, 16.0, 12.0, 17.0],
            vec![14.0, 10.0, 13.0, 91.0, 41.0],
            vec![21.0, 82.0, 11.0, 72.0, 32.0],
            vec![19.0, 15.0, 17.0, 01.0, 42.0]
        ];



    //     let transp: Vec<Vec<f32>> = estadistica:: correlacion_matriz(&matriz_5);
    let transp: Vec<Vec<f32>> = estadistica:: correlacion_matriz(&matriz);
    // for i in transp.clone().into_iter(){
    //     println!("{i:?}")
    // }
// 
    // println!("{transp:?}");
/*
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

