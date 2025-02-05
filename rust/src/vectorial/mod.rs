
pub fn n_x_vec2d(_vecn: Vec<f64>, _n: f64) -> Vec<f64> {
    let mut nw_vec: Vec<f64> = vec![];
    for i in _vecn {
        nw_vec.push(i * _n);
    }
    return nw_vec;
}

pub fn det_arr2x2(eq: [[f64; 2]; 2]) -> f64 {
    let mut det: f64;
    det = eq[0][0] * eq[1][1];
    det -= eq[0][1] * eq[1][0];
    return det;
}

pub fn inv_arr2x2(arr: [[f64; 2]; 2], det: f64) -> [[f64; 2]; 2] {
    let mut inv: [[f64; 2]; 2] = [[0f64; 2]; 2];
    inv[0][0] = 1f64 / det * arr[1][1];
    inv[0][1] = 1f64 / det * (arr[0][1] * -1f64);
    inv[1][0] = 1f64 / det * (arr[1][0] * -1f64);
    inv[1][1] = 1f64 / det * arr[0][0];
    return inv;
}

pub fn is_inv_arr2x2(arr: [[f64; 2]; 2], inv: [[f64; 2]; 2]) -> [[f64; 2]; 2] {
    let mut mtx_idntity: [[f64; 2]; 2] = [[0f64; 2]; 2];

    mtx_idntity[0][0] = ((arr[0][0] * inv[0][0]) + (arr[0][1] * inv[1][0])).round();
    mtx_idntity[0][1] = ((arr[0][0] * inv[0][1]) + (arr[0][1] * inv[1][1])).round();
    mtx_idntity[1][0] = ((arr[1][0] * inv[0][0]) + (arr[1][1] * inv[1][0])).round();
    mtx_idntity[1][1] = ((arr[1][0] * inv[0][1]) + (arr[1][1] * inv[1][1])).round();

   mtx_idntity
    
}

pub fn det_arr3x3(arr3: [[f64; 3]; 3])->f64 {
    let arr: [[f64; 3]; 5] = [arr3[0], arr3[1], arr3[2], arr3[0], arr3[1]];

    let mut acm: f64 = 0.0;
    let mut cont: u32 = 0;
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

    left - right
}