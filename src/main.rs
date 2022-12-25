use nokhwa::pixel_format::RgbFormat;
use nokhwa::{
    Camera,
    utils::{
	RequestedFormatType,
	RequestedFormat,
	CameraIndex,
    }
};
use mozjpeg::Compress;

use std::fs::File;
use std::io::Write;


fn main() {
    let mut camera = Camera::new(
	CameraIndex::Index(0),
	RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestResolution)
    ).unwrap();
    camera.open_stream().unwrap();
    let frame_buffer = camera.frame().unwrap();
    let decoded_image = frame_buffer.decode_image::<RgbFormat>().unwrap();

    let mut comp = Compress::new(mozjpeg::ColorSpace::JCS_RGB);
    comp.start_compress();
    comp.write_scanlines(&decoded_image);
    comp.finish_compress();
    
    let decoded_vec = comp.data_to_vec().unwrap();

    let mut file = File::create("/tmp/file.jpeg").unwrap();
    let _result = file.write_all(&decoded_vec);
}

