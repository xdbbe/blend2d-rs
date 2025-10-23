use blend2d::{Image, Context, BLCompOp, BLFormat, Gradient};

fn main() {
    let mut img = Image::new(480, 480, BLFormat::BL_FORMAT_PRGB32).expect("Unable to create image");

    // Attach a rendering context into `img`.
    let mut ctx = Context::new();
    ctx.begin(&mut img).expect("Unable to attach rendering context");
    let render = |mut ctx: Context| {
        ctx.set_comp_op(BLCompOp::BL_COMP_OP_SRC_COPY)?;
        ctx.fill_all()?;

        // Coordinates can be specified now or changed later.
        let mut linear = Gradient::new();
        // let mut linear = LinearGradient::new(
        //     &LinearGradientValues {
        //         x0: 0.0,
        //         y0: 0.0,
        //         x1: 0.0,
        //         y1: 480.0,
        //     },
        //     ExtendMode::PadXPadY,
        //     &[],
        //     None,
        // );

        // Color stops can be added in any order.
        linear.add_stop_rgba32(0.0, 0xFFFFFFFF)?;
        linear.add_stop_rgba32(0.5, 0xFF5FAFDF)?;
        linear.add_stop_rgba32(1.0, 0xFF2F5FDF)?;

        // `setFillStyle()` can be used for both colors and styles.
        ctx.set_fill_style_gradient(&linear)?;

        ctx.set_comp_op(BLCompOp::BL_COMP_OP_SRC_OVER)?;
        ctx.fill_round_rect(40.0, 40.0, 400.0, 400.0, 45.5, 45.5)?;
        ctx.end()
    };
    render(ctx).expect("Rendering to context failed");

    img.write_to_file(c"bl-getting-started-2.bmp")
    .expect("Writing to file failed");
}
