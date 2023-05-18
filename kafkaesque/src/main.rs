use anyhow::{Context, Result};
use std::io::prelude::*;
use std::path::PathBuf;

use comrak::{markdown_to_html, ComrakExtensionOptions, ComrakOptions};

struct Post {
    title: String,
    date: String,
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
        let content = format!(
            r#"
<html lang="en">
    <head>
    <title>Digital Horror</title>
    <div id="metadata-container">
    {metadata}
    </div>
    <link rel="import" href="/metadata.html" id="metadata-container">
    <script>
        // Import metadata content
        const metadataContainer = document.getElementById('metadata-container');
        document.head.innerHTML += metadataContainer.innerHTML;
    </script>
    </head>
    <body>
    <div class="container content-container">
        <div id="header-container">
        {header}
        </div>
        <main>
            <article class="post">
            <h2 class="post-title">{}</h2>
            <p class="post-date">{}</p>
            {}
            </article>
        </main>
    </div>

    <div id="footer-container">
    {footer}
    </div>
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
                ..ComrakOptions::default()
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