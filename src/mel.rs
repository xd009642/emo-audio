use ndarray::{prelude::*, Data};

pub fn frequency_to_mel(f: f32) -> f32 {
    2595.0 * (1.0 + f / 700.0).log10()
}

pub fn get_mel_frequencies(low_freq: f32, high_freq: f32, n_filters: usize) -> Array1<f32> {
    let low_mel = frequency_to_mel(low_freq);
    let hi_mel = frequency_to_mel(high_freq);

    Array::linspace(low_mel, hi_mel, n_filters + 2)
        .mapv(|v| 700.0 * (10.0_f32.powf(v / 2595.0) - 1.0))
}

/// State determining how to construct a Mel filter bank
pub struct MelState {
    /// Lowest frequency in the filter bank
    pub low_frequency: f32,
    /// Highest frequency in the filter bank
    pub high_frequency: f32,
    /// Number of Mel filters
    pub n_filters: usize,
    /// Number of FFTs in the spectrum
    pub n_ffts: usize,
}

pub trait MelFilterBankExt {
    fn mel_filter(&self, state: &MelState) -> Array2<f32>;
}

impl<S, D> MelFilterBank for ArrayBase<S, Ix2>
where
    S: Data<Elem = f32>,
{
    fn mel_filter(&self, state: &MelState) -> Array2<f32> {
        let freqs = get_mel_frequencies(state.low_frequency, state.high_frequency, state.n_filters);

        let mut fbank = Array2::zeros((state.n_filters, state, n_ffts));
        for m in 1..(state.n_filters + 1) {}
        unimplemented!()
    }
}
