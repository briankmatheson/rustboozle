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
use std::panic;
//use std::process;

fn main() {
    let mut camera = Camera::new(
	CameraIndex::Index(0),
	RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestResolution)
    ).unwrap();
    let result = camera.open_stream();
    eprintln!("{:?}", result);
//    process::exit(1);

	
    let frame_result = camera.frame();
    //eprintln!("{:?}", frame_result);
    let frame_buffer = frame_result.unwrap();
    let resolution = frame_buffer.resolution();
    let width = resolution.width();
    let height = resolution.height();

    let decode_result = frame_buffer.decode_image::<RgbFormat>().unwrap();
    //eprintln!("{:?}", decode_result);
    let decoded_image = decode_result;

    let result = panic::catch_unwind(|| {
	let mut comp = Compress::new(mozjpeg::ColorSpace::JCS_RGB);

	comp.set_size(width.try_into().unwrap(),
		      height.try_into().unwrap());
	comp.set_mem_dest();

	eprintln!("start compress");
	comp.start_compress();
	eprintln!("write scanlines");
	comp.write_scanlines(&decoded_image);
	eprintln!("finish compress");
	comp.finish_compress();
	
	let decoded_vec = comp.data_to_vec().unwrap();
	
	let mut file = File::create("/tmp/file.jpeg").unwrap();
	let _result = file.write_all(&decoded_vec);
    });
    assert!(result.is_ok());
}

