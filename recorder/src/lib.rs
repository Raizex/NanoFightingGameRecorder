pub mod recorder {
    use gst::State;
    use std::time::Duration;

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
}

#[cfg(test)]
mod tests {
    use crate::recorder::RecorderState;

    #[test]
    fn test_new_recorder_state() {
        let rs = RecorderState::new();
        assert_eq!(rs, RecorderState{state: None, file: None, duration: None});
    }
}
