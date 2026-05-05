use clap::ValueEnum;

/// A pair of (image URL, optional thumbnail URL).
pub type LinkPair = (String, Option<String>);

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum Format {
    Plain,
    Bbcode,
    Html,
    Markdown,
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            self.to_possible_value()
                .expect("all variants have values")
                .get_name(),
        )
    }
}

/// Join link pairs into a space-separated string in the given format.
pub fn format_links(links: &[LinkPair], fmt: Format) -> String {
    links
        .iter()
        .map(|(img, thumb)| format_link(img, thumb.as_deref(), fmt))
        .collect::<Vec<_>>()
        .join(" ")
}

fn format_link(img: &str, thumb: Option<&str>, fmt: Format) -> String {
    match (fmt, thumb) {
        (Format::Plain, _) => img.to_owned(),
        (Format::Bbcode, None) => format!("[img]{img}[/img]"),
        (Format::Bbcode, Some(t)) => format!("[url={img}][img]{t}[/img][/url]"),
        (Format::Html, None) => format!("<img src=\"{img}\" alt=\"image\">"),
        (Format::Html, Some(t)) => format!("<a href=\"{img}\"><img src=\"{t}\" alt=\"thumb\"></a>"),
        (Format::Markdown, None) => format!("![image]({img})"),
        (Format::Markdown, Some(t)) => format!("[![thumb]({t})]({img})"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_links() -> Vec<LinkPair> {
        vec![("https://example.com/img1.png".to_owned(), None)]
    }

    fn sample_links_with_thumb() -> Vec<LinkPair> {
        vec![(
            "https://example.com/img1.png".to_owned(),
            Some("https://example.com/thumb1.jpg".to_owned()),
        )]
    }

    #[test]
    fn test_plain() {
        assert_eq!(
            format_links(&sample_links(), Format::Plain),
            "https://example.com/img1.png"
        );
    }

    #[test]
    fn test_plain_multiple() {
        let links = vec![
            ("https://example.com/a.png".to_owned(), None),
            ("https://example.com/b.png".to_owned(), None),
        ];
        assert_eq!(
            format_links(&links, Format::Plain),
            "https://example.com/a.png https://example.com/b.png"
        );
    }

    #[test]
    fn test_bbcode_no_thumb() {
        assert_eq!(
            format_links(&sample_links(), Format::Bbcode),
            "[img]https://example.com/img1.png[/img]"
        );
    }

    #[test]
    fn test_bbcode_with_thumb() {
        assert_eq!(
            format_links(&sample_links_with_thumb(), Format::Bbcode),
            "[url=https://example.com/img1.png][img]https://example.com/thumb1.jpg[/img][/url]"
        );
    }

    #[test]
    fn test_html_no_thumb() {
        assert_eq!(
            format_links(&sample_links(), Format::Html),
            "<img src=\"https://example.com/img1.png\" alt=\"image\">"
        );
    }

    #[test]
    fn test_html_with_thumb() {
        assert_eq!(
            format_links(&sample_links_with_thumb(), Format::Html),
            "<a href=\"https://example.com/img1.png\"><img src=\"https://example.com/thumb1.jpg\" alt=\"thumb\"></a>"
        );
    }

    #[test]
    fn test_markdown_no_thumb() {
        assert_eq!(
            format_links(&sample_links(), Format::Markdown),
            "![image](https://example.com/img1.png)"
        );
    }

    #[test]
    fn test_markdown_with_thumb() {
        assert_eq!(
            format_links(&sample_links_with_thumb(), Format::Markdown),
            "[![thumb](https://example.com/thumb1.jpg)](https://example.com/img1.png)"
        );
    }

    #[test]
    fn test_empty_links() {
        let links: Vec<LinkPair> = vec![];
        assert_eq!(format_links(&links, Format::Plain), "");
    }
}
