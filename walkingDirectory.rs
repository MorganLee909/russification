fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with(".")) 
        .unwrap_or(false)
}

let walker = WalkDir::new("../InventoryManagement/").into_iter();
for entry in walker.filter_entry(|e| !is_hidden(e)) {
    let entry = entry.unwrap();
    let path_string: String = entry.path().display().to_string();
    
    if !path_string.contains("node_modules") && (path_string.contains(".js") || path_string.contains(".ejs")) {
        let mut contents = match read_file(&path_string[..]) {
            Ok(contents) => contents,
            Err(e) => panic!(e)
        };

        contents = contents.replace("$$", "₽$");
        contents = contents.replace("parseFloat(", "parseInt(");
        contents = contents.replace(".toFixed(2)", ".toFixed()");

        match write_file(&path_string, contents) {
            Err(e) => panic!(e),
            _ => ()
        };
    }
}