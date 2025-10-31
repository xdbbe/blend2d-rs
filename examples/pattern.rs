use blend2d::{Context, Image, Pattern, context::CompOp, image};

fn main() {
    let mut img = Image::new(480, 480, image::Format::PRgb32).expect("Unable to create image");
    Context::render(&mut img, |ctx| {
        ctx.set_comp_op(CompOp::SrcCopy)?;
        ctx.fill_all()?;

        // Read an image from file.
        let texture = Image::read_from_file(c"assets/ferris.png")?;

        // Create a pattern and use it to fill a rounded-rect.
        let pattern = Pattern::new(&texture, None, Default::default(), None)?;

        ctx.set_comp_op(CompOp::SrcOver)?;
        // Draw a solid background.
        ctx.set_fill_style_rgba32(0xFFFFFFFF)?;
        ctx.fill_round_rect(40.0, 40.0, 400.0, 400.0, 45.5, 45.5)?;
        // Draw the pattern.
        ctx.set_fill_style_pattern(&pattern)?;
        ctx.fill_round_rect(40.0, 40.0, 400.0, 400.0, 45.5, 45.5)
    })
    .expect("Rendering to context failed");

    img.write_to_file(c"bl-getting-started-3.bmp")
        .expect("Writing to file failed");
}
