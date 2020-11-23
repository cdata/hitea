use embedded_graphics::{
  draw_target::DrawTarget,
  pixelcolor::{Rgb565, RgbColor},
  primitives::Rectangle,
  Drawable,
};

pub trait Sampler {
  fn get_pixel(&self, x: u32, y: u32) -> Option<Rgb565>;

  fn bounds(&self) -> &Rectangle;
}

// type FallbackSample = Fn(u32, u32) -> Rgb565;

pub struct PixelSource<'a, T: Sampler>(pub &'a T, pub Option<&'a dyn Fn(u32, u32) -> Rgb565>);

impl<'a, T> Drawable for PixelSource<'a, T>
where
  T: Sampler,
{
  type Color = Rgb565;

  fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
  where
    D: DrawTarget<Color = Self::Color>,
  {
    let bounds = self.0.bounds();
    let optional_fallback = &self.1;

    let max_index = bounds.size.width * bounds.size.height;
    let sample = |index: u32| -> Rgb565 {
      let x = index % bounds.size.width;
      let y = index / bounds.size.width;

      match self.0.get_pixel(x, y) {
        Some(color) => color,
        None => match optional_fallback {
          Some(fallback) => fallback(x, y),
          None => Rgb565::WHITE,
        },
      }
    };

    display.fill_contiguous(bounds, SamplerIterator::new(sample, max_index))
  }
}

pub struct SamplerIterator<F>
where
  F: Fn(u32) -> Rgb565,
{
  index: u32,
  max_index: u32,
  sample: F,
}

impl<F> SamplerIterator<F>
where
  F: Fn(u32) -> Rgb565,
{
  pub fn new(sample: F, max_index: u32) -> SamplerIterator<F> {
    SamplerIterator {
      index: 0,
      max_index,
      sample,
    }
  }
}

impl<F> Iterator for SamplerIterator<F>
where
  F: Fn(u32) -> Rgb565,
{
  type Item = Rgb565;

  fn next(&mut self) -> Option<Self::Item> {
    if self.index >= self.max_index {
      return None;
    }

    let sample = &self.sample;
    let color = sample(self.index);

    self.index += 1;

    Some(color)
  }
}
