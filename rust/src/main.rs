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

    println!("\nVECTORIAL:::::::::::::\nCon estos datos: {_eq:?}\nDeterminante 2x2: {determinante}\nVector 2d multiplicado: [1.0, 6.0, 9.0, 8.0, 6.0] x 5.0 = {vect_x_n:?}");
    println!("inversa 2x2: {arr_inversa:?}\nEs inversa? : {is_invs:?}\nDeterminante de 3x3: {det_arr3x3:?}");


    // ***Token  **************************************************************

    let longitud:usize =120; // tamaño del token
    let token: String = tokens::genera_token(longitud);
    println!("\nTOKEN:::::::::::::::::::::::::::::::::::::\ntoken: {token}");

    // ******* Estadistica ***************************************************** 
 
    let valores: [f32; 5] = [1.0, 9.0, 3.8, 8.9, 2.5];

    // media
    let media: f32 = estadistica::media(&valores); 

    // varianzas [poblacional, muestral]
    let varianzas: [f32; 2] = estadistica::varianza(&valores, &media);
    
    // desviacion estandar
    let desvi_estan:f32 = estadistica::desviacion_estd(&varianzas[0]);

    println!("\nESTADISTICA::::::::::::::::::::::::::\nMedia: {media}\nVarianza: {varianzas:?}\nDesviacón estandar: {desvi_estan}");

    // ************************************************************************

}

