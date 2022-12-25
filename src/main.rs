use nokhwa::{Camera, CameraFormat, FrameFormat};
use std::fs::File;
use std::io::Write;

fn main() {
    let mut camera = Camera::new(
	0,
	Some(CameraFormat::new_from(640, 480, FrameFormat::MJPEG, 30)),
    )
    .unwrap();
    camera.open_stream().unwrap();
    let frame = camera.frame().unwrap();

    let file = File::create("/tmp/cam");
    let _result = file.unwrap().write_all(&frame);
}
