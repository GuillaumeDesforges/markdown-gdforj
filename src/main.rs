fn main() {
    let cw_dir = std::env::current_dir().unwrap();

    // Walk source and collect all .md files
    // e.g. if a file is in ./foo/bar.md, we collect "foo/bar.md"
    let md_files = walkdir::WalkDir::new(&cw_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|dir_entry| dir_entry.path().extension().unwrap_or_default() == "md")
        .map(|dir_entry| dir_entry.path().strip_prefix(&cw_dir).unwrap().to_owned())
        .collect::<Vec<_>>();
    
    if md_files.is_empty() {
        println!("No .md files found in current directory tree");
        return;
    }

    // Create dist directory if it doesn't exist
    let dist_dir = cw_dir.join("dist").strip_prefix(&cw_dir).unwrap().to_owned();
    if !dist_dir.exists() {
        std::fs::create_dir(&dist_dir).unwrap();
    }
    println!("Converting {} files", md_files.len());
    
    // Process each .md file into .html in dist directory
    for md_file in md_files {
        let html_file = dist_dir.join(md_file.with_extension("html"));
        let md_content = std::fs::read_to_string(&md_file).unwrap();
        let html_content = markdown::to_html(md_content.as_str());
        std::fs::write(&html_file, html_content).unwrap();
        println!("{} => {}", md_file.display(), html_file.display());
    }
}
