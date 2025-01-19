#[derive(clap::Parser, Debug, Clone)]
pub struct Args {
	#[arg(long, default_value_t = 500)]
	pub width: u32,

	#[arg(long, default_value_t = 500)]
	pub height: u32,

	#[arg(long, default_value_t = 2)]
	pub scale: u32,

	#[arg(long, default_value_t = false)]
	pub fullscreen: bool,

	#[arg(long)]
	pub scene: String,

	#[arg(long, default_value_t = false)]
	pub debug: bool,

	#[arg(long, default_value_t = false)]
	pub camera_light: bool,

	#[arg(long, default_value_t = 8)]
	pub threads: usize,

	#[arg(long, default_value_t = false)]
	pub untiled: bool,
}
