use std::env;
use qrcode::QrCode;
use image::Luma;
use sha256::digest;

fn parse_arguments(args: Vec<String>) -> Result<String, &'static str> {
    if args.len() > 1 {
        return generate_qr_code(args[1].to_string());
    } else {
        return Err("No file data argument given");
    }
}

fn generate_qr_code(data: String) -> Result<String, &'static str> {
    if data.len() > 0 {
        let code = QrCode::new(data.as_bytes()).unwrap();
        let image = code.render::<Luma<u8>>().build();
        let mut hash_name = digest(data);
        hash_name.push_str(".png");
        image.save(&hash_name).unwrap();
        return Ok(hash_name);
    } else {
        return Err("No data length is 0");
    }
}

#[cfg(not(tarpaulin_include))]
fn main() {
    let args: Vec<String> = env::args().collect();
    let _ = parse_arguments(args);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_arguments_with_data() {
        let args = vec!["program".to_string(), "image".to_string()];
        let file = parse_arguments(args);
        assert_eq!(file, Ok("6105d6cc76af400325e94d588ce511be5bfdbb73b437dc51eca43917d7a43e3d.png".to_string()));
    }

    #[test]
    fn test_parse_arguments_without_data() {
        let args = vec!["program".to_string()];
        let file = parse_arguments(args);
        assert!(file.is_err());
    }

    #[test]
    fn test_generate_qr_code() {
        let file = generate_qr_code("image".to_string());
        assert_eq!(file, Ok("6105d6cc76af400325e94d588ce511be5bfdbb73b437dc51eca43917d7a43e3d.png".to_string()));
    }

    #[test]
    fn test_generate_qr_code_with_no_data() {
        let file = generate_qr_code("".to_string());
        assert!(file.is_err());
    }
}
