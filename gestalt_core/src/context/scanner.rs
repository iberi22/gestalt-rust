use ignore::WalkBuilder;
use std::fs;
use std::path::Path;
use super::FileContext;

pub fn scan_markdown_files(root: &Path) -> Vec<FileContext> {
    let mut files = Vec::new();
    let walker = WalkBuilder::new(root)
        .hidden(false) // Allow hidden files like .github
        .git_ignore(true)
        .build();

    for result in walker {
        match result {
            Ok(entry) => {
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                    // Skip target/ and node_modules/ just in case .gitignore misses them
                    if path.to_string_lossy().contains("target") || path.to_string_lossy().contains("node_modules") {
                        continue;
                    }

                    if let Ok(content) = fs::read_to_string(path) {
                        let lines: Vec<&str> = content.lines().collect();
                        let truncated_content = if lines.len() > 100 {
                            let mut c = lines.into_iter().take(100).collect::<Vec<_>>().join("\n");
                            c.push_str("\n... (truncated after 100 lines)");
                            c
                        } else {
                            content
                        };

                        files.push(FileContext {
                            path: path.display().to_string(),
                            content: truncated_content,
                        });
                    }
                }
            }
            Err(err) => eprintln!("Error walking directory: {}", err),
        }
    }
    files
}

pub fn generate_directory_tree(root: &Path, max_depth: usize) -> String {
    let mut tree = String::new();
    let walker = WalkBuilder::new(root)
        .hidden(false)
        .git_ignore(true)
        .max_depth(Some(max_depth))
        .build();

    for result in walker {
        if let Ok(entry) = result {
            let depth = entry.depth();
            if depth == 0 { continue; }

            let indent = "  ".repeat(depth - 1);
            let file_name = entry.file_name().to_string_lossy();
            let marker = if entry.file_type().map_or(false, |ft| ft.is_dir()) { "/" } else { "" };

            tree.push_str(&format!("{}├── {}{}\n", indent, file_name, marker));
        }
    }
    tree
}
