#[cfg(test)]
mod test {
    use webmention::Client;

    // HTTP `Link` header, unquoted rel, relative URL
    #[tokio::test]
    async fn discovery_one() {
        let res = Client::builder()
            .source("https://webmention.rocks/test/1")
            .build()
            .unwrap()
            .send()
            .await
            .unwrap();

        assert_eq!(
            res.unwrap().as_str(),
            "https://webmention.rocks/test/1/webmention"
        );
    }

    // HTTP `Link` header, unquoted rel, absolute URL
    #[tokio::test]
    async fn discovery_two() {
        let res = Client::builder()
            .source("https://webmention.rocks/test/2")
            .build()
            .unwrap()
            .send()
            .await
            .unwrap();

        assert_eq!(
            res.unwrap().as_str(),
            "https://webmention.rocks/test/2/webmention"
        );
    }

    // HTML `<link>` tag, relative URL
    #[tokio::test]
    async fn discovery_three() {
        let res = Client::builder()
            .source("https://webmention.rocks/test/3")
            .build()
            .unwrap()
            .send()
            .await
            .unwrap();

        assert_eq!(
            res.unwrap().as_str(),
            "https://webmention.rocks/test/3/webmention"
        );
    }

    // HTML `<link>` tag, absolute URL
    #[tokio::test]
    async fn discovery_four() {
        let res = Client::builder()
            .source("https://webmention.rocks/test/4")
            .build()
            .unwrap()
            .send()
            .await
            .unwrap();

        assert_eq!(
            res.unwrap().as_str(),
            "https://webmention.rocks/test/4/webmention"
        );
    }

    // HTML `<a>` tag, relative URL
    #[tokio::test]
    async fn discovery_five() {
        let res = Client::builder()
            .source("https://webmention.rocks/test/5")
            .build()
            .unwrap()
            .send()
            .await
            .unwrap();

        assert_eq!(
            res.unwrap().as_str(),
            "https://webmention.rocks/test/5/webmention"
        );
    }

    // HTML `<a>`, absolute URL
    #[tokio::test]
    async fn discovery_six() {
        let res = Client::builder()
            .source("https://webmention.rocks/test/6")
            .build()
            .unwrap()
            .send()
            .await
            .unwrap();

        assert_eq!(
            res.unwrap().as_str(),
            "https://webmention.rocks/test/6/webmention"
        );
    }

    // HTML `Link` header with "strange casing"
    #[tokio::test]
    async fn discovery_seven() {
        let res = Client::builder()
            .source("https://webmention.rocks/test/7")
            .build()
            .unwrap()
            .send()
            .await
            .unwrap();

        assert_eq!(
            res.unwrap().as_str(),
            "https://webmention.rocks/test/7/webmention"
        );
    }

    // HTML `Link` header, quoted rel
    #[tokio::test]
    async fn discovery_eight() {
        let res = Client::builder()
            .source("https://webmention.rocks/test/8")
            .build()
            .unwrap()
            .send()
            .await
            .unwrap();

        assert_eq!(
            res.unwrap().as_str(),
            "https://webmention.rocks/test/8/webmention"
        );
    }

    // HTML `<link>` tag with multiple "rel" values
    #[tokio::test]
    async fn discovery_nine() {
        let res = Client::builder()
            .source("https://webmention.rocks/test/9")
            .build()
            .unwrap()
            .send()
            .await
            .unwrap();

        assert_eq!(
            res.unwrap().as_str(),
            "https://webmention.rocks/test/9/webmention"
        );
    }

    // HTTP `Link` header with multiple "rel" values
    #[tokio::test]
    async fn discovery_ten() {
        let res = Client::builder()
            .source("https://webmention.rocks/test/10")
            .build()
            .unwrap()
            .send()
            .await
            .unwrap();

        assert_eq!(
            res.unwrap().as_str(),
            "https://webmention.rocks/test/10/webmention"
        );
    }

    // Multiple Webmention endpoints: `Link` header, `<link>` and `<a>` tags
    #[tokio::test]
    async fn discovery_eleven() {
        let res = Client::builder()
            .source("https://webmention.rocks/test/11")
            .build()
            .unwrap()
            .send()
            .await
            .unwrap();

        assert_eq!(
            res.unwrap().as_str(),
            "https://webmention.rocks/test/11/webmention"
        );
    }

