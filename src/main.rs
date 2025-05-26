use clap::Parser;
use config::DataPath;
use std::path::PathBuf;

// mod button;
mod config;
mod log;
mod patcher_laa;
mod patcher_preload;
mod sq;
mod steamless;
// mod update;

#[derive(Parser, Debug)]
struct Cli {
	#[clap(subcommand)]
	operation: Operation,
}

#[derive(clap::Subcommand, Debug)]
enum Operation {
	Laa {
		#[clap(long)]
		exe_path: Option<PathBuf>,
	},
	Pre {
		#[clap(long)]
		data_path: Option<PathBuf>,
	},
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
	once_cell::sync::Lazy::force(&log::TRACING);
	tracing::info!("Starting MSU Launcher");

	let cli_args = Cli::parse();
	let config = init_config();

	match cli_args.operation {
		Operation::Laa { exe_path } => {
			if let Some(path_buf) = exe_path {
				match patcher_laa::patch_exe(&path_buf) {
					Ok(msg) => {
						tracing::info!("LAA Patching successful via CLI path: {}", msg);
						println!("LAA Patching successful: {}", msg);
					}
					Err(e) => {
						tracing::error!("LAA Patching failed via CLI path: {}", e);
						eprintln!("LAA Patching failed: {}", e);
						return Err(e.into());
					}
				}
			} else {
				// Use the path from config
				match patcher_laa::patch_from_config(&config) {
					Ok(_) => {
						// patch_from_config already traces its success message
						println!("LAA Patching from config successful.");
					}
					Err(e) => {
						// patch_from_config already traces its error message
						eprintln!("LAA Patching from config failed: {}", e);
						return Err(e.into());
					}
				}
			}
		}
		Operation::Pre { data_path } => {
			if let Some(path_buf) = data_path {
				patcher_preload::patch(Some(DataPath::new(path_buf)))
			} else {
				patcher_preload::patch_from_config(&config)
			};
		}
	}

	Ok(())
}

pub fn init_config() -> config::Config {
	config::Config::load_or_default()
}
