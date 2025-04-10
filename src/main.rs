use calibright::{CalibrightBuilder, CalibrightConfig, DeviceConfig};
use std::io::{self, Write};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // 构建 Calibright 配置
    let calibright_config = CalibrightConfig::new_with_defaults(&DeviceConfig::default()).await?;
    let mut calibright = CalibrightBuilder::new()
        .with_device_regex(".") // 默认匹配所有 backlight 设备
        .with_config(calibright_config)
        .build()
        .await?;

    // 读取当前亮度
    let current = calibright.get_brightness().await?;
    println!("当前亮度: {:.0}%", current * 100.0);

    // 提示用户输入新的亮度值
    print!("请输入新的亮度百分比 (0 - 100): ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let percent: f64 = match input.trim().parse() {
        Ok(v) if v >= 0.0 && v <= 100.0 => v,
        _ => {
            eprintln!("⚠️ 输入无效，请输入 0 到 100 之间的数字。");
            return Ok(());
        }
    };

    let value = percent / 100.0;
    calibright.set_brightness(value).await?;
    println!("✔️ 亮度已设置为 {:.0}%", percent);

    Ok(())
}

