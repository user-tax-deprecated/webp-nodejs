use image::EncodableLayout;
use napi::bindgen_prelude::Uint8Array;
use napi_derive::napi;
use webp::Encoder;

#[napi]
pub fn svg_webp(svg: Uint8Array, quality: u8) -> Option<Vec<u8>> {
  let opt = usvg::Options::default();
  let rtree = usvg::Tree::from_data(&svg, &opt.to_ref()).unwrap();
  let pixmap_size = rtree.svg_node().size.to_screen_size();
  let width = pixmap_size.width();
  let height = pixmap_size.height();
  let mut pixmap = tiny_skia::Pixmap::new(width, height).unwrap();
  if resvg::render(
    &rtree,
    usvg::FitTo::Original,
    tiny_skia::Transform::default(),
    pixmap.as_mut(),
  ).is_some() {
    let encoder = Encoder::from_rgba(pixmap.data(), width, height);
    let encoded_webp = encoder.encode(quality as f32);
    return Some(encoded_webp.as_bytes().into());
  }
  None
}