    // Checking for exact match of `rel=webmention`
    #[tokio::test]
    async fn discovery_twelve() {
        let res = Client::builder()
            .source("https://webmention.rocks/test/12")
            .build()
            .unwrap()
            .send()
            .await
            .unwrap();

        assert_eq!(
            res.unwrap().as_str(),
            "https://webmention.rocks/test/12/webmention"
        );
    }

    // False endpoint inside HTML comment
    #[tokio::test]
    async fn discovery_thirteen() {
        let res = Client::builder()
            .source("https://webmention.rocks/test/13")
            .build()
            .unwrap()
            .send()
            .await
            .unwrap();

        assert_eq!(
            res.unwrap().as_str(),
            "https://webmention.rocks/test/13/webmention"
        );
    }

    // False endpoint in escaped HTML
    #[tokio::test]
    async fn discovery_fourteen() {
        let res = Client::builder()
            .source("https://webmention.rocks/test/14")
            .build()
            .unwrap()
            .send()
            .await
            .unwrap();

        assert_eq!(
            res.unwrap().as_str(),
            "https://webmention.rocks/test/14/webmention"
        );
    }

    // Webmention `href` is an empty string
    #[tokio::test]
    async fn discovery_fifteen() {
        let res = Client::builder()
            .source("https://webmention.rocks/test/15")
            .build()
            .unwrap()
            .send()
            .await
            .unwrap();

        assert_eq!(res.unwrap().as_str(), "https://webmention.rocks/test/15");
    }

    // Multiple Webmention endpoints: `<a>` `<link>` tags
    #[tokio::test]
    async fn discovery_sixteen() {
        let res = Client::builder()
            .source("https://webmention.rocks/test/16")
            .build()
            .unwrap()
            .send()
            .await
            .unwrap();

        assert_eq!(
            res.unwrap().as_str(),
            "https://webmention.rocks/test/16/webmention"
        );
    }

    // Multiple Webmention endpoints: `<link>` `<a>` tags
    #[tokio::test]
    async fn discovery_seventeen() {
        let res = Client::builder()
            .source("https://webmention.rocks/test/17")
            .build()
            .unwrap()
            .send()
            .await
            .unwrap();

        assert_eq!(
            res.unwrap().as_str(),
            "https://webmention.rocks/test/17/webmention"
        );
    }

    // Multiple HTTP `Link` headers
    #[tokio::test]
    async fn discovery_eighteen() {
        let res = Client::builder()
            .source("https://webmention.rocks/test/18")
            .build()
            .unwrap()
            .send()
            .await
            .unwrap();

        assert_eq!(
            res.unwrap().as_str(),
            "https://webmention.rocks/test/18/webmention"
        );
    }

    // Single HTTP `Link` header, with multiple values
    #[tokio::test]
    async fn discovery_nineteen() {
        let res = Client::builder()
            .source("https://webmention.rocks/test/19")
            .build()
            .unwrap()
            .send()
            .await
            .unwrap();

        assert_eq!(
            res.unwrap().as_str(),
            "https://webmention.rocks/test/19/webmention"
        );
    }

    // HTML `<link>` tag with no `href` attribute
    #[tokio::test]
    async fn discovery_twenty() {
        let res = Client::builder()
            .source("https://webmention.rocks/test/20")
            .build()
            .unwrap()
            .send()
            .await
            .unwrap();

        assert_eq!(
            res.unwrap().as_str(),
            "https://webmention.rocks/test/20/webmention"
        );
    }

    // Webmention endpoint has query parameters
    #[tokio::test]
    async fn discovery_twentyone() {
        let res = Client::builder()
            .source("https://webmention.rocks/test/21")
            .build()
            .unwrap()
            .send()
            .await
            .unwrap();

        assert_eq!(
            res.unwrap().as_str(),
            "https://webmention.rocks/test/21/webmention?query=yes"
        );
    }

    // Webmention endpoint is relative to the initial request path
    #[tokio::test]
    async fn discovery_twentytwo() {
        let res = Client::builder()
            .source("https://webmention.rocks/test/22")
            .build()
            .unwrap()
            .send()
            .await
            .unwrap();

        assert_eq!(
            res.unwrap().as_str(),
            "https://webmention.rocks/test/22/webmention"
        );
    }

    // Webmention endpoint is a redirect, and is relative to the initial
    // request path
    #[tokio::test]
    async fn discovery_twentythree() {
        let res = Client::builder()
            .source("https://webmention.rocks/test/23")
            .build()
            .unwrap()
            .send()
            .await
            .unwrap();

        assert_eq!(
            res.unwrap().as_str(),
            "https://webmention.rocks/test/23/webmention"
        );
    }
}
