use std::io::Write;
use std::io::Error as IOError;

use orgize::export::{DefaultHtmlHandler, SyntectHtmlHandler, HtmlHandler};
use orgize::Element;

use crate::SlidesError;

pub struct SlidesHtmlHandler {
    inner: SyntectHtmlHandler<IOError, DefaultHtmlHandler>,
    current_slide: u32,
    current_table_row: u32,
}

impl SlidesHtmlHandler {
    pub fn new() -> Self {
        SlidesHtmlHandler {
            inner: SyntectHtmlHandler::default(),
            current_slide: 0,
            current_table_row: 0,
        }
    }
}

impl HtmlHandler<SlidesError> for SlidesHtmlHandler {
    fn start<W: Write>(&mut self, mut w: W, element: &Element<'_>) -> Result<(), SlidesError> {
        match element {
            Element::Title(title) if title.level == 1 => {
                // Intro slide
                write!(w, r#"<div class="slide intro" id="intro">"#,)?;
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
            _ => {}
        }

        // fallthrough to default handler
        self.inner.start(w, element)?;
        Ok(())
    }

    fn end<W: Write>(&mut self, mut w: W, element: &Element<'_>) -> Result<(), SlidesError> {
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
