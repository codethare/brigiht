use std::io::{self, Write};
use std::process::Command;

fn main() {
    // 提示用户输入亮度百分比
    print!("请输入亮度百分比 (0-100): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let percent: u8 = match input.trim().parse() {
        Ok(val) if val <= 100 => val,
        _ => {
            eprintln!("输入无效，请输入 0 到 100 之间的数字！");
            return;
        }
    };

    let brightness = (percent as f32) / 100.0;

    // 获取当前连接的显示器名
    let output = Command::new("xrandr")
        .arg("--current")
        .output()
        .expect("无法运行 xrandr");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let display_line = stdout.lines()
        .find(|line| line.contains(" connected"))
        .expect("未检测到连接的显示器");

    let display_name = display_line.split_whitespace().next().unwrap();

    // 设置亮度
    let status = Command::new("xrandr")
        .args(["--output", display_name, "--brightness", &brightness.to_string()])
        .status()
        .expect("无法设置亮度");

    if status.success() {
        println!("亮度已设置为 {}%", percent);
    } else {
        eprintln!("设置亮度失败，请确保安装了 xrandr 并在图形环境下运行");
    }
}

