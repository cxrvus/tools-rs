use std::{env, fs, io};

pub fn main() -> String {
	let file_info = get_file_info();
	match file_info {
		Ok(info_entries) => {
			match info_entries.len() {
				0 => { "no files found".to_string() }
				_ => {
					info_entries.iter()
					.map(|x| { x.fmt() })
					.collect::<Vec<_>>()
					.join("\n")
				}
			}
		}
		Err(e) => format!("<!> {}", e.to_string())
	} 
}

struct FileInfo {
	name: String,
	line_count: u32
}

impl FileInfo {
	fn fmt(&self) -> String {
		format!("{: >20}: {: >6} lines", self.name, self.line_count)
	}
}

fn get_file_info() -> io::Result<Vec<FileInfo>> {
	const EXTENSIONS: [&str; 1] = ["ts"];

	let path_arg = env::args().nth(1).unwrap_or(".".to_string());
	let entries = fs::read_dir(path_arg)?;
	let mut info_entries: Vec<FileInfo> = vec![];
	let mut sum = 0 as u32;
	
	for entry in entries {
		let entry = entry?;
		let path = entry.path();
		
		if let Some(extension) = path.extension() { 
			let extension_str = extension.to_str().unwrap_or_default();
			if !EXTENSIONS.contains(&extension_str) { continue }

			let name = entry.file_name().into_string().unwrap_or("[no name]".to_string());
			let content = fs::read_to_string(path)?;
			let lines = content.lines().filter(|x| x.len() > 0);
			let line_count = lines.count() as u32;
		
			sum += line_count;
			info_entries.push(FileInfo { name, line_count })
		};
	}

	if info_entries.len() > 0 {
		let sum_entry = FileInfo { name: "SUM".to_string(), line_count: sum };
		info_entries.push(sum_entry);
	}

	Ok(info_entries)
}
