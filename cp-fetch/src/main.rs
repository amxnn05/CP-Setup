use clap::Parser;
use scraper::{Html, Selector};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::collections::HashSet;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// URL of the problem or contest
    url: String,
}

fn fetch_html(url: &str) -> String {
    let output = Command::new("curl")
        .arg("-sL")
        .arg("-H")
        .arg("User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .arg(url)
        .output()
        .expect("Failed to execute curl command");
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn fetch_problem(url: &str, dir_path: &Path) {
    println!("Fetching problem from {}...", url);

    let html_content = fetch_html(url);

    if html_content.contains("Just a moment...") && html_content.contains("cloudflare") {
        eprintln!("Warning: Cloudflare blocked the request.");
    }

    let document = Html::parse_document(&html_content);
    
    let input_selector = Selector::parse("div.input pre").unwrap();
    let output_selector = Selector::parse("div.output pre").unwrap();
    let time_limit_sel = Selector::parse("div.time-limit").unwrap();
    let mem_limit_sel = Selector::parse("div.memory-limit").unwrap();

    let mut time_limit = String::from("unknown");
    let mut mem_limit = String::from("unknown");

    if let Some(tl_node) = document.select(&time_limit_sel).next() {
        let text: Vec<_> = tl_node.text().collect();
        if text.len() >= 2 {
            time_limit = text[1].trim().to_string();
        }
    }
    
    if let Some(ml_node) = document.select(&mem_limit_sel).next() {
        let text: Vec<_> = ml_node.text().collect();
        if text.len() >= 2 {
            mem_limit = text[1].trim().to_string();
        }
    }

    let mut inputs = Vec::new();
    for element in document.select(&input_selector) {
        let mut text = String::new();
        for node in element.descendants() {
            if let Some(text_node) = node.value().as_text() {
                text.push_str(text_node);
            } else if let Some(elem) = node.value().as_element() {
                if elem.name() == "br" || elem.name() == "div" {
                    text.push('\n');
                }
            }
        }
        inputs.push(text.replace("\n\n", "\n").trim().to_string());
    }

    let mut outputs = Vec::new();
    for element in document.select(&output_selector) {
        let mut text = String::new();
        for node in element.descendants() {
            if let Some(text_node) = node.value().as_text() {
                text.push_str(text_node);
            } else if let Some(elem) = node.value().as_element() {
                if elem.name() == "br" || elem.name() == "div" {
                    text.push('\n');
                }
            }
        }
        outputs.push(text.replace("\n\n", "\n").trim().to_string());
    }

    if inputs.is_empty() {
        println!("No test cases found for {}.", url);
        return;
    }

    let testcases_dir = dir_path.join("testcases");
    if testcases_dir.exists() {
        fs::remove_dir_all(&testcases_dir).unwrap();
    }
    fs::create_dir_all(&testcases_dir).unwrap();

    for (i, (input, output)) in inputs.iter().zip(outputs.iter()).enumerate() {
        let index = i + 1;
        let in_path = testcases_dir.join(format!("in{}.txt", index));
        let out_path = testcases_dir.join(format!("out{}.txt", index));
        
        let mut in_file = File::create(in_path).unwrap();
        in_file.write_all(input.as_bytes()).unwrap();
        in_file.write_all(b"\n").unwrap();
        
        let mut out_file = File::create(out_path).unwrap();
        out_file.write_all(output.as_bytes()).unwrap();
        out_file.write_all(b"\n").unwrap();
    }
    
    let limits_path = dir_path.join("limits.txt");
    let mut limits_file = File::create(limits_path).unwrap();
    limits_file.write_all(format!("TIME_LIMIT=\"{}\"\nMEM_LIMIT=\"{}\"\n", time_limit, mem_limit).as_bytes()).unwrap();

    println!("[ Successfully fetched {} test case(s) for {} ]", inputs.len(), dir_path.display());
}

fn copy_template_files(target_dir: &Path) {
    let files_to_copy = ["code.cpp", "template.cpp", "cprun.sh", "debug.h"];
    for file in files_to_copy.iter() {
        if Path::new(file).exists() {
            let target_path = target_dir.join(file);
            if let Err(e) = fs::copy(file, target_path) {
                eprintln!("Failed to copy {}: {}", file, e);
            }
        } else if *file == "code.cpp" && Path::new("template.cpp").exists() {
             let target_path = target_dir.join("code.cpp");
             if let Err(e) = fs::copy("template.cpp", target_path) {
                 eprintln!("Failed to copy template.cpp to code.cpp: {}", e);
             }
        }
    }
}

fn main() {
    let args = Args::parse();
    let url = args.url.trim_end_matches('/');

    if url.contains("/problem/") || url.contains("/problemset/problem/") {
        // Single problem
        fetch_problem(url, Path::new("."));
    } else {
        // Contest
        println!("Detecting contest URL: {}", url);
        let html_content = fetch_html(url);
        let document = Html::parse_document(&html_content);
        
        let mut problem_urls = Vec::new();
        let mut seen = HashSet::new();

        let a_selector = Selector::parse("a").unwrap();
        for element in document.select(&a_selector) {
            if let Some(href) = element.value().attr("href") {
                // Must start with /contest/{id}/problem/
                if href.contains("/problem/") && href.starts_with("/contest/") {
                    if !seen.contains(href) {
                        seen.insert(href.to_string());
                        problem_urls.push(href.to_string());
                    }
                }
            }
        }

        problem_urls.sort();

        if problem_urls.is_empty() {
            println!("No problems found on this contest page.");
            return;
        }

        // Extract contest ID
        let parts: Vec<&str> = url.split('/').collect();
        let contest_id = parts.last().unwrap_or(&"contest");
        let contest_dir = format!("Contest_{}", contest_id);

        fs::create_dir_all(&contest_dir).unwrap();
        println!("Created contest folder: {}", contest_dir);

        for href in problem_urls {
            let p_parts: Vec<&str> = href.split('/').collect();
            let problem_index = p_parts.last().unwrap();
            
            let problem_dir = Path::new(&contest_dir).join(problem_index);
            fs::create_dir_all(&problem_dir).unwrap();
            
            println!("\n[ Setting up Problem {} ]", problem_index);
            copy_template_files(&problem_dir);
            
            let full_url = if href.starts_with("http") {
                href.to_string()
            } else {
                format!("https://codeforces.com{}", href)
            };
            
            fetch_problem(&full_url, &problem_dir);
        }
        
        println!("\n[ Contest {} setup complete in folder '{}' ]", contest_id, contest_dir);
    }
}
