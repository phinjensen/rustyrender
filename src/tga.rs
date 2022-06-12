use std::io::{prelude::*, BufWriter};
use std::u8;
use std::{fs::File, io};

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts((p as *const T) as *const u8, ::std::mem::size_of::<T>())
}

pub trait ColorSpace {
    fn new() -> Self;
    const BPP: u8;
}

#[derive(Copy, Clone)]
pub struct Grayscale {
    pub i: u8,
}
#[derive(Copy, Clone)]
pub struct RGB {
    pub b: u8,
    pub g: u8,
    pub r: u8,
}
#[derive(Copy, Clone)]
pub struct RGBA {
    pub b: u8,
    pub g: u8,
    pub r: u8,
    pub a: u8,
}

impl ColorSpace for Grayscale {
    fn new() -> Self {
        Grayscale { i: 0 }
    }
    const BPP: u8 = 1;
}
impl ColorSpace for RGB {
    fn new() -> Self {
        RGB { r: 0, g: 0, b: 0 }
    }
    const BPP: u8 = 3;
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
    const BPP: u8 = 4;
}

pub struct Image<T: ColorSpace> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

const DEVELOPER_AREA_REF: [u8; 4] = [0, 0, 0, 0];
const EXTENSION_AREA_REF: [u8; 4] = [0, 0, 0, 0];
const FOOTER: &[u8; 18] = b"TRUEVISION-XFILE.\0";
const MAX_CHUNK_LENGTH: u8 = 128;

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

impl<T: ColorSpace + Copy> Image<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Image {
            width,
            height,
            data: vec![T::new(); width * height],
        }
    }

    pub fn set(&mut self, x: usize, y: usize, color: T) -> Result<(), String> {
        if x >= self.width || y >= self.height {
            return Err(String::from("Coordinates out of bounds for image"));
        }
        self.data[x + y * self.width] = color;
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
        let data = self.data_vec();
        let n_pixels = self.width * self.height;
        let mut current_pixel = 0;
        while current_pixel < n_pixels {
            let chunk_start = current_pixel * T::BPP as usize;
            let mut current_byte = chunk_start;
            let mut run_length: u8 = 1;
            let mut raw = true;
            while current_pixel + (run_length as usize) < n_pixels && run_length < MAX_CHUNK_LENGTH
            {
                let next_pixel = current_byte + (T::BPP as usize);
                let succ_eq = data[current_byte..next_pixel]
                    == data[next_pixel..next_pixel + (T::BPP as usize)];
                current_byte += T::BPP as usize;
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
            }])?;
            out.write(
                &data[chunk_start
                    ..chunk_start + (if raw { run_length * T::BPP } else { T::BPP }) as usize],
            )?;
        }
        Ok(())
    }

    pub fn write_to_file(&self, filename: &str, vflip: bool, rle: bool) -> io::Result<()> {
        let mut out = BufWriter::new(
            File::options()
                .write(true)
                .create(true)
                .truncate(true)
                .open(filename)?,
        );

        let header = Header {
            idlength: 0,
            bitsperpixel: T::BPP << 3,
            width: self.width as u16,
            height: self.height as u16,
            datatypecode: if T::BPP == Grayscale::BPP {
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
        out.write(FOOTER).expect("Error writing footer to TGA file");
        Ok(())
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}
