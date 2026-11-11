//! Display state and paint step of the central image surface: texture lifecycle
//! (one upload per successful decode) and the fit-scaled paint composed with
//! the clamped pan offset.

use eframe::egui;

use crate::image_loading::DecodedImage;
use crate::pan;
use crate::scaling;
use crate::zoom;

const CURRENT_IMAGE_TEXTURE_NAME: &str = "current_image";

/// The 3-argument paint request (project max-args rule): WHICH decoded image
/// is on screen, WHAT zoom factor scales the whole-image fit base into the
/// draw size, and the session pan offset composed onto that draw rect.
pub(crate) struct ImagePainting<'a> {
    pub(crate) decoded_image: &'a DecodedImage,
    pub(crate) zoom_factor: f32,
    pub(crate) pan_offset: egui::Vec2,
}

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

    /// Paints the decoded image at `fit * zoom`, repositioned by the clamped
    /// pan offset. The offset is clamped per axis so an overflowing axis never
    /// shows a gap or overscan; a fitting axis stays centered; the panel clip
    /// rect hides anything beyond. No texture re-upload: the offset only
    /// repositions the rect (one upload per successful decode remains).
    pub(crate) fn paint(&mut self, panel_ui: &mut egui::Ui, painting: ImagePainting) {
        self.ensure_current_image_texture(panel_ui.ctx(), painting.decoded_image);
        let Some(image_texture) = self.image_texture.as_ref() else {
            return;
        };
        let panel_rect = panel_ui.available_rect_before_wrap();
        let Some(fit_size) = scaling::fitted_size(panel_rect.size(), painting.decoded_image.size())
        else {
            return;
        };
        let display_size = zoom::zoomed_draw_size(fit_size, painting.zoom_factor);
        let overflow = pan::axis_overflow_pair(display_size, panel_rect.size());
        let clamped_offset = pan::clamped_pan_offset(painting.pan_offset, overflow);
        let centered_min = panel_rect.center() - display_size * 0.5;
        let image_rect = egui::Rect::from_min_size(centered_min - clamped_offset, display_size);
        panel_ui.put(
            image_rect,
            egui::Image::new(image_texture).fit_to_exact_size(display_size),
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
