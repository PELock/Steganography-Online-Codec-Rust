//! Minimal example: hide a secret message in an image.

use steganography_online_codec::SteganographyOnlineCodec;

#[tokio::main]
async fn main() {
    let my_steganography_online_codec =
        SteganographyOnlineCodec::new(Some("YOUR-WEB-API-KEY".to_string()));

    let input_file = "input_file.jpg";
    let secret_message = "Secret message";
    let password = "Pa$$word";
    let output_file = "output_file_with_hidden_secret_message.png";

    match my_steganography_online_codec
        .encode(input_file, secret_message, password, output_file)
        .await
    {
        Ok(_result) => {
            println!("Secret messaged encoded and saved to the output PNG file.");
        }
        Err(err) => {
            eprintln!(
                "Encoding failed: {}",
                err.error_message()
            );
        }
    }
}
