use man::prelude::*;
pub struct Helper {}

impl Helper {
pub fn get_help_text() -> String {
    let page = Manual::new("ascii-tool")
        .about("Little tool to convert asciidoc files to primary html")
        .author(Author::new("Mühlhans Florian").email("muehlhans.florian@web.de"))
        .flag(
            Flag::new()
                .short("help")
                .help("prints this text"),
        )
        .flag(
            Flag::new()
                .short("add")
                .help("adds a file to the file list"),
        )
        .option(
            Opt::new("output")
                .short("list")
                .help("prints all files, that are currently in the file list"),
        )
        .render();
    return page;
}}