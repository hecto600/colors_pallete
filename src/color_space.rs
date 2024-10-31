pub struct Color {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
    pub hex_code: String,
}
pub struct ColorsPallete {
    pub color_list: Vec<Color>,
    pub rows_qty: usize,
}

fn render_color(r: usize, g: usize, b: usize) {
    print!("\x1b[48;2;{};{};{}m  \x1b[0m", r, g, b);
    // print!(
    //     "\x1b[48;2;{};{};{}m #{}{}{}  \x1b[0m",
    //     r,
    //     g,
    //     b,
    //     format!("{:02X}", r),
    //     format!("{:02X}", g),
    //     format!("{:02X}", b)
    // );
}

pub fn rgb(bit_exponent: i32) -> ColorsPallete {
    let channel_bits: i32;
    let mut color_data: Vec<Color> = Vec::new();

    if bit_exponent < 1 || bit_exponent > 8 {
        channel_bits = bit_exponent.clamp(1, 8);
    } else {
        channel_bits = bit_exponent;
    }
    let r_size = (2.0 as f64).powi(channel_bits) - 1 as f64;
    let g_size = (2.0 as f64).powi(channel_bits) - 1 as f64;
    let b_size = (2.0 as f64).powi(channel_bits) - 1 as f64;
    const MAX_CHANNEL_SIZE: usize = 256;
    let red_step = MAX_CHANNEL_SIZE / r_size as usize;
    let green_step = MAX_CHANNEL_SIZE / g_size as usize;
    let blue_step = MAX_CHANNEL_SIZE / b_size as usize;

    let mut rows_counter: usize = 0;
    let mut colorspace: Vec<Vec<Vec<(usize, usize, usize)>>> =
        vec![vec![vec![(0, 0, 0); MAX_CHANNEL_SIZE]; MAX_CHANNEL_SIZE]; MAX_CHANNEL_SIZE];

    for r_idx in 0..MAX_CHANNEL_SIZE {
        for g_idx in 0..MAX_CHANNEL_SIZE {
            for b_idx in 0..MAX_CHANNEL_SIZE {
                colorspace[r_idx][g_idx].insert(b_idx, (r_idx, g_idx, b_idx));
            }
        }
    }

    println!("Greyscale line");
    for idx in (0..MAX_CHANNEL_SIZE).step_by(red_step) {
        let (r, g, b) = colorspace[idx as usize][idx as usize][idx as usize];
        render_color(r, g, b);
    }
    println!("");

    for dim in 0..3 {
        //dimension r,g,b
        match dim {
            0 => {
                //rg rv vc
                // println!("Planes: rg, rv, vc");
                for r_idx in (0..MAX_CHANNEL_SIZE).step_by(red_step) {
                    for plane in (1..=256).step_by(127) {
                        for g_idx in (0..MAX_CHANNEL_SIZE).step_by(green_step) {
                            let (r, g, b) = colorspace[r_idx][g_idx][plane - 1];
                            render_color(r, g, b);
                            color_data.push(Color {
                                red: r,
                                green: g,
                                blue: b,
                                hex_code: format!("#{:02X}{:02X}{:02X}", r, g, b),
                            });
                        }
                    }
                    rows_counter += 1;
                    println!("");
                }
            }
            1 => {
                //rb ry yc
                // println!("Planes: rb, ry, yc");
                for r_idx in (0..MAX_CHANNEL_SIZE).step_by(red_step) {
                    for plane in (1..=256).step_by(127) {
                        for b_idx in (0..MAX_CHANNEL_SIZE).step_by(blue_step) {
                            let (r, g, b) = colorspace[r_idx][plane - 1][b_idx];
                            render_color(r, g, b);

                            color_data.push(Color {
                                red: r,
                                green: g,
                                blue: b,
                                hex_code: format!("#{:02X}{:02X}{:02X}", r, g, b),
                            });
                        }
                    }
                    rows_counter += 1;
                    println!("");
                }
            }
            2 => {
                // gb gy yv
                // println!("Planes: gb, gy, yv");
                for g_idx in (0..MAX_CHANNEL_SIZE).step_by(green_step) {
                    for plane in (1..=256).step_by(127) {
                        for b_idx in (0..MAX_CHANNEL_SIZE).step_by(blue_step) {
                            let (r, g, b) = colorspace[plane - 1][g_idx][b_idx];
                            render_color(r, g, b);
                            color_data.push(Color {
                                red: r,
                                green: g,
                                blue: b,
                                hex_code: format!("#{:02X}{:02X}{:02X}", r, g, b),
                            });
                        }
                    }
                    rows_counter += 1;
                    println!("");
                }
            }
            _ => {}
        }
    }

    // println!("\nEverything");
    // for r_idx in (0..MAX_CHANNEL_SIZE).step_by(red_step) {
    //     for g_idx in (0..MAX_CHANNEL_SIZE).step_by(green_step) {
    //         for b_idx in (0..MAX_CHANNEL_SIZE).step_by(blue_step) {
    //             let (r, g, b) = colorspace[r_idx][g_idx][b_idx];
    //             render_color(r, g, b);
    //         }
    //         println!("");
    //     }
    // }
    let rgb_pallete: ColorsPallete = ColorsPallete {
        color_list: color_data,
        rows_qty: rows_counter,
    };
    rgb_pallete
}

