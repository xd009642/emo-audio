use ndarray::{prelude::*, Data};

pub fn frequency_to_mel(f: f32) -> f32 {
    2595.0 * (1.0 + f / 700.0).log10()
}

fn scale(v: f32) -> f32 {
    700.0 * (10.0f32.powf(v / 2595.0) - 1.0)
}

fn bin_frequency(n_fft: usize, hz_point: f32, sample_rate: f32) -> f32 {
    (n_fft + 1) as f32 * hz_point / sample_rate
}

pub fn get_mel_frequencies(low_freq: f32, high_freq: f32, n_filters: usize) -> Array1<f32> {
    let low_mel = frequency_to_mel(low_freq);
    let hi_mel = frequency_to_mel(high_freq);

    Array::linspace(low_mel, hi_mel, n_filters + 2).mapv(scale)
}

/// State determining how to construct a Mel filter bank
pub struct MelState {
    /// Sample rate of the incoming signal
    pub sample_rate: f32,
    /// Lowest frequency in the filter bank
    pub low_frequency: f32,
    /// Number of Mel bands to generate
    pub n_filters: usize,
    /// Number of FFTs in the spectrum
    pub n_ffts: usize,
}

pub trait MelFilterBankExt {
    fn mel_filter(&self, state: &MelState) -> Array2<f32>;
}

impl<S> MelFilterBankExt for ArrayBase<S, Ix2>
where
    S: Data<Elem = f32>,
{
    fn mel_filter(&self, state: &MelState) -> Array2<f32> {
        let freqs = get_mel_frequencies(
            state.low_frequency,
            0.5 * state.sample_rate,
            state.n_filters,
        );
        let hz_points = freqs.mapv(scale);
        let bins = hz_points.mapv(|x| bin_frequency(state.n_ffts, x, state.sample_rate));

        let cols = state.n_ffts / 2 + 1;
        let mut fbank = Array2::zeros((state.n_filters, cols));
        for m in 1..(state.n_filters + 1) {
            let f_m_minus = bins[m - 1] as usize;
            let f_m = bins[m] as usize;
            let f_m_plus = bins[m + 1] as usize;

            for k in f_m_minus..f_m {
                fbank[[m - 1, k]] = (k as f32 - bins[m - 1]) / (bins[m] - bins[m - 1])
            }
            for k in f_m..f_m_plus {
                fbank[[m - 1, k]] = (bins[m + 1] - k as f32) / (bins[m + 1] - bins[m])
            }
        }
        // let mut filter_banks = pow_frames.dot(fbank.t())
        // filter_banks.mapv_inplace(|x| x < f32::EPSILON ? f32::EPSILON * 20.0 : x * 20.0);
        // filter_banks
        unimplemented!()
    }
}
