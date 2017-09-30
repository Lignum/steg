use raster::Image;

pub fn extract_from_image(image: Image, length: usize) -> Vec<u8> {
    let (width, height) = (image.width, image.height);
    let mut data: Vec<u8> = vec![0; length];

    let mut index = 0;
    let mut bit = 0;

    for y in 0..(height-1) {
        for x in 0..(width-1) {
            if index >= length {
                break;
            }

            let c = image.get_pixel(x, y).expect("Failed to get pixel");
            let d = c.r & 0x1;

            if d > 0 {
                data[index] |= 1 << bit;
            }

            bit += 1;

            if bit > 7 {
                bit = 0;
                index += 1;
            }
        }
    }

    data
}