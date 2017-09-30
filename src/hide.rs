use raster::Image;

pub fn hide_in_image(mut image: Image, data: &[u8]) -> Image {
    let (width, height) = (image.width, image.height);

    if data.len() > ((width * height) / 8) as usize {
        panic!("File to hide is too large!!");
    }

    let mut index = 0;
    let mut bit = 0;

    for y in 0..(height-1) {
        for x in 0..(width-1) {
            if index >= data.len() {
                break;
            }

            let mut c = image.get_pixel(x, y).expect("Failed to get pixel");
            let d = (data[index] >> bit) & 0x1;
            c.r = c.r & !(1 << 0) | (d << 0);
            image.set_pixel(x, y, c).expect("Failed to set pixel");

            bit += 1;

            if bit > 7 {
                bit = 0;
                index += 1;
            }
        }
    }

    image
}