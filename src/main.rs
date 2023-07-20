mod api;
mod helpers;
mod model;
mod request;
mod utils;

use api::Api;
use model::{Model, ModelType};

use clap::Parser;
use rascii_art::{charsets, RenderOptions};
use spinoff::{spinners, Color, Spinner, Streams};
use std::{error::Error, io};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    ///Prompt to enter
    prompt: String,

    /// Use AI to generate ascii art, but with a negative prompt
    #[arg(short, long, default_value = "")]
    negative_prompt: Option<String>,

    /// Number of images to generate when using AI [1..9]
    #[arg(short = 'N', long, value_name = "NUMBER", default_value = "9")]
    num_image: usize,

    /// Model to use in generation
    #[arg(value_enum, short, long, default_value = "general")]
    model_type: Option<ModelType>,

    /// Model API version
    #[arg(value_enum, short, name = "API_VERSION", default_value = "3")]
    version: Option<Api>,

    /// API token for premium users (Faster generation, No watermark)
    #[arg(short, long, name = "TOKEN")]
    api_token: Option<String>,

    /// Width of the output image. Defaults to 128 if width and height are not specified
    #[arg(short, long)]
    width: Option<u32>,

    /// Height of the output image, if not specified, it will be calculated to keep the aspect ratio
    #[arg(short = 'H', long)]
    height: Option<u32>,

    /// Whether to use colors in the output image
    #[arg(name = "color", short, long)]
    colored: bool,

    /// Inverts the weights of the characters. Useful for white backgrounds
    #[arg(short, long)]
    invert: bool,

    /// Characters used to render the image, from transparent to opaque. Built-in charsets: block, emoji, default, russian, slight
    #[arg(short = 'C', long, default_value = "default")]
    charset: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut args = Args::parse();

    let clusters = UnicodeSegmentation::graphemes(args.charset.as_str(), true).collect::<Vec<_>>();
    let charset = charsets::from_str(args.charset.as_str()).unwrap_or(clusters.as_slice());

    if args.width.is_none() && args.height.is_none() {
        args.width = Some(80);
    }

    let prompt = args.prompt;

    let spinner = Spinner::new_with_stream(
        spinners::Arc,
        prompt.to_string(),
        Color::Green,
        Streams::Stderr,
    );

    let model = Model::from(args.model_type.unwrap(), args.version.unwrap())
        .api_token(args.api_token);

    let images = model
        .generate(&prompt, &args.negative_prompt.unwrap(), args.num_image)
        .await?;

    spinner.success("\x1b[32mDone!\x1b[0m"); // TODO replace it with non-magic string

    for image in images {
        rascii_art::render_image(
            &image,
            &mut io::stdout(),
            &RenderOptions {
                width: args.width,
                height: args.height,
                colored: args.colored,
                invert: args.invert,
                charset,
            },
        )?;
        println!("\n");
    }
    Ok(())
}
