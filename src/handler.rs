use std::fs::File;
use std::io::prelude::*;
use std::io::Error as IOError;
use std::io::Write;
use std::path::{Path, PathBuf};

use orgize::export::{DefaultHtmlHandler, HtmlHandler, SyntectHtmlHandler};
use orgize::Element;

// const THEME: &'static str = "base16-ocean.light";
const THEME: &'static str = "Solarized (light)";

pub struct SlidesHtmlHandler {
    inner: SyntectHtmlHandler<IOError, DefaultHtmlHandler>,
    current_slide: u32,
    current_table_row: u32,
    presentation_path: PathBuf,
}

impl SlidesHtmlHandler {
    pub fn new(presentation_path: &Path) -> Self {
        let inner = SyntectHtmlHandler {
            theme: THEME.to_string(),
            ..SyntectHtmlHandler::default()
        };

        SlidesHtmlHandler {
            inner,
            current_slide: 0,
            current_table_row: 0,
            presentation_path: presentation_path.to_path_buf(),
        }
    }
}

impl Default for SlidesHtmlHandler {
    fn default() -> Self {
        Self::new(&Path::new(""))
    }
}

impl HtmlHandler<IOError> for SlidesHtmlHandler {
    fn start<W: Write>(&mut self, mut w: W, element: &Element<'_>) -> Result<(), IOError> {
        match element {
            Element::Title(title) if title.level == 1 => {
                // Intro slide
                write!(w, r#"<div class="slide intro" id="intro">"#,)?;
            }
            Element::Title(title) if title.level == 2 => {
                if let Some(layout) = title.properties.get("SLIDE_LAYOUT") {
                    if layout == "no-title" {
                        // Hide title and skip default implementation
                        return write!(w, r#"<h2 class="hidden">"#);
                    } else {
                        eprintln!("Unsupported slide layout: '{}'", layout);
                    }
                }
            }
            Element::Headline { level } if *level == 2 => {
                // New slide
                self.current_slide += 1;
                write!(
                    w,
                    r#"<div class="slide" id="slide-{}">"#,
                    self.current_slide
                )?;
            }
            Element::Table(_) => {
                self.current_table_row = 0;
                write!(w, "<table>")?;
            }
            Element::TableRow(_) => {
                self.current_table_row += 1;
                if self.current_table_row == 1 {
                    // This is the header
                    write!(w, "<thead>")?;
                }
                write!(w, "<tr>")?;
            }
            Element::TableCell => {
                if self.current_table_row == 1 {
                    write!(w, "<th>")?;
                } else {
                    write!(w, "<td>")?;
                }
            }
            Element::Link(link) => {
                let path = match self.presentation_path.parent() {
                    Some(parent) => parent.join(&*link.path),
                    None => Path::new(&*link.path).to_path_buf(),
                };

                if let Some(ext) = path.extension() {
                    if ext == "png" || ext == "jpg" || ext == "jpeg" || ext == "gif" {
                        // Read image and encode to base64
                        let mut image = match File::open(&path) {
                            Ok(image) => image,
                            Err(err) => {
                                // Fallback to default handler if failed
                                eprintln!(
                                    "Failed to open file '{}' with error '{}'",
                                    path.display(),
                                    err
                                );
                                return self.inner.start(w, element);
                            }
                        };
                        let mut contents = vec![];
                        image.read_to_end(&mut contents)?;

                        let encoded = base64::encode(&contents);
                        let content_type = if ext == "png" {
                            "image/png"
                        } else if ext == "gif" {
                            "image/gif"
                        } else {
                            "image/jpeg"
                        };

                        return write!(
                            w,
                            r#"<img src="data:{};base64, {}" alt="{}" />"#,
                            content_type,
                            encoded,
                            path.display()
                        );
                    }
                }
            }
            Element::Keyword(keyword) => {
                write!(
                    w,
                    r#"<div class="keyword"><label>{}:</label> <span>{}</span></div>"#,
                    keyword.key.to_lowercase(),
                    keyword.value
                )?;
            }
            _ => {}
        }

        // fallthrough to default handler
        self.inner.start(w, element)
    }

    fn end<W: Write>(&mut self, mut w: W, element: &Element<'_>) -> Result<(), IOError> {
        match element {
            Element::Title(title) if title.level == 1 => {
                // Intro slide ended
                if let Some(author) = title.properties.get("AUTHOR") {
                    write!(w, r#"<h3 class="secondary">{}</h3>"#, author)?;
                }
                write!(w, "</div>")?;
            }
            Element::Headline { level } if *level == 2 => {
                // Slide ended
                write!(w, "</div>")?;
            }
            Element::Table(_) => {
                write!(w, "</table>")?;
            }
            Element::TableRow(_) => {
                write!(w, "</tr>")?;
                if self.current_table_row == 1 {
                    write!(w, "</thead>")?;
                }
            }
            Element::TableCell => {
                if self.current_table_row == 1 {
                    write!(w, "</th>")?;
                } else {
                    write!(w, "</td>")?;
                }
            }
            _ => {}
        }

        self.inner.end(w, element)?;
        Ok(())
    }
}
