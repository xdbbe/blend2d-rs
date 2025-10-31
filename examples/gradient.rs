use blend2d::{
    Context, ExtendMode, Gradient, Image, context::CompOp, gradient::LinearGradientValues, image,
};

fn main() {
    let mut img = Image::new(480, 480, image::Format::PRgb32).expect("Unable to create image");

    // Attach a rendering context into `img`.
    Context::render(&mut img, |ctx| {
        ctx.set_comp_op(CompOp::SrcCopy)?;
        ctx.fill_all()?;

        // Coordinates can be specified now or changed later.
        let mut linear = Gradient::new_linear(
            &LinearGradientValues {
                x0: 0.0,
                y0: 0.0,
                x1: 0.0,
                y1: 480.0,
            },
            ExtendMode::PadXPadY,
        );

        // Color stops can be added in any order.
        linear.add_stop_rgba32(0.0, 0xFFFFFFFF)?;
        linear.add_stop_rgba32(0.5, 0xFF5FAFDF)?;
        linear.add_stop_rgba32(1.0, 0xFF2F5FDF)?;

        // `setFillStyle()` can be used for both colors and styles.
        ctx.set_fill_style_gradient(&linear)?;

        ctx.set_comp_op(CompOp::SrcOver)?;
        ctx.fill_round_rect(40.0, 40.0, 400.0, 400.0, 45.5, 45.5)
    })
    .expect("Rendering to context failed");

    img.write_to_file(c"bl-getting-started-2.bmp")
        .expect("Writing to file failed");
}
