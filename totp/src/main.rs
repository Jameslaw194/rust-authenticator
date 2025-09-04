use rqrr;
use image::ImageReader;
use google_authenticator_converter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let img = ImageReader::open("qr.png")?.decode()?.to_luma8();

    let mut img = rqrr::PreparedImage::prepare(img);
    let grids = img.detect_grids();

    if grids.is_empty() {
        eprintln!("No QR code found");
        return Ok(());
    }


    let (meta, content) = grids[0].decode()?;
    println!("Decoded content: {}", content);

    if let Ok(ga_qr) = google_authenticator_converter::parse(&content) {
        println!("Issuer: {:?}", ga_qr.issuer);
        println!("Account Name: {:?}", ga_qr.account_name);
        println!("Secret: {:?}", ga_qr.secret);
        println!("Algorithm: {:?}", ga_qr.algorithm);
        println!("Digits: {:?}", ga_qr.digits);
        println!("Period: {:?}", ga_qr.period);
    } else {
        eprintln!("Failed to parse Google Authenticator QR code");
    } 


}
