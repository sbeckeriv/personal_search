use std::time::Duration;
#[derive(Debug, Clone)]
pub enum GetterResults {
    Html(String),
    Text(String),
    Nothing,
}
pub trait IndexGetter {
    fn get_url(&self, url: &str) -> GetterResults {
        dbg!(&url);
        let agent = ureq::Agent::default().build();
        let res = agent
            .get(url)
            .set(
                "User-Agent",
                "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)",
            )
            .set("X-Source", "https://github.com/sbeckeriv/personal_search")
            .timeout(Duration::new(10, 0))
            .call();
        if res.status() < 300 {
            if let Some(lower) = res.header("Content-Type") {
                dbg!(&lower);
                let lower = lower.to_lowercase();
                if lower == "" || lower.contains("html") {
                    GetterResults::Html(res.into_string().unwrap_or_else(|_| "".to_string()))
                } else if lower.contains("text") && !lower.contains("javascript") {
                    GetterResults::Text(res.into_string().unwrap_or_else(|_| "".to_string()))
                } else if lower.contains("pdf") {
                    //GetterResults::Text(res.into_string().unwrap_or_else(|_| "".to_string()))
                    GetterResults::Nothing
                } else {
                    GetterResults::Nothing
                }
            } else {
                GetterResults::Nothing
            }
        } else {
            println!("{} status: {} {}", url, res.status_text(), res.status());
            GetterResults::Nothing
        }
    }
}
