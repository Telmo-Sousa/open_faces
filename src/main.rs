use opencv::prelude::*;
use opencv::{
    core::{self, Mat, Point, Rect, Scalar, Size},
    highgui, imgproc, objdetect,
    prelude::{CascadeClassifierTrait, MatTraitConst, VideoCaptureTrait, VideoCaptureTraitConst},
    types::VectorOfRect,
    videoio,
};

fn main() {
    let face_cascade_file =
        "haarcascade_frontalface_default2.xml";
    let mut face_cascade = objdetect::CascadeClassifier::new(face_cascade_file).unwrap();

    let mut webcam = videoio::VideoCapture::new(0, videoio::CAP_ANY).unwrap();
    assert!(webcam.is_opened().unwrap());

    highgui::named_window("Face Detection", highgui::WINDOW_AUTOSIZE).unwrap();

    let mut key = 0;
    while key != 'q' as i32 {
        let mut frame = Mat::default();
        let mut gray_frame = Mat::default();

        webcam.read(&mut frame).expect("Failed to read frame");

        if frame.size().unwrap().width > 0 {
            imgproc::cvt_color(&frame, &mut gray_frame, imgproc::COLOR_BGR2GRAY, 0).unwrap();

            let mut faces = VectorOfRect::new();
            face_cascade
                .detect_multi_scale(
                    &gray_frame,
                    &mut faces,
                    1.1,
                    3,
                    objdetect::CASCADE_SCALE_IMAGE,
                    Size::new(30, 30),
                    Size::new(0, 0),
                )
                .unwrap();

            for face in faces.iter() {
                let p1 = Point::new(face.x, face.y);
                let p2 = Point::new(face.x + face.width, face.y + face.height);
                imgproc::rectangle(
                    &mut frame,
                    core::Rect::new(p1.x, p1.y, p2.x - p1.x, p2.y - p1.y),
                    Scalar::new(0.0, 255.0, 0.0, 0.0),
                    2,
                    8,
                    0,
                )
                .unwrap();
            }

            highgui::imshow("Face Detection", &frame).unwrap();
        }

        key = highgui::wait_key(10).unwrap();
    }
}
