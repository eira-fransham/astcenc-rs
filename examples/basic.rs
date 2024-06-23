fn main() {
    let in_file = std::env::args().nth(1).unwrap();
    let out_file = std::env::args().nth(2);

    let in_file_image =
        image::load_from_memory(&std::fs::read(std::path::Path::new(&in_file)).unwrap())
            .unwrap()
            .into_rgba8();

    let config = astcenc_rs::ConfigBuilder::new().build().unwrap();
    let mut context = astcenc_rs::Context::new(config).unwrap();
let output =    context
        .compress(
            &astcenc_rs::Image {
                extents: astcenc_rs::Extents {
                    x: in_file_image.width(),
                    y: in_file_image.height(),
                    z: 1
                },
                data: in_file_image
                    .pixels()
                    .flat_map(|px| px.0)
                    .collect::<Vec<_>>(),
            },
            astcenc_rs::Swizzle::rgba(),
        )
        .unwrap();
}
