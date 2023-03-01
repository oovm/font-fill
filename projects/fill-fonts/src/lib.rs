

#[test]
fn test() {
    let font = SystemSource::new()
        .select_by_postscript_name("ArialMT")
        .unwrap()
        .load()
        .unwrap();

    let glyph_id = font.glyph_for_char('A').unwrap();
    let mut canvas = Canvas::new(&Size2D::new(32, 32), Format::A8);

    font.rasterize_glyph(
        &mut canvas,
        glyph_id,
        32.0,
        &Point2D::new(0.0, 32.0),
        HintingOptions::None,
        RasterizationOptions::GrayscaleAa,
    )
        .unwrap();
}