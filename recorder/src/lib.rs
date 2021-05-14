pub mod recorder {
    use std::time::Duration;
    use gst::{State, Pipeline, prelude::*};
    use std::thread;
    use std::sync::{Arc, RwLock};
    use serde::{Serialize, Deserialize};

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
        source: gst::Element,
    }

    impl Recorder {
        pub fn new() -> Recorder {
            gst::init().expect("Error innitializing gstreamer");

            // Build elements
            let source = gst::ElementFactory::make("videotestsrc", Some("source"))
                .expect("Could not create source element.");
            let encoder = gst::ElementFactory::make("x265enc", Some("encoder"))
                .expect("Could not create encoder element.");
            let parser = gst::ElementFactory::make("h265parse", Some("parser"))
                .expect("Could not create muxer element.");
            let muxer = gst::ElementFactory::make("mp4mux", Some("muxer"))
                .expect("Could not create muxer element.");
            let sink = gst::ElementFactory::make("filesink", Some("sink"))
                .expect("Could not create sink element.");

            // Build the pipeline
            let pipeline = gst::Pipeline::new(Some("test-pipeline"));
            pipeline.add_many(&[&source, &encoder, &parser, &muxer, &sink]).unwrap();
            source.link(&encoder).expect("Source could not be linked to encoder.");
            encoder.link(&parser).expect("Encoder could not be linked to parser.");
            parser.link(&muxer).expect("Parser could not be linked to muxer.");
            muxer.link(&sink).expect("Muxer could not be linked to sink.");

            // Modify the source's properties
            let location = "test.mp4";
            sink.set_property("location", &location).expect("Unable to set location property on sink.");

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
                        MessageView::Eos(..) => {
                            pipeline_ref
                                .write()
                                .expect("Observer failed to obtain write lock on pipeline.")
                                .set_state(gst::State::Ready)
                                .expect("Observer failed to set pipeline to ready state.");
                        },
                        _ => (),
                    }
                }
            });

            Recorder {
                state:  state,
                pipeline: pipeline,
                observer: observer,
                source: source,
            }
        }

        pub fn get_state(&self) -> Arc<RwLock<RecorderState>> {
            Arc::clone(&self.state)
        }

        pub fn record(&mut self) {
            self.pipeline
                .write()
                .expect("Failed to obtain write lock on pipeline.")
                .set_state(gst::State::Playing)
                .expect("Unable to set the pipeline to the `Playing` state");
        }

        pub fn stop(&mut self) {
            self.source.send_event(gst::event::Eos::new());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::recorder::{RecorderState, Recorder};
    use std::{thread, time};

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
    fn test_recorder_record_and_stop() {
        let mut recorder = Recorder::new();
        recorder.record();
        thread::sleep(time::Duration::from_millis(2000));
        {
            let state_ref = recorder.get_state();
            assert_eq!(*state_ref.read().unwrap(), RecorderState{state: Some(gst::State::Playing), file: None, duration: None});
        }

        recorder.stop();
        thread::sleep(time::Duration::from_millis(2000));
        {
            let state_ref = recorder.get_state();
            assert_eq!(*state_ref.read().unwrap(), RecorderState{state: Some(gst::State::Ready), file: None, duration: None});
        }
    }
}
