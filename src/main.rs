use calibright::{CalibrightBuilder, CalibrightConfig, DeviceConfig};
use std::io::{self, Write};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Set up Calibright configuration with default device settings
    let calibright_config = CalibrightConfig::new_with_defaults(&DeviceConfig::default()).await?;
    let mut calibright = CalibrightBuilder::new()
        .with_device_regex(".") // Match all backlight devices by default
        .with_config(calibright_config)
        .build()
        .await?;

    // Get current brightness level
    let current = calibright.get_brightness().await?;
    println!("Current brightness: {:.0}%", current * 100.0);

    // Ask user for new brightness value
    print!("Enter new brightness percentage (0-100): ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let percent: f64 = match input.trim().parse() {
        Ok(v) if v >= 0.0 && v <= 100.0 => v,
        _ => {
            eprintln!("⚠️ Invalid input. Please enter a number between 0 and 100.");
            return Ok(());
        }
    };

    let value = percent / 100.0;
    calibright.set_brightness(value).await?;
    println!("✔️ Brightness set to {:.0}%", percent);

    Ok(())
}
