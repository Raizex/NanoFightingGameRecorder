pub mod recorder {
    use std::time::Duration;
    use gst::{State, Pipeline};
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
        pipeline: Pipeline,
        observer: thread::JoinHandle<()>,
    }

    impl Recorder {
        pub fn new() -> Recorder {
            gst::init().expect("Error innitializing gstreamer");

            Recorder {
                state:  Arc::new(RwLock::new(RecorderState::new())),
                pipeline: Pipeline::new(Some("Pipeline")),
                observer: thread::spawn(move || {
                }),
            }
        }

        pub fn get_state(&self) -> Arc<RwLock<RecorderState>> {
            Arc::clone(&self.state)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::recorder::{RecorderState, Recorder};

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
}
