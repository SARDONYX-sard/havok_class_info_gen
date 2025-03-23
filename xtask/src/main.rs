use clap::{Parser, Subcommand};
use snafu::{ResultExt, Snafu};
use std::{fs, process::Command};

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Failed to execute command: {}", source))]
    CommandExecution { source: std::io::Error },
    #[snafu(display("Failed to write to file: {}", source))]
    FileWrite { source: std::io::Error },
}

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Build generator.
    Build,
    /// Generate Havok json.
    Generate(GenerateArgs),
    /// Run tests.
    Test,
    /// Run tests with nextest(need `cargo install nextest`).
    NTest,
}

#[derive(clap::Args)]
struct GenerateArgs {
    #[clap(long, value_enum, default_value_t = GenType::Normal)]
    mode: GenType,
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum GenType {
    /// For serde_hkx json
    Normal,
    /// For nemesis json(i8..=i64, u8..=u64, pointer -> accepts string)
    Nemesis,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Build) | None => {
            println!("Building...");
            run_command("cargo", &["build"], None)
        }
        Some(Commands::Generate(args)) => {
            const URL: &str= "https://github.com/SARDONYX-sard/havok_class_info_gen/releases/download/reflections/reflections.zip";
            let crate_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            let out_dir = crate_root.join("../crates/hkx_class_generator/assets");
            if !out_dir.join("hkxcmd_help").exists() {
                get_url(URL, out_dir);
            }

            match args.mode {
                GenType::Normal => run_command(
                    "cargo",
                    &[
                        "test",
                        "--package",
                        "hkx_class_generator",
                        "--lib",
                        "--",
                        "reflection_generator::tests::should_generate_x86_64_json",
                        "--exact",
                        "--show-output",
                        "--ignored",
                    ],
                    None,
                ),
                GenType::Nemesis => run_command(
                    "cargo",
                    &[
                        "test",
                        "--package",
                        "hkx_class_generator",
                        "--features",
                        "nemesis",
                        "--lib",
                        "--",
                        "reflection_generator::tests::should_generate_x86_64_json2",
                        "--exact",
                        "--show-output",
                        "--ignored",
                    ],
                    None,
                ),
            }
        }
        Some(Commands::Test) => run_command(
            "cargo",
            &["test", "--workspace"],
            Some("./test_results.txt"),
        ),
        Some(Commands::NTest) => run_command("cargo", &["nextest", "run", "--workspace"], None),
    }
}

fn run_command(cmd: &str, args: &[&str], output_file: Option<&str>) -> Result<()> {
    println!("Running: {} {:?}", cmd, args);
    let output = Command::new(cmd)
        .args(args)
        .output()
        .context(CommandExecutionSnafu)?;

    if let Some(output_file) = output_file {
        fs::write(output_file, &output.stdout).context(FileWriteSnafu)?;
        fs::write(output_file, &output.stderr).context(FileWriteSnafu)?;
    } else {
        std::io::Write::write_all(&mut std::io::stdout(), &output.stdout)
            .context(FileWriteSnafu)?;
        std::io::Write::write_all(&mut std::io::stderr(), &output.stderr)
            .context(FileWriteSnafu)?;
    }

    if !output.status.success() {
        eprintln!("Command failed: {} {:?}", cmd, args);
    }

    Ok(())
}

fn get_url<U, P>(url: U, out_dir: P)
where
    U: reqwest::IntoUrl,
    P: AsRef<std::path::Path>,
{
    use std::io::Cursor;

    let out_dir = out_dir.as_ref();

    // Download zip(Wait up to 30 minutes to download 160 MB considering the slow network.)
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(60 * 30))
        .build()
        .unwrap();
    let response = client.get(url).send().expect("Failed to download ZIP");
    let bytes = response.bytes().expect("Failed to read response bytes");

    zip_extract::extract(Cursor::new(bytes), out_dir, false).unwrap_or_else(|err| panic!("{err}"));
}
