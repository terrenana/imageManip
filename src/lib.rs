use byteorder::{ByteOrder, LittleEndian};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug)]
enum Pixel {
    ColorData(u8, u8, u8),
    Padding,
}
impl Pixel {
    fn pixel2d_to_bytes(pixels: Vec<Vec<Pixel>>) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        for y in 0..pixels[0].len() {
            for x in 0..pixels.len() {
                if let Pixel::ColorData(b, g, r) = pixels[x][y] {
                    result.push(b);
                    result.push(g);
                    result.push(r);
                } else {
                    result.push(0);
                }
            }
        }
        result
    }
}
#[derive(Debug)]
#[allow(dead_code)]
struct Header {
    bmp_ident: [u8; 2],
    file_size: u32,
    reserved1: [u8; 2],
    reserved2: [u8; 2],
    offset: u32,
    header_size: u32,
    width: i32,
    height: i32,
    color_planes: u16,
    bits_per_pixel: u16,
    compression: u32,
    pixel_image_size: u32,
    hres: i32,
    vres: i32,
    gap: Vec<u8>,
}
impl From<Vec<u8>> for Header {
    fn from(header: Vec<u8>) -> Header {
        Header {
            bmp_ident: header[0..2].try_into().unwrap(),
            file_size: LittleEndian::read_u32(&header[2..6]),
            reserved1: header[6..8].try_into().unwrap(),
            reserved2: header[8..10].try_into().unwrap(),
            offset: LittleEndian::read_u32(&header[10..14]),
            header_size: LittleEndian::read_u32(&header[14..18]),
            width: LittleEndian::read_i32(&header[18..22]),
            height: LittleEndian::read_i32(&header[22..26]),
            color_planes: LittleEndian::read_u16(&header[26..28]),
            bits_per_pixel: LittleEndian::read_u16(&header[28..30]),
            compression: LittleEndian::read_u32(&header[30..34]),
            pixel_image_size: LittleEndian::read_u32(&header[34..38]),
            hres: LittleEndian::read_i32(&header[38..42]),
            vres: LittleEndian::read_i32(&header[42..46]),
            gap: header[46..].to_vec(),
        }
    }
}
impl From<Header> for Vec<u8> {
    fn from(mut header: Header) -> Self {
        let mut bytes: [u8; 46] = [0; 46];
        bytes[0] = header.bmp_ident[0];
        bytes[1] = header.bmp_ident[1];
        let file_size: [u8; 4] = header.file_size.to_le_bytes();
        bytes[2] = file_size[0];
        bytes[3] = file_size[1];
        bytes[4] = file_size[2];
        bytes[5] = file_size[3];
        bytes[6] = header.reserved1[0];
        bytes[7] = header.reserved1[1];
        bytes[8] = header.reserved2[0];
        bytes[9] = header.reserved2[1];
        let offset: [u8; 4] = header.offset.to_le_bytes();
        bytes[10] = offset[0];
        bytes[11] = offset[1];
        bytes[12] = offset[2];
        bytes[13] = offset[3];
        let header_size: [u8; 4] = header.header_size.to_le_bytes();
        bytes[14] = header_size[0];
        bytes[15] = header_size[1];
        bytes[16] = header_size[2];
        bytes[17] = header_size[3];
        let width: [u8; 4] = header.width.to_le_bytes();
        bytes[18] = width[0];
        bytes[19] = width[1];
        bytes[20] = width[2];
        bytes[21] = width[3];
        let height: [u8; 4] = header.height.to_le_bytes();
        bytes[22] = height[0];
        bytes[23] = height[1];
        bytes[24] = height[2];
        bytes[25] = height[3];
        let color_planes: [u8; 2] = header.color_planes.to_le_bytes();
        bytes[26] = color_planes[0];
        bytes[27] = color_planes[1];
        let bits_per_pixel: [u8; 2] = header.bits_per_pixel.to_le_bytes();
        bytes[28] = bits_per_pixel[0];
        bytes[29] = bits_per_pixel[1];
        let compression: [u8; 4] = header.compression.to_le_bytes();
        bytes[30] = compression[0];
        bytes[31] = compression[1];
        bytes[32] = compression[2];
        bytes[33] = compression[2];
        let pixel_image_size: [u8; 4] = header.pixel_image_size.to_le_bytes();
        bytes[34] = pixel_image_size[0];
        bytes[35] = pixel_image_size[1];
        bytes[36] = pixel_image_size[1];
        bytes[37] = pixel_image_size[1];
        let hres: [u8; 4] = header.hres.to_le_bytes();
        bytes[38] = hres[0];
        bytes[39] = hres[1];
        bytes[40] = hres[2];
        bytes[41] = hres[2];
        let vres: [u8; 4] = header.vres.to_le_bytes();
        bytes[42] = vres[0];
        bytes[43] = vres[1];
        bytes[44] = vres[2];
        bytes[45] = vres[2];
        let mut bytes_vec: Vec<u8> = bytes.to_vec();
        bytes_vec.append(&mut header.gap);
        bytes_vec
    }
}
impl Display for Header {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "BMP Ident: {:#?}", self.bmp_ident)?;
        writeln!(f, "Raw File Size: {:#?}", self.file_size)?;
        writeln!(f, "Reserved 1: {:#?}", self.reserved1)?;
        writeln!(f, "Reserved 2: {:#?}", self.reserved2)?;
        writeln!(f, "Pixel Start Offset: {:#?}", self.offset)?;
        writeln!(f, "Header Size: {:#?}", self.header_size)?;
        writeln!(f, "Width: {:#?}", self.width)?;
        writeln!(f, "Height: {:#?}", self.height)?;
        writeln!(f, "Color Planes: {:#?}", self.color_planes)?;
        writeln!(f, "Bits Per Pixel: {:#?}", self.bits_per_pixel)?;
        writeln!(f, "Compression enum: {:#?}", self.compression)?;
        writeln!(f, "Image size (pixels only): {:#?}", self.pixel_image_size)?;
        writeln!(f, "Hres: {:#?}", self.hres)?;
        writeln!(f, "Vres: {:#?}", self.vres)?;
        writeln!(f, "Gap Length: {:#?}", self.gap.len())?;
        Ok(())
    }
}

