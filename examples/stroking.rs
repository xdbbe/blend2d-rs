use blend2d::{
    CompOp, Context, ExtendMode, Gradient, Image, geometry::StrokeCap,
    gradient::LinearGradientValues, image, path::Path,
};

fn main() {
    let mut img = Image::new(480, 480, image::Format::PRgb32).expect("Unable to create image");
    Context::render(&mut img, |ctx| {
        ctx.set_comp_op(CompOp::SrcCopy)?;
        ctx.fill_all()?;

        let mut linear = Gradient::new_linear(
            &LinearGradientValues {
                x0: 0.0,
                y0: 0.0,
                x1: 0.0,
                y1: 480.0,
            },
            ExtendMode::PadXPadY,
        );
        linear.add_stop_rgba32(0.0, 0xFFFFFFFF)?;
        linear.add_stop_rgba32(0.5, 0xFFFF1F7F)?;
        linear.add_stop_rgba32(1.0, 0xFF1F7FFF)?;

        let mut path = Path::default();
        path.move_to(119.0, 49.0)?;
        path.cubic_to(259.0, 29.0, 99.0, 279.0, 275.0, 267.0)?;
        path.cubic_to(537.0, 245.0, 300.0, -170.0, 274.0, 430.0)?;

        ctx.set_comp_op(CompOp::SrcOver)?;
        ctx.set_stroke_style_gradient(&linear)?;
        ctx.set_stroke_width(15.0)?;
        ctx.set_stroke_start_cap(StrokeCap::Round)?;
        ctx.set_stroke_end_cap(StrokeCap::Butt)?;
        ctx.stroke_path(&path)
    })
    .expect("Rendering to context failed");

    img.write_to_file(c"bl-getting-started-6.bmp")
        .expect("Writing to file failed");
}
