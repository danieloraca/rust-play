use url::Url;
use serde::{Deserialize, Deserializer};

trait ParseImageLink {
    fn unbox_image_links(&self) -> &str;
    fn split_image_links(&self) -> Vec<&str>;
    fn remove_single_quotes(&self) -> &str;
    fn parse_image_link(&self) -> Result<Option<Url>, &'static str>;
}

impl ParseImageLink for &str {
    fn unbox_image_links(&self) -> &str {
        self.trim_start_matches("[")
            .trim_end_matches("]")
    }

    fn split_image_links(&self) -> Vec<&str> {
        self.split(", ")
            .collect()
    }

    fn remove_single_quotes(&self) -> &str {
        self.trim_start_matches("'")
            .trim_end_matches("'")
    }

    fn parse_image_link(&self) -> Result<Option<Url>, &'static str> {
        match self {
            &"No Images" => Ok(None),
            _ => Url::parse(self)
                .map(Some)
                .map_err(|_| "Invalid URL"),
        }
    }
}

pub fn parse_image_links(link_str: &str) -> Result<Option<Vec<Url>>, &'static str> {
    link_str
        .unbox_image_links()
        .split_image_links()
        .into_iter()
        .map(|link| link.remove_single_quotes().parse_image_link())
        .collect::<Result<Option<Vec<Url>>, _>>()
}

pub fn deserialize_image_links<'de, D>(deserializer: D) -> Result<Option<Vec<Url>>, D::Error>
where
    D: Deserializer<'de>,
{
    let link_str: &str = Deserialize::deserialize(deserializer)?;
    parse_image_links(link_str).map_err(serde::de::Error::custom)
}

pub fn serialize_image_links<S>(
    image_links: &Option<Vec<Url>>, 
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(
        &serde_json::to_string(image_links)
            .map_err(|_| serde::ser::Error::custom("Invalid URL"))?,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unbox_image_links() {
        let image_links: &str = "[https://www.example.com/image1.jpg, https://www.example.com/image2.jpg]";
        let unboxed = image_links.unbox_image_links();
        assert_eq!(unboxed, "https://www.example.com/image1.jpg, https://www.example.com/image2.jpg");
    }

    #[test]
    fn test_split_image_links() {
        let image_links: &str = "'https://www.example.com/image1.jpg', 'https://www.example.com/image2.jpg'";
        let split = image_links.split_image_links();
        assert_eq!(split, vec!["'https://www.example.com/image1.jpg'", "'https://www.example.com/image2.jpg'"]);
    }

    #[test]
    fn test_remove_single_quotes() {
        let image_links: &str = "'https://www.example.com/image1.jpg'";
        let removed = image_links.remove_single_quotes();
        assert_eq!(removed, "https://www.example.com/image1.jpg");
    }

    #[test]
    fn test_parse_image_link() {
        let image_link: &str = "https://www.example.com/image1.jpg";
        let parsed = image_link.parse_image_link();
        assert_eq!(parsed, Ok(Some(Url::parse(image_link).unwrap())));
    }

    #[test]
    fn test_parse_image_links() {
        let image_links: &str = "['https://www.example.com/image1.jpg', 'https://www.example.com/image2.jpg']";
        let parsed = parse_image_links(image_links);
        assert_eq!(
            parsed, 
            Ok(Some(vec![
                 Url::parse("https://www.example.com/image1.jpg").unwrap(), 
                 Url::parse("https://www.example.com/image2.jpg").unwrap()
            ]))
        );
    }
   
    #[test]
    fn test_parse_image_links_invalid_url() {
        let image_links: &str = "['No Images']";
        let parsed = parse_image_links(image_links);
        assert_eq!(
            parsed, 
            Ok(None)
        );
    }

}

