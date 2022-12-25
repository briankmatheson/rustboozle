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
    let open_result = camera.open_stream();
    eprintln!("open camera stream {:?}", open_result);

    let frame_buffer = camera.frame().unwrap();
    let width = frame_buffer.resolution().width();
    let height = frame_buffer.resolution().height();
    let decoded_image = frame_buffer.decode_image::<RgbFormat>().unwrap();

    let unwind_result = panic::catch_unwind(|| {
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
	let write_result = file.write_all(&decoded_vec);
	eprintln!("write result {:?}", write_result);
    });
    assert!(unwind_result.is_ok());
}