#[derive(Debug)]
struct BmpFile {
    header: Header,
    pixels: Vec<Vec<Pixel>>,
}
impl TryFrom<File> for BmpFile {
    type Error = std::io::Error;
    fn try_from(mut file: File) -> Result<BmpFile, std::io::Error> {
        let mut bytes: Vec<u8> = Vec::new();
        file.read_to_end(&mut bytes).unwrap();
        let header = Header::from(bytes[0..138].to_vec());
        let fpp: usize = header.offset as usize;
        let pixel_array: Vec<u8> = bytes[fpp..].to_vec();
        let mut padding: i32 = 0;
        if header.width * 3 % 4 != 0 {
            padding = 4 - header.width * 3 % 4
        }
        let mut pixels: Vec<Vec<Pixel>> = Vec::new();
        for _ in 0..header.width + padding {
            pixels.push(Vec::new());
        }
        let mut pixel_array_index: usize = 2;
        for _ in 0..header.height {
            for i in 0..header.width {
                pixels[i as usize].push(Pixel::ColorData(
                    pixel_array[pixel_array_index - 2],
                    pixel_array[pixel_array_index - 1],
                    pixel_array[pixel_array_index],
                ));
                pixel_array_index += 3
            }
            for i in header.width..header.width + padding {
                pixels[i as usize].push(Pixel::Padding);
                pixel_array_index += 1
            }
        }
        Ok(BmpFile { header, pixels })
    }
}
impl From<BmpFile> for Vec<u8> {
    fn from(file: BmpFile) -> Self {
        let mut header: Vec<u8> = Vec::from(file.header);
        let mut pixels: Vec<u8> = Pixel::pixel2d_to_bytes(file.pixels);

        header.append(&mut pixels);
        header
    }
}
impl Display for BmpFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "Begin BmpFile Headerdump")?;
        writeln!(f, "{}", self.header)?;
        writeln!(f, "Begin BmpFile Pixeldump\n")?;
        for row in self.pixels.iter() {
            for pixel in row.iter() {
                if let Pixel::ColorData(_, _, _) = pixel {
                    write!(f, "P")?
                } else {
                    writeln!(f, " Padding")?
                }
            }
        }
        write!(f, "fileend")
    }
}

pub fn test() {
    let path = Path::new("src/pad1.bmp");

    let file = File::open(path).unwrap();
    let mut bmp = BmpFile::try_from(file).unwrap();
    bmp.pixels[0][0] = Pixel::ColorData(255, 255, 255);
    let bytes = Vec::from(bmp);

    let mut new_file = File::create("src/manipulated-pad1.bmp").unwrap();
    new_file.write_all(&bytes).unwrap();
}
