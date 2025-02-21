use nih_plug::prelude::*;
use std::sync::Arc;

mod metadata;
use metadata::*;

/// A simple delay plugin.
/// Features:
/// * Delay time up to 2 seconds
/// * Feedback control for multiple echoes
/// * Wet/dry mix control#[derive(Default)]
#[derive(Default)]
struct TapeDelay {
    params: Arc<TapeDelayParams>,
    sample_rate: f32,
    delay_buffer: Vec<f32>,
    write_pos: usize,
}

#[derive(Params)]
struct TapeDelayParams {
    /// The delay time in s
    #[id = "time"]
    pub delay_time: FloatParam,

    /// The feedback amount, controlling how many repeats occur
    #[id = "feedback"]
    pub feedback: FloatParam,

    /// Mix between the dry and wet signal
    #[id = "mix"]
    pub mix: FloatParam,
}

impl Default for TapeDelayParams {
    fn default() -> Self {
        Self {
            delay_time: FloatParam::new(
                "Time",
                0.5,
                FloatRange::Linear {
                    min: 0.0,
                    max: 2.0,
                },
            )
            .with_unit(" sec")
            .with_step_size(0.1),

            feedback: FloatParam::new(
                "Feedback",
                0.3,
                FloatRange::Linear {
                    min: 0.0,
                    max: 0.95,
                },
            )
            .with_step_size(0.01),

            mix: FloatParam::new(
                "Mix",
                0.5,
                FloatRange::Linear {
                    min: 0.0,
                    max: 1.0,
                },
            )
            .with_step_size(0.01),
        }
    }
}

impl Plugin for TapeDelay {
    const NAME: &'static str = PLUGIN_NAME;
    const VENDOR: &'static str = PLUGIN_VENDOR;
    const URL: &'static str = PLUGIN_URL;
    const EMAIL: &'static str = PLUGIN_EMAIL;
    const VERSION: &'static str = PLUGIN_VERSION;

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
    ];

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type BackgroundTask = ();
    type SysExMessage = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.sample_rate = buffer_config.sample_rate;
        
        // Initialize delay buffer for maximum delay time (2 seconds)
        let buffer_size = (self.sample_rate * 2.0) as usize;
        self.delay_buffer = vec![0.0; buffer_size];
        self.write_pos = 0;
        
        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for mut channel_samples in buffer.iter_samples() {
            
            // Get parameter values
            let delay_time = self.params.delay_time.value() as f32;
            let feedback = self.params.feedback.value() as f32;
            let mix = self.params.mix.value() as f32;

            // Calculate delay in samples
            let delay_samples = (delay_time * self.sample_rate) as usize;
            
            // Process each channel
            for sample in channel_samples.iter_mut() {
                // Store the dry sample
                let dry = *sample;
                
                // Calculate read position
                let read_pos = (self.write_pos + self.delay_buffer.len() - delay_samples)
                    % self.delay_buffer.len();
                
                // Read delayed sample
                let delayed = self.delay_buffer[read_pos];
                
                // Write to delay buffer with feedback
                self.delay_buffer[self.write_pos] = *sample + (delayed * feedback);
                
                // Mix dry and wet signals
                *sample = (dry * (1.0 - mix)) + (delayed * mix);
            }
            
            // Update write position
            self.write_pos = (self.write_pos + 1) % self.delay_buffer.len();
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for TapeDelay {
    const CLAP_ID: &'static str = PLUGIN_ID;
    const CLAP_DESCRIPTION: Option<&'static str> = Some(PLUGIN_DESCRIPTION);
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Stereo,
        ClapFeature::Delay,
    ];
}

impl Vst3Plugin for TapeDelay {
    const VST3_CLASS_ID: [u8; 16] = VST3_CLASS_ID;
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Fx,
        Vst3SubCategory::Delay,
    ];
}

nih_export_clap!(TapeDelay);
nih_export_vst3!(TapeDelay);