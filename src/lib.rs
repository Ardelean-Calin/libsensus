#![cfg_attr(not(test), no_std)]

use core::iter::zip;

use heapless::Vec;

#[inline]
fn interpolate_segment(x0: f32, y0: f32, x1: f32, y1: f32, x: f32) -> f32 {
    if x <= x0 {
        return y0;
    };
    if x >= x1 {
        return y1;
    };

    let t = (x - x0) / (x1 - x0);

    y0 + t * (y1 - y0)
}

fn moisture_from_freq<const N: usize>(freq: u32, lut: Vec<f32, N>) -> f32 {
    // IMPORTANT: The LUT needs to be sorted by frequency. So 100000, 1, 16000000, 0 not inverse
    let freq_f32 = freq as f32;
    let lut_len = lut.len();

    let frequencies = lut.clone().into_iter().step_by(2);
    let percentages = lut.clone().into_iter().skip(1).step_by(2);

    let my_values = zip(frequencies, percentages);

    if freq_f32 < lut[0] {
        return lut[1] * 100.0;
    } else if freq_f32 > lut[lut_len - 2] {
        return lut[lut_len - 1] * 100.0;
    }

    let mut x0 = 0f32;
    let mut y0 = 0f32;
    let mut x1 = 0f32;
    let mut y1 = 0f32;
    for (freq_lut, perc_lut) in my_values {
        if freq_f32 > freq_lut {
            x0 = freq_lut;
            y0 = perc_lut;
        } else {
            x1 = freq_lut;
            y1 = perc_lut;
            break;
        }
    }

    let moisture = interpolate_segment(x0, y0, x1, y1, freq_f32);

    moisture * 100.0f32
}


#[cfg(test)]
mod tests {
    use super::*;
    use assert_float_eq::{assert_float_relative_eq, afe_is_relative_eq, afe_abs,afe_relative_error_msg};

    #[test]
    fn test_calibration() {
        // let result = add(2, 2);
        let lut = Vec::<f32, 4>::from_slice(&[100000.0, 1.0, 1600000.0, 0.0]).unwrap();

        assert_eq!(moisture_from_freq(99999999, lut.clone()), 0.0);
        assert_eq!(moisture_from_freq(1600000, lut.clone()), 0.0);
        assert_eq!(moisture_from_freq(100000, lut.clone()), 100.0);
        assert_eq!(moisture_from_freq(1234, lut.clone()), 100.0);
        // assert_eq!(2, 4);
    }

    #[test]
    fn test_interpolation() {
        // let result = add(2, 2);
        let result = interpolate_segment(100000.0, 1.0, 1600000.0, 0.0, 100000.0);
        assert_float_relative_eq!(result, 1.0);

        let result = interpolate_segment(100000.0, 1.0, 1600000.0, 0.0, 750_000.0);
        assert_float_relative_eq!(result, 0.56, 0.1);

        let result = interpolate_segment(100000.0, 1.0, 1600000.0, 0.0, 125_000.0);
        assert_float_relative_eq!(result, 0.98, 0.1);

        let result = interpolate_segment(100000.0, 1.0, 1600000.0, 0.0, 999_299.0);
        assert_float_relative_eq!(result, 0.4, 0.1);

        let result = interpolate_segment(100000.0, 1.0, 1600000.0, 0.0, 1600000.0);
        assert_float_relative_eq!(result, 0.0);
    }
}
