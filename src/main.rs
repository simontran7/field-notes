mod dev;
mod fmt;
mod lint;
mod ssg;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("fmt") => fmt::run(args.iter().any(|a| a == "--check")),
        Some("lint") => lint::run(),
        Some("serve") => dev::run(),
        _ => {
            let stats = ssg::Site::new(false).build()?;
            println!("built {} notes -> {}", stats.notes, ssg::DIST_DIR);
            Ok(())
        }
    }
}
