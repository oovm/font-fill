use font_fill::{FontFillResult, FontFillVideo, Rgba};

fn main() -> FontFillResult<()> {
    let font = r"F:\project-a\client\ProjectA\Assets\Art\Fonts\SourceHanSansSC-Regular.otf";
    let mut file = FontFillVideo::create("../../target/SourceHanSansSC/out.av1", font, 200).unwrap();
    for c in include_str!("../../font-fill/tests/长恨歌.txt").chars() {
        if !c.is_alphabetic() {
            continue;
        }
        println!("encode: {}", c);
        file.encode_frame(c, Rgba([1.0, 1.0, 1.0, 1.0]))?;
    }
    file.report_json()
}
