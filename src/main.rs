#[macro_use] extern crate rocket;
use std::io::Cursor;
use rocket::http::ContentType;
use arducam_mipicamera::*;

use image::{io::Reader as ImageReader, ImageFormat, ImageOutputFormat};


const V4L2_CID_FOCUS_ABSOLUTE: i32 = 10094858; // ((0x009a0000 | 0x900)+10)
const V4L2_CID_EXPOSURE: i32 = 9963793; 


#[get("/")]
fn index() -> &'static str {
    "arducamera RESTful API is working"
}


fn get_image_owned() -> (ContentType, Vec<u8>) {
    let dyn_image = ImageReader::open("./stereo_capture.jpg").unwrap().decode().unwrap();
    
    let mut bytes: Vec<u8> = Vec::new();

    dyn_image.write_to(&mut Cursor::new(&mut bytes), ImageOutputFormat::Jpeg(100)).unwrap();
    
    //ImageContent { raw: bytes }
    (ContentType::JPEG, bytes)
}


#[get("/camera/capture")]
fn capture() -> (ContentType, Vec<u8>){
    println!("Initializing Camera!");
    let mut camera = arducam_mipicamera::Camera::init(None).unwrap();
    //camera.set_lens_table();
    camera.set_mode(9);

    //println!("reseting control {} = {}", "V4L2_CID_FOCUS_ABSOLUTE", 10094858 );
    camera.reset_control(V4L2_CID_FOCUS_ABSOLUTE);  // todo:// this fails unwrap() ?
    camera.set_control( V4L2_CID_FOCUS_ABSOLUTE, 0 );
    camera.set_control( V4L2_CID_EXPOSURE, 1758 );
    
    let rslt = camera.set_resolution(3840,1080).unwrap();
    println!("resolution: {:?}", rslt);

    println!("setting auto exposure");
    camera.arducam_software_auto_exposure(true).unwrap();

    println!("setting whitebalance");
    camera.arducam_software_auto_white_balance(true).unwrap();

    println!("setting awb stuff");
    camera.arducam_manual_set_awb_compensation(100,100);

    let format = camera.get_format().unwrap();
    println!("\nformat: {:?}", format);

    // JPEG quality setting (1-100)
    println!("\nCapturing Image:");
    let buffer = camera.capture(1000, Encoding::Jpeg, 100).unwrap();
    println!("Capture complete");
    let bytes = buffer.data();
   // let dyn_image = ImageReader::new(Cursor::new(bytes))
   //     .with_guessed_format().unwrap()
   //     .decode().unwrap();
    

    //let mut bytes: Vec<u8> = Vec::new();
    println!("Writing bytes to image buffer");
   // dyn_image.write_to(&mut Cursor::new(&mut bytes), ImageOutputFormat::Jpeg(50)).unwrap();
    println!("Capture complete sending result");
    (ContentType::JPEG, bytes.to_vec())
}



#[launch]
fn rocket() -> _ {
    let figment = rocket::config::Config::figment()
        .merge(("secret_key", "+KS94aFsclxip0qY54kx82g5APnF4a2pdyMiDhny9M10wfbuYPS6dZp1OaaKTceo5WDCUbWaNqD1Z8v3tEEfOg=="));

    let config = rocket::config::Config::from(figment);
    assert!(!config.secret_key.is_zero());

    rocket::build().mount("/", routes![index, capture])
}




pub struct ImageContent{
    pub raw: Vec<u8>,
}
