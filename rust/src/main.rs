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
 
    let valores:  [f32; 5] = [12.0, 2.0, 3.0, 4.0, 5.0];
    let valores2: [f32; 5] = [13.0, 7.0, 8.0, 9.0, 0.0];

    // media
    let media: f32 = estadistica::media(&valores); 

    // varianzas [poblacional, muestral]
    let varianzas: [f32; 2] = estadistica::varianza(&valores, &media);
    
    // desviacion estandar
    let desvi_estan:f32 = estadistica::desviacion_estd(&varianzas[0]);

    // factor de correlación
    let factor: f32 = estadistica::factor_correlacion(&valores, &valores2);



    println!("\nESTADISTICA::::::::::::::::::::::::::
    Datos: {valores:?} , {valores2:?}
    Media: {media}
    Varianza: {varianzas:?}
    Desviacón estandar: {desvi_estan}
    Factor: {factor}");
    

    // ************************************************************************

}

