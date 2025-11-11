// This is free and unencumbered software released into the public domain.

#[cfg(not(feature = "std"))]
compile_error!("asimov-llamacpp-prompter requires the 'std' feature");

use asimov_module::SysexitsError::{self, *};
use asimov_module::tracing;
use clap::Parser;
use clientele::StandardOptions;
use std::{error::Error, io::Read};

/// asimov-llamacpp-prompter
#[derive(Debug, Parser)]
struct Options {
    #[clap(flatten)]
    flags: StandardOptions,

    #[clap(long, short)]
    model: Option<String>,

    /// Path to a GBNF grammar file (used to constrain decoding)
    #[clap(long, short)]
    grammar: Option<String>,

    /// Input file (defaults to STDIN)
    #[clap(long, short)]
    input: Option<String>,

    /// Output file (defaults to STDOUT)
    #[clap(long, short)]
    output: Option<String>,
}

pub fn main() -> Result<SysexitsError, Box<dyn Error>> {
    // Load environment variables from `.env`:
    asimov_module::dotenv().ok();

    // Expand wildcards and @argfiles:
    let args = asimov_module::args_os()?;

    // Parse command-line options:
    let options = Options::parse_from(args);

    // Handle the `--version` flag:
    if options.flags.version {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(EX_OK);
    }

    // Handle the `--license` flag:
    if options.flags.license {
        print!("{}", include_str!("../../UNLICENSE"));
        return Ok(EX_OK);
    }

    let Ok(manifest) = asimov_module::ModuleManifest::read_manifest("llamacpp")
        .inspect_err(|e| eprintln!("failed to read module manifest: {e}"))
    else {
        return Ok(EX_CONFIG);
    };

    let Ok(endpoint) = manifest
        .variable("endpoint", None)
        .inspect_err(|e| eprintln!("failed to read configured endpoint: {e}"))
    else {
        return Ok(EX_CONFIG); // not configured
    };

    let Some(model) = options.model.or_else(|| {
        manifest
            .variable("model", None)
            .inspect_err(|e| eprintln!("failed to read configured model: {e}"))
            .ok()
    }) else {
        return Ok(EX_CONFIG); // not configured
    };

    #[cfg(feature = "tracing")]
    asimov_module::init_tracing_subscriber(&options.flags).expect("failed to initialize logging");

    let input = if let Some(input) = options.input {
        std::fs::read_to_string(&input)
            .inspect_err(|e| tracing::error!("unable to read input file: {e}"))?
    } else {
        let mut buf = String::new();
        std::io::stdin()
            .read_to_string(&mut buf)
            .inspect_err(|e| tracing::error!("unable to read STDIN: {e}"))?;
        buf.trim().to_string()
    };

    let mut output: Box<dyn std::io::Write> = if let Some(output) = options.output {
        let out = std::fs::File::create(&output)
            .inspect_err(|e| tracing::error!("unable to open output file: {e}"))?;
        Box::new(out)
    } else {
        Box::new(std::io::stdout().lock())
    };

    let grammar: Option<String> = if let Some(path) = options.grammar {
        match std::fs::read_to_string(&path) {
            Ok(s) => Some(s),
            Err(e) => {
                tracing::error!("unable to read grammar file '{}': {e}", path);
                return Ok(EX_NOINPUT);
            },
        }
    } else {
        None
    };

    let options = asimov_llamacpp_module::Options {
        endpoint,
        model,
        grammar,
    };

    let response = asimov_llamacpp_module::generate(&input, &options)?;

    for text in response {
        output.write_all(text.as_bytes()).unwrap();
    }

    Ok(EX_OK)
}
