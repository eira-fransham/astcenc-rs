use astcenc_rs::ConfigBuilder;
use astcenc_rs::Extents;
use astcenc_rs::Profile;
use astcenc_rs::PRESET_FASTEST;
use image::ColorType;
use std::path::Path;

fn main() {
    // Load image
    // Image source: https://polyhaven.com/a/rocky_terrain
    let rgba_img = image::open("examples/rocky_terrain_diff_1k.png")
        .unwrap()
        .to_rgba8();
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
    let uncompressed_data = ctx
        .decompress::<u8>(&compressed_data, rgba_img.extents, swz)
        .unwrap();
    println!("Done");

    // Write ping-pong compressed image to disk
    image::save_buffer(
        Path::new("examples/rocky_terrain_diff_1k_compressed.png"),
        uncompressed_data.data[0].as_slice(),
        width,
        height,
        ColorType::Rgba8,
    )
    .unwrap();
}
