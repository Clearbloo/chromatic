use clap::{Arg, Command};
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    // Constructor from RGB values
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    // Constructor from HEX code
    pub fn from_hex(hex: &str) -> Result<Self, &'static str> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return Err("Hex code must be 6 characters long");
        }

        let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "Invalid hex code")?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "Invalid hex code")?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "Invalid hex code")?;

        Ok(Self { r, g, b })
    }

    // Constructor from HSV values
    pub fn from_hsv(h: f32, s: f32, v: f32) -> Self {
        let c = v * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;

        let (r_prime, g_prime, b_prime) = match h {
            0.0..=60.0 => (c, x, 0.0),
            60.0..=120.0 => (x, c, 0.0),
            120.0..=180.0 => (0.0, c, x),
            180.0..=240.0 => (0.0, x, c),
            240.0..=300.0 => (x, 0.0, c),
            300.0..=360.0 => (c, 0.0, x),
            _ => (0.0, 0.0, 0.0),
        };

        let r = ((r_prime + m) * 255.0).round() as u8;
        let g = ((g_prime + m) * 255.0).round() as u8;
        let b = ((b_prime + m) * 255.0).round() as u8;

        Self { r, g, b }
    }

    #[allow(dead_code)]
    // Convert to HEX string
    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    #[allow(dead_code)]
    // Convert to RGB tuple
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }

    // Convert to HSV tuple
    pub fn to_hsv(&self) -> (f32, f32, f32) {
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        let h = if delta == 0.0 {
            0.0
        } else if max == r {
            60.0 * (((g - b) / delta) % 6.0)
        } else if max == g {
            60.0 * (((b - r) / delta) + 2.0)
        } else {
            60.0 * (((r - g) / delta) + 4.0)
        };

        let s = if max == 0.0 { 0.0 } else { delta / max };
        let v = max;

        (h.abs(), s, v)
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RGB({}, {}, {})", self.r, self.g, self.b)
    }
}

fn rgb_complement(color: Color) -> Color {
    Color {
        r: 255 - color.r,
        g: 255 - color.g,
        b: 255 - color.b,
    }
}

fn hsv_complement(color: Color) -> Color {
    let (hue, sat, val) = color.to_hsv();
    let new_hue = (hue + 180.) % 360.;
    Color::from_hsv(new_hue, sat, val)
}

fn main() {
    let matches = Command::new("Color CLI")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Calculate complementary colors in RGB, HEX, or HSV")
        .arg(
            Arg::new("rgb")
                .long("rgb")
                .value_names(["R", "G", "B"])
                .help("Input color as RGB values (0-255)")
                .num_args(3),
        )
        .arg(
            Arg::new("hex")
                .long("hex")
                .value_name("HEX")
                .help("Input color as a HEX code (e.g., #RRGGBB)"),
        )
        .arg(
            Arg::new("hsv")
                .long("hsv")
                .value_names(["H", "S", "V"])
                .help("Input color as HSV values (Hue 0-360, Saturation 0-1, Value 0-1)")
                .num_args(3),
        )
        .get_matches();

    let color = if let Some(values) = matches.get_many::<String>("rgb") {
        let values: Vec<u8> = values.map(|v| u8::from_str(v).unwrap()).collect();
        Color::from_rgb(values[0], values[1], values[2])
    } else if let Some(hex) = matches.get_one::<String>("hex") {
        Color::from_hex(hex).expect("Invalid HEX value")
    } else if let Some(values) = matches.get_many::<String>("hsv") {
        let values: Vec<f32> = values.map(|v| f32::from_str(v).unwrap()).collect();
        Color::from_hsv(values[0], values[1], values[2])
    } else {
        eprintln!("No color input provided.");
        return;
    };

    println!("Input Color: {}", color);
    println!(
        "Complementary Color (RGB Complement): {}",
        rgb_complement(color.clone())
    );
    println!(
        "Complementary Color (HSV Complement): {}",
        hsv_complement(color)
    );
}
