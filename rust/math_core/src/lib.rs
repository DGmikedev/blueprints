/*
* arr3: { [[f64; 3]; 3] }
* return  determinate del array 3x3
*/
pub fn det_arr3x3(arr3: [[f64; 3]; 3]) {
    let arr: [[f64; 3]; 5] = [arr3[0], arr3[1], arr3[2], arr3[0], arr3[1]];

    let mut acm: f64 = 0.0;
    let mut cont: u8 = 0;
    let mut left: f64 = 0.0;
    let mut right: f64 = 0.0;

    let chord: [[f64; 3]; 6] = [
        [arr[0][0], arr[1][1], arr[2][2]],
        [arr[1][0], arr[2][1], arr[3][2]],
        [arr[2][0], arr[3][1], arr[4][2]],
        [arr[0][2], arr[1][1], arr[2][0]],
        [arr[1][2], arr[2][1], arr[3][0]],
        [arr[2][2], arr[3][1], arr[4][0]],
    ];

    chord.iter().for_each(|vector| {
        acm = vector.iter().fold(1.0, |acm, x| acm * x);
        if cont < 3 { left += acm } else { right += acm }
        cont += 1;
    });

    println!("determinante 3x3-{}", left - right);
}

/* _---------------------------------_*/

pub fn det_arr2x2(eq: [[f64; 2]; 2]) -> f64 {
    let mut det: f64;
    det = eq[0][0] * eq[1][1];
    det -= eq[0][1] * eq[1][0];
    return det;
}

pub fn n_x_vec2d(_vecn: Vec<f64>, _n: f64) -> Vec<f64> {
    let mut nw_vec: Vec<f64> = vec![];
    for i in _vecn {
        nw_vec.push(i * _n);
    }
    return nw_vec;
}

pub fn inv_arr2x2(arr: [[f64; 2]; 2], det: f64) -> [[f64; 2]; 2] {
    let mut inv: [[f64; 2]; 2] = [[0f64; 2]; 2];
    inv[0][0] = 1f64 / det * arr[1][1];
    inv[0][1] = 1f64 / det * (arr[0][1] * -1f64);
    inv[1][0] = 1f64 / det * (arr[1][0] * -1f64);
    inv[1][1] = 1f64 / det * arr[0][0];
    return inv;
}

pub fn is_inv_arr2x2(arr: [[f64; 2]; 2], inv: [[f64; 2]; 2]) -> String {
    let mut mtx_idntity: [[f64; 2]; 2] = [[0f64; 2]; 2];

    mtx_idntity[0][0] = ((arr[0][0] * inv[0][0]) + (arr[0][1] * inv[1][0])).round();
    mtx_idntity[0][1] = ((arr[0][0] * inv[0][1]) + (arr[0][1] * inv[1][1])).round();
    mtx_idntity[1][0] = ((arr[1][0] * inv[0][0]) + (arr[1][1] * inv[1][0])).round();
    mtx_idntity[1][1] = ((arr[1][0] * inv[0][1]) + (arr[1][1] * inv[1][1])).round();

    println!("{:?}", mtx_idntity);
    return String::from("resultado");
}


/* fn main() {
    let _eq: [[f64; 2]; 2] = [[-1.0, 7.0], [-5.20, 10.0]];
    println!("{:?}", _eq);
    let determinante = det_arr2x2(_eq);

    let vecn: Vec<f64> = vec![1.0, 6.0, 9.0, 8.0, 6.0];

    println!("Determinate {}", determinante);

    let g = n_x_vec2d(vecn, 5.0);

    //  println!("{:?}",g);

    let invArr: [[f64; 2]; 2] = inv_arr2x2(_eq, 26.4);

    println!("Inversa = {:#?}", invArr);

    let is_invs = is_inv_arr2x2(_eq, invArr);

    let arr3: [[f64; 3]; 3] = [[1.0, 6.0, 9.7], [4.6, 6.1, 9.0], [1.0, 5.0, 7.0]];

    det_arr3x3(arr3);
}  */



/*
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/