pub mod tga {
    use std::io::prelude::*;
    use std::u8;
    use std::{fs::File, io};

    unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
        ::std::slice::from_raw_parts((p as *const T) as *const u8, ::std::mem::size_of::<T>())
    }

    #[derive(Default)]
    #[repr(packed)]
    #[allow(dead_code)]
    struct Header {
        idlength: u8,
        colormaptype: u8,
        datatypecode: u8,
        colormaporigin: u16,
        colormaplength: u16,
        colormapdepth: u8,
        x_origin: u16,
        y_origin: u16,
        width: u16,
        height: u16,
        bitsperpixel: u8,
        imagedescriptor: u8,
    }

    pub trait ColorSpace {
        fn new() -> Self;
        fn bpp() -> u8;
    }

    #[derive(Clone, Eq, PartialEq)]
    #[repr(packed)]
    pub struct Grayscale {
        pub i: u8,
    }
    #[derive(Clone, Eq, PartialEq)]
    #[repr(packed)]
    pub struct RGB {
        pub b: u8,
        pub g: u8,
        pub r: u8,
    }
    #[derive(Clone, Eq, PartialEq)]
    #[repr(packed)]
    pub struct RGBA {
        pub b: u8,
        pub g: u8,
        pub r: u8,
        pub a: u8,
    }

    pub struct Image<T: ColorSpace> {
        width: usize,
        height: usize,
        data: Vec<T>,
    }

    impl ColorSpace for Grayscale {
        fn new() -> Self {
            Grayscale { i: 0 }
        }
        fn bpp() -> u8 {
            1
        }
    }
    impl ColorSpace for RGB {
        fn new() -> Self {
            RGB { r: 0, g: 0, b: 0 }
        }
        fn bpp() -> u8 {
            3
        }
    }
    impl ColorSpace for RGBA {
        fn new() -> Self {
            RGBA {
                r: 0,
                g: 0,
                b: 0,
                a: 0,
            }
        }
        fn bpp() -> u8 {
            4
        }
    }

    const DEVELOPER_AREA_REF: [u8; 4] = [0, 0, 0, 0];
    const EXTENSION_AREA_REF: [u8; 4] = [0, 0, 0, 0];
    const FOOTER: [u8; 18] = [
        b'T', b'R', b'U', b'E', b'V', b'I', b'S', b'I', b'O', b'N', b'-', b'X', b'F', b'I', b'L',
        b'E', b'.', b'\0',
    ];

    impl<T: ColorSpace + Clone + Eq> Image<T> {
        pub fn new(width: usize, height: usize) -> Self {
            Image {
                width,
                height,
                data: vec![T::new(); width * height],
            }
        }

        pub fn set(&mut self, x: usize, y: usize, color: &T) -> Result<(), &'static str> {
            if x >= self.width || y > self.height {
                return Err("Coordinates out of bounds for image");
            }
            self.data[x + y * self.width] = color.clone();
            Ok(())
        }

        fn data_vec(&self) -> Vec<u8> {
            self.data
                .iter()
                .flat_map(|p| unsafe { any_as_u8_slice(p) })
                .copied()
                .collect::<Vec<u8>>()
        }

        fn write_rle_data(&self, out: &mut dyn Write) -> io::Result<()> {
            // TODO: Learn how this works
            const MAX_CHUNK_LENGTH: u8 = 128;
            let data = self.data_vec();
            let n_pixels = self.width * self.height;
            let mut current_pixel = 0;
            while current_pixel < n_pixels {
                let chunk_start = current_pixel * T::bpp() as usize;
                let mut current_byte = chunk_start;
                let mut run_length: u8 = 1;
                let mut raw = true;
                while current_pixel + (run_length as usize) < n_pixels
                    && run_length < MAX_CHUNK_LENGTH.into()
                {
                    let mut succ_eq = true;
                    for i in 0..(T::bpp() as usize) {
                        succ_eq =
                            data[current_byte + i] == data[current_byte + i + T::bpp() as usize];
                        if !succ_eq {
                            break;
                        }
                    }
                    current_byte += T::bpp() as usize;
                    if run_length == 1 {
                        raw = !succ_eq;
                    }
                    if raw && succ_eq {
                        run_length -= 1;
                        break;
                    }
                    if !raw && !succ_eq {
                        break;
                    }
                    run_length += 1;
                }
                current_pixel += run_length as usize;
                out.write(&[if raw {
                    run_length - 1
                } else {
                    run_length + 127
                }])
                .expect("Can't write run length to TGA file");
                out.write(
                    &data[chunk_start
                        ..chunk_start
                            + (if raw { run_length * T::bpp() } else { T::bpp() }) as usize],
                )
                .expect("Can't write RLE run to TGA file");
            }
            Ok(())
        }

        pub fn write_to_file(&self, filename: &str, vflip: bool, rle: bool) -> io::Result<()> {
            let mut out = File::options()
                .write(true)
                .create(true)
                .truncate(true)
                .open(filename)?;
            let header = Header {
                idlength: 0,
                bitsperpixel: T::bpp() << 3,
                width: self.width as u16,
                height: self.height as u16,
                datatypecode: if T::bpp() == Grayscale::bpp() {
                    match rle {
                        true => 11,
                        false => 3,
                    }
                } else {
                    match rle {
                        true => 10,
                        false => 2,
                    }
                },
                imagedescriptor: if vflip { 0x00 } else { 0x20 },
                ..Default::default()
            };
            out.write(unsafe { any_as_u8_slice(&header) })
                .expect("Error writing TGA header.");
            if !rle {
                println!("writing non RLE");
                out.write(&self.data_vec().as_slice())
                    .expect("Error dumping data to TGA file.");
            } else {
                println!("writing RLE");
                self.write_rle_data(&mut out)
                    .expect("Error dumping RLE data to TGA file");
            }
            out.write(&DEVELOPER_AREA_REF)
                .expect("Error writing developer area ref to TGA file");
            out.write(&EXTENSION_AREA_REF)
                .expect("Error writing extension area ref to TGA file");
            out.write(&FOOTER)
                .expect("Error writing footer to TGA file");
            Ok(())
        }
    }
}
