use std::time::Duration;
#[derive(Debug, Clone)]
pub enum GetterResults {
    Html(String),
    Text(String),
    Nothing,
}
pub trait IndexGetter {
    fn get_url(&self, url: &str) -> GetterResults {
        let agent = ureq::agent();

        let res = agent
            .get(url)
            .set(
                "User-Agent",
                "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)",
            )
            .set("X-Source", "https://github.com/sbeckeriv/personal_search")
            .timeout(Duration::new(3, 0))
            .call();
        match res {
            Ok(res) => {
                if res.status() < 300 {
                    if let Some(lower) = res.header("Content-Type") {
                        let lower = lower.to_lowercase();
                        if lower == "" || lower.contains("html") {
                            GetterResults::Html(
                                res.into_string().unwrap_or_else(|_| "".to_string()),
                            )
                        } else if lower.contains("text") && !lower.contains("javascript") {
                            GetterResults::Text(
                                res.into_string().unwrap_or_else(|_| "".to_string()),
                            )
                        //                } else if lower.contains("pdf") {
                        //GetterResults::Text(res.into_string().unwrap_or_else(|_| "".to_string()))
                        //                   GetterResults::Nothing
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
            Err(error) => {
                println!("{:?} status: {} ", url, error);
                GetterResults::Nothing
            }
        }
    }
}
