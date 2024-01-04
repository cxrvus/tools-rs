use std::{fs, io};

pub fn main() -> String {
	match get_file_info() {
		Ok(info_entries)  => {
			match info_entries.len() {
				0 => { "no files found".to_string() }
				_ => { fmt_info_entries(&info_entries) }
			}
		}
		Err(e) => e.to_string()
	} 
}

struct FileInfo {
	name: String,
	lines: u32
}

impl FileInfo {
	fn fmt(&self) -> String {
		format!("{: >20}: {: >6} lines", self.name, self.lines)
	}
}

fn fmt_info_entries(info_entries: &Vec<FileInfo>) -> String {
	let mut acc = String::new();
	let mut sum: u32 = 0;

	for info in info_entries {
		acc += &format!("{}\n", info.fmt());
		sum += info.lines;
	}

	let sum_info = FileInfo { name: "SUM".to_string(), lines: sum };
	acc += &format!("\n{}", sum_info.fmt());
	acc
}

fn get_file_info() -> io::Result<Vec<FileInfo>> {
	const EXTENSIONS: [&str; 1] = ["ts"];

	let path_arg = std::env::args().nth(1).unwrap_or(".".to_string());
	let entries = fs::read_dir(path_arg)?;
	
	let mut info: Vec<FileInfo> = vec![];
	
	for entry in entries {
		let entry = entry?;
		let path = entry.path();
		
		if let Some(extension) = path.extension() { 
			if !EXTENSIONS.contains(&extension.to_str().unwrap()) { continue }

			let name = entry.file_name().into_string().unwrap();
			let content = fs::read_to_string(path)?;
			let lines = content.lines().count() as u32;
		
			info.push(FileInfo { name, lines })
		};
	}

	Ok(info)
}
