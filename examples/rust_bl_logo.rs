use blend2d::{
    CompOp, Context, ExtendMode, Gradient, Image, Pattern,
    geometry::{Matrix2D, SizeI},
    gradient::{LinearGradientValues, RadialGradientValues},
    image,
};

fn main() {
    let mut img = Image::new(480, 480, image::Format::PRgb32).expect("Unable to create image");
    Context::render(&mut img, |ctx| {
        let c_r = 160.0;
        let c_x = 180.0;
        let c_y = 180.0;
        // Clear the image.
        ctx.set_comp_op(CompOp::SrcCopy)?;
        ctx.fill_all()?;

        // Draw a circle with a red-white radial gradient.
        let mut radial = Gradient::new_radial(
            &RadialGradientValues {
                x0: c_x,
                y0: c_y,
                x1: c_x,
                y1: c_y,
                r0: c_r,
                r1: 0.0,
            },
            ExtendMode::PadXPadY,
        );
        radial.add_stop_rgba64(0.0, 0xFFFFFFFFFFFFFFFF)?;
        radial.add_stop_rgba64(1.0, 0xFFFFFFFF6F6F3F3F)?;
        ctx.set_comp_op(CompOp::SrcOver)?;
        ctx.set_fill_style_gradient(&radial)?;
        ctx.fill_circle(c_x, c_y, c_r)?;

        // Multiply a circle with our logo scaled to the radius of the circle on top of
        // your image.
        let mut logo = Image::read_from_file(c"assets/rust-logo-512x512-blk.png")?;
        logo.scale(
            SizeI {
                w: 2 * c_r as i32,
                h: 2 * c_r as i32,
            },
            image::ScaleFilter::Bicubic,
        )?;
        let pattern = Pattern::new(
            &logo,
            None,
            Default::default(),
            &Matrix2D::translation(20.0, 20.0),
        )?;
        ctx.set_comp_op(CompOp::Multiply)?;
        ctx.set_fill_style_pattern(&pattern)?;
        ctx.fill_circle(c_x, c_y, c_r)?;

        // Draw the difference of a square with a blue-white linear gradient to the
        // image with regards to the image.
        let mut linear = Gradient::new_linear(
            &LinearGradientValues {
                x0: 195.0,
                y0: 195.0,
                x1: 470.0,
                y1: 470.0,
            },
            ExtendMode::PadXPadY,
        );
        linear.add_stop_rgba64(0.0, 0xFFFFFFFFFFFFFFFF)?;
        linear.add_stop_rgba64(1.0, 0xFFFF3F3F9F9FFFFF)?;

        ctx.set_comp_op(CompOp::Difference)?;
        ctx.set_fill_style_gradient(&linear)?;
        ctx.fill_round_rect(195.0, 195.0, 270.0, 270.0, 25.0, 25.0)
    })
    .expect("Rendering to context failed");

    img.write_to_file(c"rust_bl_logo.bmp")
        .expect("Writing to file failed");
}
