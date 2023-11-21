extern crate ini;
use ini::Ini;
use std::path::Path;

fn main() {
	if (!Path::new("./config.ini").exists()) {
		let mut config = Ini::new();
		config.with_section(None::<String>).set("encoding", "utf-8");

		config
			.with_section(Some("General"))
			.set("depth", 3)
			.set("current", "OS_0");
		config
			.with_section(Some("OS_0"))
			.set("type", "windows")
			.set("partition", "sda1")
			.set("directory", "C:\\Users\\username\\Documents\\projects");
		config
			.with_section(Some("OS_1"))
			.set("type", "linux")
			.set("partition", "sda4")
			.set("directory", "/home/username/Documents/projects");

		config.write_to_file("config.ini").unwrap();
	}
}
