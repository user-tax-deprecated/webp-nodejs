use image::EncodableLayout;
use napi::bindgen_prelude::Buffer;
use napi_derive::napi;
use tiny_skia::PremultipliedColorU8;
use webp::Encoder;

#[napi]
async fn svg_webp(svg: Buffer, quality: u8) -> napi::Result<Option<Buffer>> {
  let svg = svg.as_ref();
  Ok(
    async move {
      let opt = usvg::Options::default();
      let rtree = usvg::Tree::from_data(svg, &opt.to_ref())?;
      let pixmap_size = rtree.svg_node().size.to_screen_size();
      let width = pixmap_size.width();
      let height = pixmap_size.height();
      if let Some(mut pixmap) = tiny_skia::Pixmap::new(width, height) {
        // 去除透明度（默认是黑底，255-颜色会改为用白底）
        for px in pixmap.pixels_mut() {
          *px =
            PremultipliedColorU8::from_rgba(255 - px.red(), 255 - px.green(), 255 - px.blue(), 255)
              .unwrap();
        }
        if resvg::render(
          &rtree,
          usvg::FitTo::Original,
          tiny_skia::Transform::default(),
          pixmap.as_mut(),
        )
        .is_some()
        {
          let img = pixmap.data();

          let encoder = Encoder::from_rgba(img, width, height);
          let encoded_webp = encoder.encode(quality as f32);
          return Ok(Some(encoded_webp.as_bytes().into()));
        }
      }
      Ok::<_, anyhow::Error>(None)
    }
    .await?,
  )
}
