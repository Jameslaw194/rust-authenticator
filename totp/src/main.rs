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

    match google_authenticator_converter::process_data(&content) {
        Ok(accounts) => {
            for account in accounts {
                println!("Issuer: {:?}", account.issuer);
                println!("Account Name: {:?}", account.name);
                println!("Secret: {:?}", account.secret);
            }
            Ok(())
        }
        
        Err(e) => {
            eprintln!("Failed to parse Google Authenticator QR code: {}", e);
            Ok(())
        }
    }


}