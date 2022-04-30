mod tga {
    enum PixelType {
        Grayscale,
        RGB,
        RGBA,
    }

    struct Image {
        width: i32,
        height: i32,
        pixel_type: PixelType,
        pixel_size: u8,
        data: Vec<u8>,
    }

    impl Image {
        pub const fn new(width: i32, height: i32, pixel_type: PixelType) -> Self {
            pixel_size = match pixel_type {
                Grayscale => 1,
                RGB => 3,
                RGBA => 4,
            };
            Image {
                width,
                height,
                pixel_type,
                pixel_size,
                data: vec![0; width * height * pixel_size],
            }
        }
    }
}
