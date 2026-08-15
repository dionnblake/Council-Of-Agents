use std::fs::{self, File};
use std::path::Path;

fn ensure_icon() {
    let icon_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("icons");
    let icon_path = icon_dir.join("icon.png");
    fs::create_dir_all(&icon_dir).expect("create Tauri icon directory");
    let mut pixels = vec![0_u8; 32 * 32 * 4];
    for y in 0..32 {
        for x in 0..32 {
            let index = (y * 32 + x) * 4;
            let mark = (x > 7 && x < 12 && y > 16)
                || (x > 14 && x < 19 && y > 10)
                || (x > 21 && x < 26 && y > 4);
            pixels[index] = if mark { 212 } else { 11 };
            pixels[index + 1] = if mark { 131 } else { 13 };
            pixels[index + 2] = if mark { 88 } else { 15 };
            pixels[index + 3] = 255;
        }
    }
    let file = File::create(&icon_path).expect("create Tauri icon");
    let mut encoder = png::Encoder::new(file, 32, 32);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().expect("write Tauri icon header");
    writer
        .write_image_data(&pixels)
        .expect("write Tauri icon pixels");

    let ico_path = icon_dir.join("icon.ico");
    let image = ico::IconImage::from_rgba_data(32, 32, pixels);
    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);
    icon_dir.add_entry(ico::IconDirEntry::encode(&image).expect("encode Tauri ico"));
    icon_dir
        .write(File::create(ico_path).expect("create Tauri ico"))
        .expect("write Tauri ico");
}

fn main() {
    ensure_icon();
    tauri_build::build()
}
