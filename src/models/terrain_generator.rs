use crate::utils::gen_murmur3_f32;

struct HeightsGenerator {
    seed: u32,
}

impl HeightsGenerator {
    const AMPLITUDE: f32 = 70.0;

    fn new() -> Self {
        let seed = 0x00c0ffee;
        HeightsGenerator {
            seed,
        }
    }

    pub fn get_smooth_noise(&self, x: i32, y: i32) -> f32 {
        // we do gaussian blurring here to smoothen the noise
        // to do so we use a 3x3 gaussian blur which approximates the weights 
        // that would be obtained from a sampling of normal distribution
        let corners = self.get_noise(x-1,y-1) + self.get_noise(x-1,y+1) + self.get_noise(x+1,y-1) + self.get_noise(x+1,y+1);
        let edges = 2.0 * (self.get_noise(x-1,y) + self.get_noise(x+1,y) + self.get_noise(x,y-1) + self.get_noise(x,y+1));
        let center = 4.0 * self.get_noise(x, y);
        (center + edges + corners) / 16.0
    }

    fn get_noise(&self, x: i32, y: i32) -> f32 {
        let hash = gen_murmur3_f32(x as u32, y as u32, self.seed);
        hash * HeightsGenerator::AMPLITUDE
    }
}