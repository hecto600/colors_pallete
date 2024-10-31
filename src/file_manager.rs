use std::{fs::File, io::Write};

use crate::color_space::ColorsPallete;

pub fn create_file(colors_pallete: ColorsPallete) -> std::io::Result<()> {
    // let svg_file_path = ;
    let mut svg_file = File::create("output/colors.svg")?;

    let square_size = 50;
    let squares_per_row = colors_pallete.color_list.len() / colors_pallete.rows_qty as usize;
    let svg_width = square_size * squares_per_row;
    let svg_height = square_size * colors_pallete.rows_qty;

    let mut svg_content = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}">"#,
        svg_width, svg_height
    );

    println!(
        "color_list len:{} rows_qty:{} squares_per_row:{}",
        colors_pallete.color_list.len(),
        colors_pallete.rows_qty,
        squares_per_row
    );

    for (i, c) in colors_pallete.color_list.iter().enumerate() {
        let x = (i % squares_per_row) * square_size;
        let y = (i / squares_per_row) * square_size;
        svg_content.push_str(&format!(
            r#"<rect x="{}" y="{}" width="{}" height="{}" fill="rgb({},{},{})" />"#,
            x, y, square_size, square_size, c.red, c.green, c.blue
        ));
    }

    svg_content.push_str("</svg>");
    svg_file.write_all(svg_content.as_bytes())?;

    let mut html_content = String::from(
        r#"
    <!DOCTYPE html>  
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Color swatches</title>
        <link rel="stylesheet" href="colors.css">
    </head>
    <body>
    <h1>Color swatches</h1>
    <object type="image/svg+xml" data="colors.svg"></object>

    <style>
    body {background-color: #202020}
    h1 {color:#fefefe}
    </style>
    "#,
    );

    html_content.push_str("</body></html>");

    let html_file_path = "output/index.html";
    let mut html_file = File::create(html_file_path)?;
    html_file.write_all(html_content.as_bytes())?;

    Ok(())
}
