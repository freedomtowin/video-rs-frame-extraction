// use std::fs::create_dir_all;
// use opencv::{imgcodecs, prelude::*, videoio, Result};
// use opencv::core::Mat;
// use opencv::types::VectorOfi32;

// fn main() -> Result<()> {
    
//     let video_url = "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4";
//     let mut cam = videoio::VideoCapture::from_file(video_url, videoio::CAP_ANY)?; // 0 is the default camera
//     let opened = videoio::VideoCapture::is_opened(&cam)?;
//     if !opened {
//         panic!("Unable to open default camera!");
//     }

//     let output_folder = "frames_opencv";
//     create_dir_all(output_folder).expect("failed to create output directory");

//     let frame_rate = cam.get(videoio::CAP_PROP_FPS)?; // Get the frame rate of the video
//     let max_duration = 20.0; // Max duration in seconds
//     let max_frames = (frame_rate * max_duration).ceil() as usize;

//     let mut frame_count = 0;

//     while frame_count < max_frames {
//         let mut frame = Mat::default();
//         cam.read(&mut frame)?;
//         if frame.size()?.width > 0 {
//             let frame_path = format!("{}/frame_{:05}.png", output_folder, frame_count);
//             imgcodecs::imwrite(&frame_path, &frame, &VectorOfi32::new())?;

//             frame_count += 1;
//         }
  
//     }

//     println!("Saved {} frames in the '{}' directory", frame_count, output_folder);
//     Ok(())
// }


// use std::fs::create_dir_all;
// use std::path::Path;
// use std::time::Instant;
// use opencv::{imgcodecs, prelude::*, videoio, Result};
// use opencv::prelude::Mat;
// use tokio::task;

// #[tokio::main]
// async fn main() -> Result<()> {
//     let window = "video capture";

//     let video_url = "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4";
//     let mut cam = videoio::VideoCapture::from_file(video_url, videoio::CAP_ANY)?; // 0 is the default camera
//     let opened = videoio::VideoCapture::is_opened(&cam)?;
//     if !opened {
//         panic!("Unable to open default camera!");
//     }

//     let output_folder = "frames_opencv";
//     create_dir_all(output_folder).expect("failed to create output directory");

//     let frame_rate = cam.get(videoio::CAP_PROP_FPS)?; // Get the frame rate of the video
//     let max_duration = 20.0; // Max duration in seconds
//     let max_frames = (frame_rate * max_duration).ceil() as usize;

//     let mut frame_count = 0;
//     let mut tasks = vec![];

//     while frame_count < max_frames {
//         let mut frame = Mat::default();
//         cam.read(&mut frame)?;
//         if frame.size()?.width > 0 {
//             let frame_path = format!("{}/frame_{:05}.png", output_folder, frame_count);
//             let frame_clone = frame.clone(); // Clone the frame to move into the async block

//             // Spawn a blocking task to save the frame
//             let task = task::spawn_blocking(move || {
//                 imgcodecs::imwrite(&frame_path, &frame_clone, &opencv::types::VectorOfi32::new()).expect("failed to save frame");
//             });
//             tasks.push(task);

//             frame_count += 1;
//         }
  
//     }

//     // Await all tasks to finish
//     for task in tasks {
//         task.await.expect("task failed");
//     }

//     println!("Saved {} frames in the '{}' directory", frame_count, output_folder);
//     Ok(())
// }

// extern crate ffmpeg_next as ffmpeg;

// use ffmpeg::format::{input, Pixel};
// use ffmpeg::media::Type;
// use ffmpeg::software::scaling::{context::Context, flag::Flags};
// use ffmpeg::util::frame::video::Video;
// use std::fs::{self, File};
// use std::path::Path;

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     ffmpeg::init().unwrap();

//     let source_url = "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4";
//     let frames_folder = "frames_ffmpeg";

//     // Create the frames folder if it does not exist
//     if !Path::new(frames_folder).exists() {
//         fs::create_dir(frames_folder)?;
//     }

//     // Open the input video file
//     let mut ictx = input(&source_url)?;

//     // Find the best video stream
//     let input_stream = ictx
//         .streams()
//         .best(Type::Video)
//         .ok_or(ffmpeg::Error::StreamNotFound)?;

