// TODO: open questions
// * How can we add presentation notes?

// TODO: embed images
// TODO: allow skipping slide titles
// TODO: enable code highlighting

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use std::convert::From;
use std::io::{Error as IOError, Write};
use std::string::FromUtf8Error;

use orgize::export::{DefaultHtmlHandler, HtmlHandler};
use orgize::{Element, Org};

fn main() {
    // Get file from the arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Please pass in a file to render");
        return;
    }

    let file = File::open(&args[1]).expect("Failed to open the file");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).expect("Read file");

    let mut writer = Vec::new();
    let mut handler = MyHtmlHandler::new();
    let org = Org::parse(&contents);
    org.html_with_handler(&mut writer, &mut handler)
        .expect("Export html");

    let exported_html = String::from_utf8(writer).expect("Converted to utf8");

    let style = r#"
body {
  margin: 0;
  font-size: 2rem;
  font-weight: 300;
  line-height: 1.4;
  font-family: Calibri, "Open Sans", Helvetica, sans-serif;
}

.slide {
  border-bottom: 2px solid #ddd;
  height: 100vh;
}

.slide > section {
  padding: 1rem 2rem;
}

.slide:first-child {
  margin-top: 0;
}

.slide:last-child {
  border-bottom: 0;
  margin-bottom: 0;
}

#intro {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}

h1,
h2,
h3 {
  margin: 0;
}

h1,
h2,
h3 {
  padding: 2rem 2rem 0.5rem;
}

.secondary {
  color: #888;
}
"#;

    // Generate HTML
    println!(
        r#"
<!DOCTYPE html>
<html>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <head>
    <title>{}</title>
    <style>{}</style>
  </head>
  <body>"#,
        "Presentation", style
    );

    println!("{}", exported_html);

    let script = r#"
;(function () {
document.addEventListener('keyup', function (e) {
  var currentSlide = parseInt(location.hash.slice(7), 10) || 0;

  if (e.keyCode === 40 || e.keyCode === 39) {
    // Key down or key right
    e.preventDefault();

    var nextId = 'slide-' + (currentSlide + 1);
    if (document.getElementById(nextId)) {
      location.hash = nextId;
    }
  }

  if (e.keyCode === 38 || e.keyCode === 37) {
    // Key up or key left
    e.preventDefault();

    if (currentSlide > 1) {
      var previousId = 'slide-' + (currentSlide - 1);
      location.hash = previousId;
    } else {
      location.hash = 'intro';
    }
  }
});

})();
"#;

    println!(
        r#"
    <script>{}</script>
  </body>
</html>"#,
        script
    );
}

#[derive(Debug)]
enum MyError {
    IO(IOError),
    // Heading,
    Utf8(FromUtf8Error),
}

// From<std::io::Error> trait is required for custom error type
impl From<IOError> for MyError {
    fn from(err: IOError) -> Self {
        MyError::IO(err)
    }
}

impl From<FromUtf8Error> for MyError {
    fn from(err: FromUtf8Error) -> Self {
        MyError::Utf8(err)
    }
}

struct MyHtmlHandler {
    inner: DefaultHtmlHandler,
    current_slide: u32,
}

impl MyHtmlHandler {
    fn new() -> Self {
        Self {
            inner: DefaultHtmlHandler,
            current_slide: 0,
        }
    }
}

impl HtmlHandler<MyError> for MyHtmlHandler {
    fn start<W: Write>(&mut self, mut w: W, element: &Element<'_>) -> Result<(), MyError> {
        match element {
            Element::Title(title) if title.level == 1 => {
                // Intro slide
                write!(w, r#"<div class="slide" id="intro">"#,)?;
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
            _ => {}
        }

        // fallthrough to default handler
        self.inner.start(w, element)?;
        Ok(())
    }

    fn end<W: Write>(&mut self, mut w: W, element: &Element<'_>) -> Result<(), MyError> {
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
            _ => {}
        }

        self.inner.end(w, element)?;
        Ok(())
    }
}
