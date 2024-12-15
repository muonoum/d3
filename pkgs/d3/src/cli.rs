use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
	#[arg(long, default_value_t = 500)]
	pub width: u32,

	#[arg(long, default_value_t = 500)]
	pub height: u32,

	#[arg(long, default_value_t = 2)]
	pub scale: u32,

	#[arg(long)]
	pub scene: String,

	#[arg(long, default_value = "phong2")]
	pub reflection: String,

	#[arg(long, default_value = "phong")]
	pub shading: String,
}
