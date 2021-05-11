pub mod recorder {
    use std::time::Duration;
    use gst::{State, Pipeline, prelude::*};
    use std::thread;
    use std::sync::{Arc, RwLock};

    #[derive(Debug, PartialEq)]
    pub struct RecorderState {
        pub state: Option<State>,
        pub file: Option<String>,
        pub duration: Option<Duration>,
    }

    impl RecorderState {
        pub fn new() -> RecorderState {
            RecorderState {
                state: None,
                file: None,
                duration: None,
            }
        }
    }

    #[derive(Debug)]
    pub struct Recorder {
        state: Arc<RwLock<RecorderState>>,
        pipeline: Arc<RwLock<Pipeline>>,
        observer: thread::JoinHandle<()>,
    }

    impl Recorder {
        pub fn new() -> Recorder {
            gst::init().expect("Error innitializing gstreamer");

            // Build elements
            let source = gst::ElementFactory::make("videotestsrc", Some("source"))
                .expect("Could not create source element.");
            let sink = gst::ElementFactory::make("autovideosink", Some("sink")).expect("Could not create sink element.");

            // Build the pipeline
            let pipeline = gst::Pipeline::new(Some("test-pipeline"));
            pipeline.add_many(&[&source, &sink]).unwrap();
            source.link(&sink).expect("Elements could not be linked.");

            // Modify the source's properties
            source.set_property_from_str("pathern", "smpte");

            let bus = pipeline.get_bus().unwrap();

            let state = Arc::new(RwLock::new(RecorderState::new()));
            let state_ref = Arc::clone(&state);
            let pipeline = Arc::new(RwLock::new(pipeline));
            let pipeline_ref = Arc::clone(&pipeline);
            let observer = thread::spawn(move || {
                for msg in bus.iter_timed(gst::CLOCK_TIME_NONE) {
                    use gst::MessageView;
                    match msg.view() {
                        MessageView::Error(err) => {
                            eprintln!(
                                "Error recieved from element {:?} {}",
                                err.get_src().map(|s| s.get_path_string()),
                                err.get_error()
                            );
                            eprintln!("Debugging information: {:?}", err.get_debug());
                            break;
                        },
                        MessageView::StateChanged(state_changed) => {
                            if state_changed.get_src().map(|s| s == *pipeline_ref.read().expect("Unable to obtain read lock on pipeline.")).unwrap_or(false) {
                                println!(
                                    "Pipeline state changed from {:?} to {:?}",
                                    state_changed.get_old(),
                                    state_changed.get_current()
                                );
                                state_ref.write().expect("Unable to obtain write lock on state").state = Some(state_changed.get_current());
                            }
                        },
                        MessageView::Eos(..) => break,
                        _ => (),
                    }
                }
            });

            Recorder {
                state:  state,
                pipeline: pipeline,
                observer: observer,
            }
        }

        pub fn get_state(&self) -> Arc<RwLock<RecorderState>> {
            Arc::clone(&self.state)
        }

        pub fn play(&mut self) {
            self.pipeline
                .write()
                .expect("Failed to obtain write lock on pipeline.")
                .set_state(gst::State::Playing)
                .expect("Unable to set the pipeline to the `Playing` state");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::recorder::{RecorderState, Recorder};
    use std::thread;

    #[test]
    fn test_new_recorder_state() {
        let rs = RecorderState::new();
        assert_eq!(rs, RecorderState{state: None, file: None, duration: None});
    }

    #[test]
    fn test_new_recorder() {
        let recorder = Recorder::new();
        let state_ref = recorder.get_state();
        assert_eq!(*state_ref.read().unwrap(), RecorderState{state: None, file: None, duration: None});
    }

    #[test]
    fn test_recorder_play() {
        let mut recorder = Recorder::new();
        recorder.play();
        thread::sleep_ms(2000);
        let state_ref = recorder.get_state();
        assert_eq!(*state_ref.read().unwrap(), RecorderState{state: Some(gst::State::Playing), file: None, duration: None});
    }

}
