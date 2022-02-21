/// Select n numbers from the array whose sum is equal to a fixed value
///
/// # Examples
///
/// ```
/// let keys: [f64; 7] = [0f64, 1f64, 2f64, 3f64, 4f64, 5f64, 6f64];
/// println!("{:?}", comb_sum::compute(&keys, 9f64));
/// ```
///
/// `The printed result is as follows`
///
/// ```
/// [[2.0, 3.0, 4.0], [0.0, 2.0, 3.0, 4.0], [1.0, 3.0, 5.0], [0.0, 1.0, 3.0, 5.0], [4.0, 5.0], [0.0, 4.0, 5.0], [1.0, 2.0, 6.0], [0.0, 1.0, 2.0, 6.0], [3.0, 6.0], [0.0, 3.0, 6.0]]
/// ```
#[allow(dead_code)]
pub fn compute(keys: &[f64], kill: f64) -> Vec<Vec<f64>> {
    let ks_len = keys.len();
    let n_bit = 1 << ks_len;
    let mut nin: f64;

    let mut n_ret = Vec::<f64>::new();
    let mut ret = Vec::<Vec<f64>>::new();

    for i in 0..n_bit {
        nin = 0f64;
        n_ret.clear();
        for j in 0..ks_len {
            let tmp = 1 << j; // Right shift from 0 to n
            if tmp & i != 0
            /* And operation, it will be 1 when the same is 1 */
            {
                nin += keys[j];
                n_ret.push(keys[j]);
            }
        }
        if nin == kill {
            ret.push(n_ret.clone());
            continue;
        }
    }

    if ret.is_empty() {
        ret.push(Vec::<f64>::new().clone());
    }
    ret
}
