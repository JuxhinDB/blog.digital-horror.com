#[macro_use]
extern crate json;

use anyhow::{Context, Result};
use std::io::prelude::*;
use std::path::PathBuf;
use time::{format_description, Date};

use comrak::{markdown_to_html, ComrakExtensionOptions, ComrakOptions, ComrakRenderOptions};

#[derive(Debug)]
struct Post {
    title: String,
    date: String,
    description: String,
    content: String,
}

fn main() {
    let output_dir = PathBuf::from("build/");

    let metadata = load_template(&PathBuf::from(
        "/home/juxhin/dev/blog.digital-horror.com/blog/metadata.html",
    ))
    .context("Failed to load metadata")
    .unwrap();

    let header = load_template(&PathBuf::from(
        "/home/juxhin/dev/blog.digital-horror.com/blog/header.html",
    ))
    .context("Failed to load header")
    .unwrap();

    let footer: String = load_template(&PathBuf::from(
        "/home/juxhin/dev/blog.digital-horror.com/blog/footer.html",
    ))
    .context("Failed to load footer")
    .unwrap();

    let mut posts = load_posts(&PathBuf::from(
        "/home/juxhin/dev/blog.digital-horror.com/kafkaesque/posts",
    ))
    .context("Failed to load posts")
    .unwrap()
    .into_iter()
    .map(|post| {
        let description = post.description;

        let content = format!(
            r#"
<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01//EN" "http://www.w3.org/TR/html4/strict.dtd">
<html lang="en">
    <head>
        <title>Digital Horror</title>
        <meta name="description" content="{description}">
        {metadata}
    </head>
    <body>
    <div class="container content-container">
        {header}
        <main>
            <article class="post">
            <h2 class="post-title">{}</h2>
            <p class="post-date">{}</p>
                <div class = "post-content"> 
                {}
                </div>
            </article>
        </main>
    </div>

    {footer}

    <script src="/js/script.js" defer></script>
    <script src="/js/prism.js" defer></script>

    </body>
</html>
        "#,
            post.title, post.date, post.content
        );

        Post {
            title: post.title,
            date: post.date,
            description,
            content,
        }
    })
    .collect::<Vec<Post>>();

    // Sort posts by date using `OffsetDateTime` and `sort_by_key`
    posts.sort_by_key(|post| {
        let format = format_description::parse("[month]/[day]/[year]").unwrap();
        match Date::parse(post.date.as_str(), &format) {
            Ok(d) => d,
            Err(e) => {
                panic!("Failed to parse date: {} for post {}", e, post.title);
            }
        }
    });
    posts.reverse();

    // Output rendered posts into build directory
    for post in posts.iter() {
        let mut file = std::fs::File::create(output_dir.join(format!(
            "{}.html",
            post.title.to_lowercase().replace(' ', "-")
        )))
        .unwrap();
        file.write_all(post.content.as_bytes()).unwrap();
    }

    // Build posts.json file with the title, date and description of each post
    // using the `object!` macro from the `json` crate.
    let posts_json = object! {
        posts: posts.iter().map(|post| {
            object! {
                title: post.title.clone(),
                date: post.date.clone(),
                description: post.description.clone(),
                url: format!("posts/{}.html", post.title.to_lowercase().replace(' ', "-"))
            }
        }).collect::<Vec<_>>()
    };

    let mut posts_json_file = std::fs::File::create(output_dir.join("posts.json")).unwrap();
    posts_json_file
        .write_all(posts_json.dump().as_bytes())
        .unwrap();
}

fn load_posts(dir: &PathBuf) -> Result<Vec<Post>> {
    let mut posts = Vec::new();

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().unwrap() == "md" {
            let options = ComrakOptions {
                extension: ComrakExtensionOptions {
                    strikethrough: true,
                    table: true,
                    autolink: true,
                    tasklist: true,
                    superscript: true,
                    footnotes: true,
                    header_ids: Some("header-".to_string()),
                    front_matter_delimiter: Some(String::from("---")),
                    ..ComrakExtensionOptions::default()
                },
                render: ComrakRenderOptions {
                    // This is needed in order to retain certain raw HTML such as
                    // SVGs and embedded videos. Since we are containing the user
                    // input entirely ourselves, this is _probably_ safe.
                    unsafe_: true,
                    ..ComrakRenderOptions::default()
                },
                parse: ComrakOptions::default().parse,
            };

            let raw_content = load_template(&path)?;
            let content = markdown_to_html(raw_content.as_str(), &options);

            let post = Post {
                title: raw_content
                    .lines()
                    .nth(1)
                    .unwrap()
                    .to_string()
                    .split(": ")
                    .nth(1)
                    .unwrap()
                    .to_string(),
                date: raw_content
                    .lines()
                    .nth(2)
                    .unwrap()
                    .to_string()
                    .split(": ")
                    .nth(1)
                    .unwrap()
                    .to_string(),
                description: raw_content
                    .lines()
                    .nth(3)
                    .unwrap()
                    .to_string()
                    .split(": ")
                    .nth(1)
                    .unwrap()
                    .to_string(),
                content,
            };

            posts.push(post);
        }
    }

    Ok(posts)
}

fn load_template(filename: &PathBuf) -> Result<String> {
    let mut file = std::fs::File::open(filename)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    Ok(contents)
}
