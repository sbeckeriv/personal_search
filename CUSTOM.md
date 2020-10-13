# Custom indexing

Do you want private data? You want to process and index videos?
You can have it. Create a new syncer that generates urls:

```
# create your list of urls

pub struct AuthGetter {
    fn new(setup)->Self{
    }
}
impl IndexGetter for AuthGetter {

    fn get_url(&self, url: &str) -> GetterResults {
        let agent = ureq::Agent::default().build();
        // add auth
        let res = agent.get(url).timeout(Duration::new(10, 0)).call();

        if let Some(lower) = res.header("Content-Type") {
            dbg!(&lower);
            let lower = lower.to_lowercase();
            if lower == ""
                || lower.contains("html")
                || (lower.contains("text") && !lower.contains("javascript"))
            {
                GetterResults::Html(res.into_string().unwrap_or("".to_string()))
            } else {
                GetterResults::Nothing
            }
        } else {
            GetterResults::Nothing
        }
    }
}

indexer::index_url(
url.to_string(),
indexer::UrlMeta::default(),
None,
AuthGetter::new(auth),
)
```
