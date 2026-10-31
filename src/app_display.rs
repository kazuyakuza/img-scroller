//! Display state and paint step of the central image surface: texture lifecycle
//! (one upload per successful decode) and the fit-scaled centered render.

use eframe::egui;

use crate::image_loading::DecodedImage;
use crate::scaling;

const CURRENT_IMAGE_TEXTURE_NAME: &str = "current_image";

/// Texture state of the image shown in the central surface. A successful open
/// flags a pending upload; the next paint uploads exactly once and clears it.
pub(crate) struct CurrentImageDisplay {
    image_texture: Option<egui::TextureHandle>,
    texture_upload_pending: bool,
}

impl CurrentImageDisplay {
    pub(crate) fn initial() -> Self {
        Self {
            image_texture: None,
            texture_upload_pending: false,
        }
    }

    /// A successful decode replaced the decoded payload: the next paint uploads it.
    pub(crate) fn accept_decoded_image(&mut self) {
        self.texture_upload_pending = true;
    }

    /// Paints the decoded image fit-scaled and centered on the given surface.
    pub(crate) fn paint(&mut self, panel_ui: &mut egui::Ui, decoded_image: &DecodedImage) {
        self.ensure_current_image_texture(panel_ui.ctx(), decoded_image);
        let Some(image_texture) = self.image_texture.as_ref() else {
            return;
        };
        let Some(display_size) =
            scaling::fitted_size(panel_ui.available_size(), image_dimensions(decoded_image))
        else {
            return;
        };
        panel_ui.with_layout(
            egui::Layout::centered_and_justified(egui::Direction::TopDown),
            |image_ui| {
                image_ui.add(egui::Image::new(image_texture).fit_to_exact_size(display_size));
            },
        );
    }

    /// Uploads the decoded pixels as a texture only while a new decode is pending,
    /// replacing the previous handle by assignment (the dropped handle frees the
    /// old texture). Never runs more than once per successful decode.
    fn ensure_current_image_texture(&mut self, ctx: &egui::Context, decoded_image: &DecodedImage) {
        if !self.texture_upload_pending {
            return;
        }
        let image_size = [decoded_image.width as usize, decoded_image.height as usize];
        let color_image =
            egui::ColorImage::from_rgba_unmultiplied(image_size, &decoded_image.pixels);
        self.image_texture = Some(ctx.load_texture(
            CURRENT_IMAGE_TEXTURE_NAME,
            color_image,
            egui::TextureOptions::default(),
        ));
        self.texture_upload_pending = false;
    }
}

/// Image dimensions as `egui::Vec2`, shared by the one `fitted_size` call in `paint`.
fn image_dimensions(decoded_image: &DecodedImage) -> egui::Vec2 {
    egui::Vec2::new(decoded_image.width as f32, decoded_image.height as f32)
}

#[cfg(test)]
mod tests {
    use super::CurrentImageDisplay;
    use crate::image_loading::DecodedImage;
    use eframe::egui;

    #[test]
    fn initial_display_has_no_texture_and_no_pending_upload() {
        let display = CurrentImageDisplay::initial();
        assert!(display.image_texture.is_none());
        assert!(!display.texture_upload_pending);
    }

    #[test]
    fn successful_upload_happens_exactly_once_per_decoded_image() {
        let mut display = CurrentImageDisplay::initial();
        display.accept_decoded_image();
        assert!(display.texture_upload_pending);

        let decoded_image = DecodedImage {
            width: 2,
            height: 2,
            pixels: vec![0; 16],
        };
        let context = egui::Context::default();
        display.ensure_current_image_texture(&context, &decoded_image);
        assert!(display.image_texture.is_some());
        assert!(!display.texture_upload_pending);
        let uploaded_texture = display.image_texture.clone().expect("uploaded texture");

        display.ensure_current_image_texture(&context, &decoded_image);
        assert!(display.image_texture == Some(uploaded_texture));
    }
}
