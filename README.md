# open_faces

This project demonstrates real-time face detection using the OpenCV library in Rust. It captures video from a webcam, detects faces in each frame, and draws rectangles around the detected faces.

## Features

- Real-time face detection from webcam feed.
- Cross-platform compatibility (works on Windows, macOS, and Linux).
- Minimal setup required.

## Requirements

- Rust programming language installed ([Rust Website](https://www.rust-lang.org/))
- OpenCV library for Rust
- Webcam

## Installation

```bash
git clone https://github.com/Telmo-Sousa/open_faces.git
```

```bash
cd open_faces
```

```bash
cargo run
```

## FQA

- "Why are there 2 .XML files"
    - These files contain pre-trained data for the Haar Cascade classifier, specifically for frontal face detection. In OpenCV, Haar Cascade classifiers are a popular method for object detection in images.

## License

This project is licensed under the Apache License 2.0. See the [LICENSE](LICENSE) file for details.
