use blend2d::{Path, Image, Context, BLFormat};
use ffi::BLCompOp;

fn main() {
    let mut img = Image::new(480, 480, BLFormat::BL_FORMAT_PRGB32).expect("Unable to create image");

    // Attach a rendering context into `img`.
    let mut ctx = Context::new();
    ctx.begin(&mut img).expect("Unable to attach rendering context");
    // The closure here just acts as a `try` block to catch possible errors
    let render = |mut ctx: Context| {
        // Clear the image.
        ctx.set_comp_op(BLCompOp::BL_COMP_OP_SRC_COPY)?;
        ctx.fill_all()?;

        // Fill some path.
        let mut path = Path::new();
        path.move_to(26.0, 31.0)?;
        path.cubic_to(642.0, 132.0, 587.0, -136.0, 25.0, 464.0)?;
        path.cubic_to(882.0, 404.0, 144.0, 267.0, 27.0, 31.0)?;

        ctx.set_comp_op(BLCompOp::BL_COMP_OP_SRC_OVER)?;
        ctx.set_fill_style_rgba32(0xFFFFFFFF)?;
        ctx.fill_path(&path)?;

        // Detach the rendering context from `img`.
        ctx.end()
    };
    render(ctx).expect("Rendering to context failed");
    img.write_to_file(c"bl-getting-started-1.bmp")
    .expect("Writing to file failed");
}
