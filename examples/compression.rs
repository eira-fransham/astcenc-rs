use astcenc_rs::ConfigBuilder;
use astcenc_rs::Extents;
use astcenc_rs::Profile;
use astcenc_rs::PRESET_FASTEST;
use bytesize::ByteSize;
use show_image::{create_window, event};

#[show_image::main]
fn main() {
    // Load image
    // Image source: https://polyhaven.com/a/rocky_terrain
    let rgba_img = image::open("assets/rocky_terrain_diff_1k.png")
        .unwrap()
        .to_rgba8();
    let uncompressed_size: usize = rgba_img.as_raw().len();
    let (width, height) = rgba_img.dimensions();
    println!("Width is {}", width);
    println!("Height is {}", height);

    // Get in desired format
    let img_array = [rgba_img];
    let rgba_img = astcenc_rs::Image {
        extents: astcenc_rs::Extents::new(width, height),
        data: img_array.as_slice(),
    };

    // Set configs
    let config = ConfigBuilder::new()
        .with_preset(PRESET_FASTEST)
        .with_block_size(Extents::new(12, 12))
        .with_profile(Profile::LdrRgba)
        .build()
        .unwrap();
    let mut ctx = astcenc_rs::Context::new(config).unwrap();
    let swz = astcenc_rs::Swizzle::rgba();

    // Compress and decompress
    println!("Compressing");

    let compressed_data = ctx.compress(&rgba_img, swz).unwrap();
    let compressed_size = compressed_data.len();

    let uncompressed_data = ctx
        .decompress::<u8>(&compressed_data, rgba_img.extents, swz)
        .unwrap();
    let uncompressed_data = image::DynamicImage::from(
        image::RgbaImage::from_raw(width, height, uncompressed_data.data[0].clone()).unwrap(),
    );

    println!("Done");
    println!(
        "Compression ratio: {} uncompressed, {} compressed ({:.01}x)",
        ByteSize(uncompressed_size as _),
        ByteSize(compressed_size as _),
        uncompressed_size as f32 / compressed_size as f32
    );

    // Create a window and display the image.
    let window = create_window("astcenc-rs roundtrip example", Default::default()).unwrap();
    window.set_image("uncompressed", uncompressed_data).unwrap();

    // Print keyboard events until Escape is pressed, then exit.
    // If the user closes the window, the channel is closed and the loop also exits.
    for event in window.event_channel().unwrap() {
        if let event::WindowEvent::KeyboardInput(event) = event {
            if event.input.key_code == Some(event::VirtualKeyCode::Escape)
                && event.input.state.is_pressed()
            {
                break;
            }
        }
    }
}
