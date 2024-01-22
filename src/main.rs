fn find_images(node: &markdown::mdast::Node) -> Vec<&markdown::mdast::Image> {
    match node {
        markdown::mdast::Node::Image(image) => vec![image],
        node => {
            let children = node.children();
            match children {
                Some(children) => children.iter().map(find_images).flatten().collect(),
                None => vec![],
            }
        }
    }
}

fn main() {
    let cw_dir = std::env::current_dir().unwrap();

    let md_opts = markdown::Options {
        parse: markdown::ParseOptions {
            constructs: markdown::Constructs {
                frontmatter: true,
                ..markdown::ParseOptions::default().constructs
            },
            ..markdown::ParseOptions::gfm()
        },
        compile: markdown::CompileOptions {
            allow_dangerous_html: true,
            ..markdown::CompileOptions::default()
        },
        ..markdown::Options::gfm()
    };

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

        // Process and write HTML
        let html_content = markdown::to_html_with_options(md_content.as_str(), &md_opts).unwrap();
        if !html_file.parent().unwrap().exists() {
            std::fs::create_dir_all(html_file.parent().unwrap()).unwrap();
        }
        std::fs::write(&html_file, html_content).unwrap();
        println!("{} => {}", md_file.display(), html_file.display());

        // Copy as well images that are referenced in the .md file
        let md_ast = markdown::to_mdast(md_content.as_str(), &md_opts.parse).unwrap();
        let image_files = find_images(&md_ast)   ;
        for image_file in image_files {
            let image_file = md_file.parent().unwrap().join(image_file.url.to_owned());
            if !image_file.exists() {
                println!("WARNING: {} referenced but not found", image_file.display());
                continue;
            }
            let image_file_dist = dist_dir.join(image_file.to_owned());
            if !image_file_dist.parent().unwrap().exists() {
                std::fs::create_dir_all(image_file_dist.parent().unwrap()).unwrap();
            }
            std::fs::copy(&image_file, &image_file_dist).unwrap();
            println!("{} => {}", image_file.display(), image_file_dist.display());
        }
    }
}
