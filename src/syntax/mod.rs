use egui::{text::LayoutJob, Color32, FontId, TextFormat};

/// Very basic syntax highlighting for the prototype
pub struct HighlightOptions {
    pub font_size: f32,
    pub text_color: Color32,
    pub keyword_color: Color32,
    pub comment_color: Color32,
    pub heading_color: Color32,
}

impl Default for HighlightOptions {
    fn default() -> Self {
        Self {
            font_size: 14.0,
            text_color: Color32::from_rgb(220, 223, 228),
            keyword_color: Color32::from_rgb(198, 120, 221),
            comment_color: Color32::from_rgb(92, 99, 112),
            heading_color: Color32::from_rgb(229, 192, 123),
        }
    }
}

/// Basic highlighter function for prototype
pub fn basic_highlight(text: &str, options: &HighlightOptions) -> LayoutJob {
    let mut job = LayoutJob::default();

    // Just highlight a few basic things for the prototype
    for line in text.lines() {
        // Highlight headings in markdown
        if line.starts_with("# ") {
            job.append(
                line,
                0.0,
                TextFormat {
                    font_id: FontId::proportional(options.font_size * 1.5),
                    color: options.heading_color,
                    ..Default::default()
                },
            );
        }
        // Highlight comments
        else if line.trim_start().starts_with("//") {
            job.append(
                line,
                0.0,
                TextFormat {
                    font_id: FontId::monospace(options.font_size),
                    color: options.comment_color,
                    ..TextFormat::default()
                },
            );
        }
        // Default formatting
        else {
            job.append(
                line,
                0.0,
                TextFormat {
                    font_id: FontId::monospace(options.font_size),
                    color: options.text_color,
                    ..Default::default()
                },
            );
        }
        job.append("\n", 0.0, TextFormat::default());
    }

    job
}
