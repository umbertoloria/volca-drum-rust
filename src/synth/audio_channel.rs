use rodio::{OutputStream, OutputStreamHandle, Sink};

pub struct AudioChannel {
    stream: OutputStream,
    stream_handle: OutputStreamHandle,
}
impl AudioChannel {
    pub fn new_from_main() -> Self {
        let (stream, stream_handle) = get_default_output_stream();
        Self {
            stream,
            stream_handle,
        }
    }
    pub fn create_sink(&self, volume: f32) -> Sink {
        create_empty_sink(&self.stream_handle, volume)
    }
}

// Audio Vendor stuff
fn get_default_output_stream() -> (OutputStream, OutputStreamHandle) {
    let (stream, stream_handle) = OutputStream::try_default().unwrap();
    (stream, stream_handle)
}

fn create_empty_sink(stream_handle: &OutputStreamHandle, volume: f32) -> Sink {
    let mut sink = Sink::try_new(&stream_handle).unwrap();
    sink.set_volume(volume);
    sink
}
