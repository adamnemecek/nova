use super::image::{self, Image};

pub struct Texture {
  image: Arc<Image>,
  sampler: Arc<image::Sampler>,
}

impl Texture {
  pub fn new(
    descriptor_pool: &mut pipeline::DescriptorPool,
    image: &Arc<Image>,
    sampler: &Arc<Sampler>,
  ) -> Texture {
    Texture {
      image: image.clone(),
      sampler: sampler.clone(),
    }
  }
}
