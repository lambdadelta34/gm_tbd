use crate::Unit;

pub struct Settings {
    pixels_per_unit: f32,
    pub unit_graphics_ratio: f32,
    pub camera_scale: f32,
}

impl Settings {
    pub fn update_camera_scale(&mut self, new_scale: f32) {
        self.camera_scale = new_scale;
        self.unit_graphics_ratio = self.unit_pixel_ratio();
    }

    pub fn update_pixels_per_unit(&mut self, new_value: f32) {
        self.pixels_per_unit = new_value;
        self.unit_graphics_ratio = self.unit_pixel_ratio();
    }

    fn unit_pixel_ratio(&self) -> f32 {
        self.pixels_per_unit / self.camera_scale
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            pixels_per_unit: 50.,
            unit_graphics_ratio: 50.,
            camera_scale: 1.,
        }
    }
}

pub fn units_to_graphics(units: Unit, settings: &Settings) -> f32 {
    units * settings.unit_graphics_ratio
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_saves_camera_scale() {
        let mut settings = Settings::default();
        settings.update_camera_scale(2.);
        assert_eq!(settings.unit_graphics_ratio, 25.);
    }

    #[test]
    fn correctly_saves_pixels_per_unit() {
        let mut settings = Settings::default();
        settings.update_pixels_per_unit(1000.);
        assert_eq!(settings.unit_graphics_ratio, 1000.);
    }
}
