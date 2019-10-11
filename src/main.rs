// TODO: open questions
// * How can we add presentation notes?

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use orgize::Org;

mod error;
mod handler;

use error::SlidesError;
use handler::SlidesHtmlHandler;

const STYLES: &'static str = include_str!("../assets/styles.css");
const SCRIPT: &'static str = include_str!("../assets/script.js");

fn main() -> Result<(), SlidesError> {
    // Get file from the arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return Err(SlidesError::Args("Please pass a file to render"));
    }

    let file = File::open(&args[1])?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let mut writer = Vec::new();
    let mut handler = SlidesHtmlHandler::default();
    let org = Org::parse(&contents);
    org.html_with_handler(&mut writer, &mut handler)?;

    let exported_html = String::from_utf8(writer)?;

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
        "Presentation", STYLES
    );

    println!("{}", exported_html);

    println!(
        r#"
    <script>{}</script>
  </body>
</html>"#,
        SCRIPT
    );

    Ok(())
}