pub fn hsl_fixed_l(step_hue: i32, step_sat: i32) {
    let mut hue_prime: f64;
    let mut chroma: f64;
    let mut x: f64;
    let mut rgb: (i32, i32, i32);
    let mut col: i32 = 0;
    let nor_l: f64 = 0.50;
    let mut nor_s: f64 = 0.00;

    for h in (0..360).step_by(step_hue as usize) {
        for _s in (0..=100).step_by(step_sat as usize) {
            chroma = (1.0 - (2.0 * nor_l - 1.0).abs()) * nor_s;

            hue_prime = h as f64 / 60.0;
            x = chroma * (1.0 - ((hue_prime % 2.0) - 1.0).abs());

            let m = nor_l - (chroma / 2.0);

            let mut rgb_f = match hue_prime.trunc() as i32 {
                0..1 => (chroma, x, 0.0),
                1..2 => (x, chroma, 0.0),
                2..3 => (0.0, chroma, x),
                3..4 => (0.0, x, chroma),
                4..5 => (x, 0.0, chroma),
                5..6 => (chroma, 0.0, x),
                _ => (-1.0, -1.0, -1.0),
            };

            rgb_f = (
                (rgb_f.0 + m) * 255.0,
                (rgb_f.1 + m) * 255.0,
                (rgb_f.2 + m) * 255.0,
            );
            rgb = (
                ((rgb_f.0).round() as i32),
                ((rgb_f.1).round() as i32),
                ((rgb_f.2).round() as i32),
            );

            print!(
                "\x1b[48;2;{};{};{}m \x1b[30m#{}{}{} \x1b[0m",
                rgb.0,
                rgb.1,
                rgb.2,
                format!("{:02X}", rgb.0),
                format!("{:02X}", rgb.1),
                format!("{:02X}", rgb.2)
            );
            if col >= 100 / step_sat {
                println!("");
                col = 0;
            } else {
                col += 1;
            }

            nor_s += 0.01 * step_sat as f64;
        }
        nor_s = 0.0;
    }
}

pub fn hsl_rgb(step_hue: i32, step_sat: i32, step_lig: i32) {
    let mut hue_prime: f64;
    let mut chroma: f64;
    let mut x: f64;
    let mut rgb: (i32, i32, i32);
    let mut col: i32 = 0;
    let mut nor_l: f64 = 0.00;
    let mut nor_s: f64 = 0.00;

    for h in (0..360).step_by(step_hue as usize) {
        for _s in (0..100).step_by(step_sat as usize) {
            for _l in (0..100).step_by(step_lig as usize) {
                chroma = (1.0 - (2.0 * nor_l - 1.0).abs()) * nor_s;

                hue_prime = h as f64 / 60.0;
                x = chroma * (1.0 - ((hue_prime % 2.0) - 1.0).abs());

                let m = nor_l - (chroma / 2.0);

                let mut rgb_f = match hue_prime.trunc() as i32 {
                    0..1 => (chroma, x, 0.0),
                    1..2 => (x, chroma, 0.0),
                    2..3 => (0.0, chroma, x),
                    3..4 => (0.0, x, chroma),
                    4..5 => (x, 0.0, chroma),
                    5..6 => (chroma, 0.0, x),
                    _ => (-1.0, -1.0, -1.0),
                };

                rgb_f = (
                    (rgb_f.0 + m) * 255.0,
                    (rgb_f.1 + m) * 255.0,
                    (rgb_f.2 + m) * 255.0,
                );
                rgb = (
                    ((rgb_f.0).round() as i32),
                    ((rgb_f.1).round() as i32),
                    ((rgb_f.2).round() as i32),
                );

                print!("\x1b[48;2;{};{};{}m  \x1b[0m", rgb.0, rgb.1, rgb.2,);
                if col >= 100 {
                    // println!("HSL:{h},{:.2},{:.2} {:?}", s, l, rgb);
                    println!("");
                    col = 0;
                } else {
                    col += step_lig;
                }

                nor_l += 0.01 * step_lig as f64;
            }
            nor_l = 0.0;
            nor_s += 0.01 * step_sat as f64;
        }
        nor_s = 0.0;
    }
}

fn v1() {
    let mut col: u32 = 0;
    let mut h_r;
    let mut h_g;
    let mut h_b;

    for r in 0..256 {
        for g in 0..256 {
            for b in 0..256 {
                h_r = format!("{:X}", r);
                h_g = format!("{:X}", g);
                h_b = format!("{:X}", b);

                print!(
                    "\x1b[48;2;{r};{g};{b}m  #{:0<2}{:0<2}{:0<2} \x1b[0m",
                    h_r, h_g, h_b
                );

                if col == 7 {
                    col = 0;
                    println!("")
                } else {
                    col += 1;
                }
            }
        }
    }
    println!("")
}
