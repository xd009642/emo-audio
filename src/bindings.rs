#![deny(rust_2018_idioms)]
use crate::stft::*;
use ndarray::arr2;
use numpy::{IntoPyArray, PyArray1, PyArray2, ToPyArray};
use pyo3::prelude::*;
use rustfft::num_complex::Complex32;

#[pymodule]
fn emo_audio(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    type StftResult = Py<PyArray2<Complex32>>;

    #[pyfn(m, "stft")]
    fn stft(
        py: Python<'_>,
        x: &PyArray1<f32>,
        n_fft: Option<usize>,
        win_length: Option<usize>,
        hop_length: Option<usize>,
    ) -> PyResult<StftResult> {
        let mut stft = ShortTimeFourierTransform::default();
        if let Some(ffts) = n_fft {
            stft.n_fft = ffts;
        }
        if let Some(win_len) = win_length {
            stft.win_length = win_len;
            stft.win_alg = WindowingAlgorithm::Hann(win_len);
            stft.hop_length = win_len / 4;
        }
        if let Some(hop_len) = hop_length {
            stft.hop_length = hop_len;
        }
        if let Ok(data) = x.as_slice() {
            // Have an option of complex
            match stft.run(data) {
                Some(x) => Ok(x.into_pyarray(py).to_owned()),
                None => Ok(arr2(&[[]]).to_pyarray(py).to_owned()),
            }
        } else {
            Ok(arr2(&[[]]).to_pyarray(py).to_owned())
        }
    }

    Ok(())
}
