use anyhow::{Context, Result};
use std::io::prelude::*;
use std::path::PathBuf;

use comrak::{markdown_to_html, ComrakExtensionOptions, ComrakOptions, ComrakRenderOptions};

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

    let posts = load_posts(&PathBuf::from(
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

    for post in posts.iter() {
        let mut file = std::fs::File::create(output_dir.join(format!(
            "{}.html",
            post.title.to_lowercase().replace(" ", "-")
        )))
        .unwrap();
        file.write_all(post.content.as_bytes()).unwrap();
    }
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
                    header_ids: Some(String::from("header-".to_string())),
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