//     let framerate_q = input_stream.avg_frame_rate();
//     let framerate_f64 = framerate_q.numerator() as f64 / framerate_q.denominator() as f64;
//     let max_duration = 20.0; // Max duration in seconds
//     let max_frames = (framerate_f64 * max_duration).ceil() as usize;

//     // Get the index of the video stream
//     let input_video_stream_index = input_stream.index();

//     // Set up the video decoder
//     let context_decoder = ffmpeg::codec::context::Context::from_parameters(input_stream.parameters())?;
//     let mut video_decoder = context_decoder.decoder().video()?;

//     // Set up the scaler to convert frames to RGB24
//     let mut scaler = Context::get(
//         video_decoder.format(),
//         video_decoder.width(),
//         video_decoder.height(),
//         Pixel::RGB24,
//         video_decoder.width(),
//         video_decoder.height(),
//         Flags::BILINEAR,
//     )?;

//     let mut frame_index = 0;

//     // Function to receive and process decoded frames
//     let mut receive_and_process_decoded_frames = |decoder: &mut ffmpeg::decoder::Video, frame_index: &mut usize| -> Result<(), ffmpeg::Error> {
//         let mut decoded = Video::empty();
//         while decoder.receive_frame(&mut decoded).is_ok() {
//             let mut rgb_frame = Video::empty();
//             scaler.run(&decoded, &mut rgb_frame)?;
//             save_file(&rgb_frame, *frame_index, frames_folder).unwrap();
//             *frame_index += 1;
//         }
//         Ok(())
//     };

//     // Process packets from the input video stream
//     for (stream, packet) in ictx.packets() {
//         if frame_index > max_frames {
//             break;
//         }
//         if stream.index() == input_video_stream_index {
//             video_decoder.send_packet(&packet)?;
//             receive_and_process_decoded_frames(&mut video_decoder, &mut frame_index)?;
//         }
//     }

//     // Finalize decoding
//     video_decoder.send_eof()?;
//     receive_and_process_decoded_frames(&mut video_decoder, &mut frame_index)?;

//     Ok(())
// }

// // Function to save a frame as a PNG file
// fn save_file(frame: &Video, index: usize, folder: &str) -> image::ImageResult<()> {
//     let filename = format!("{}/frame{}.png", folder, index);
//     let path = Path::new(&filename);
//     let mut _file = File::create(path)?;

//     let (width, height) = (frame.width(), frame.height());
//     let buffer = frame.data(0);

//     // Create an ImageBuffer from the raw frame data
//     let img_buffer = image::ImageBuffer::from_raw(width, height, buffer.to_vec()).unwrap();
//     let img = image::DynamicImage::ImageRgb8(img_buffer);
//     img.save(&filename)?;
    
//     Ok(())
// }

use std::fs::create_dir_all;
use std::path::Path;
use std::error::Error;
use video_rs::decode::Decoder;
use video_rs::Url;
use image::{ImageBuffer, Rgb};
use tokio::task;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>  {
    video_rs::init().unwrap();

    let source =
        "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4"
            .parse::<Url>()
            .unwrap();
    let mut decoder = Decoder::new(source).expect("failed to create decoder");

    let output_folder = "frames_video_rs";
    create_dir_all(output_folder).expect("failed to create output directory");

    let (width, height) = decoder.size();
    let frame_rate = decoder.frame_rate(); // Assuming 30 FPS if not available
    
    let max_duration = 20.0; // Max duration in seconds
    let max_frames = (frame_rate * max_duration).ceil() as usize;

    let mut frame_count = 0;
    let mut elapsed_time = 0.0;
    let mut tasks = vec![];

    for frame in decoder.decode_iter() {
        if let Ok((_timestamp, frame)) = frame {
            if elapsed_time > max_duration {
                break;
            }

            let rgb = frame.slice(ndarray::s![.., .., 0..3]).to_slice().unwrap();
            
            let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_raw(width, height, rgb.to_vec())
                .expect("failed to create image buffer");

            let frame_path = format!("{}/frame_{:05}.png", output_folder, frame_count);
            
            let task = task::spawn_blocking(move || {
                img.save(&frame_path).expect("failed to save frame");
            });

            tasks.push(task);

            frame_count += 1;
            elapsed_time += 1.0 / frame_rate;
        } else {
            break;
        }
    }

    // Await all tasks to finish
    for task in tasks {
        task.await.expect("task failed");
    }

    println!("Saved {} frames in the '{}' directory", frame_count, output_folder);
    Ok(())
}