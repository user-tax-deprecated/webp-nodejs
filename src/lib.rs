use image::EncodableLayout;
use napi::{
  bindgen_prelude::{AsyncTask, Uint8Array},
  Env, Result, Task,
};
use napi_derive::napi;
use tiny_skia::PremultipliedColorU8;
use webp::Encoder;

struct SvgWebp {
  quality: u8,
  svg: Uint8Array,
}

impl Task for SvgWebp {
  type Output = Option<Vec<u8>>;
  type JsValue = Option<Uint8Array>;

  fn compute(&mut self) -> Result<Self::Output> {
    let opt = usvg::Options::default();
    if let Ok(rtree) = usvg::Tree::from_data(&self.svg, &opt.to_ref()) {
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
          let encoded_webp = encoder.encode(self.quality as f32);
          return Ok(Some(encoded_webp.as_bytes().into()));
        }
      }
    }
    Ok(None)
  }

  fn resolve(&mut self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(match output {
      Some(o) => Some(Uint8Array::new(o)),
      None => None,
    })
  }
}

#[napi]
fn svg_webp(svg: Uint8Array, quality: u8) -> AsyncTask<SvgWebp> {
  AsyncTask::new(SvgWebp { svg, quality })
}

/*
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
*/
