use crate::OpenlibraryRequest;
use derive_builder::Builder;

/// The struct representation to the [Subjects API](https://openlibrary.org/dev/docs/api/subjects)
///
/// The fields for this struct are private. To view the available fields, see the [`SubjectsBuilder`](./struct.SubjectsBuilder.html) struct.
#[derive(Builder, Default, Debug)]
#[builder(setter(into), default)]
pub struct Subjects {
    subject: String,
    details: bool,
    ebooks: bool,
    published_in: String,
    #[builder(default = "10")]
    limit: u32,
    #[builder(default = "0")]
    offset: u32,
}

impl OpenlibraryRequest for Subjects {
    fn path(&self) -> String {
        format!("/subjects/{}.json", self.subject)
    }
    fn query(&self) -> Vec<(&'static str, String)> {
        vec![
            ("details", self.details.to_string()),
            ("ebooks", self.ebooks.to_string()),
            ("published_in", self.published_in.clone()),
            ("limit", self.limit.to_string()),
            ("offset", self.offset.to_string()),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::SubjectsBuilder;
    use crate::OpenlibraryRequest;
    use mockito::{mock, Matcher};
    use serde_json::json;

    #[test]
    fn test_subjects_execute() {
        let subjects = SubjectsBuilder::default()
            .subject("love")
            .published_in("1500-1600")
            .build()
            .unwrap();

        let json = json!({
            "key":  "/subjects/love",
            "name": "love",
            "work_count": 62
        });

        let _m = mock(
            "GET",
            Matcher::Regex(format!(r"{}\w*", subjects.url().path())),
        )
        .with_header("content-type", "application/json")
        .with_body(json.to_string())
        .create();

        let res = subjects.execute();
        assert_eq!(res["key"], "/subjects/love");
        assert_eq!(res["name"], "love");
        assert_eq!(res["work_count"], 62);
    }
}
