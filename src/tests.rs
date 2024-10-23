#[cfg(test)]
mod tests {
    use crate::{
        cli::Cli,
        filters::{parse_filter_list, parse_range_filter},
        fuzz::create_fuzzlist,
        input::parse_wordlist,
        parse_response_filters,
    };
    use clap::{CommandFactory, Parser};

    #[test]
    fn test_cli_parsing() {
        let args = vec![
            "furl",
            "http://example.com/FUZZ",
            "wordlist.txt",
            "--concurrency",
            "100",
            "--match-status",
            "200,301",
            "--timeout",
            "5",
            "--redirects",
            "5",
            "--delay",
            "2",
        ];
        let cli = Cli::parse_from(args);

        assert_eq!(cli.target, "http://example.com/FUZZ");
        assert_eq!(cli.wordlist, "wordlist.txt");
        assert_eq!(cli.concurrency, 100);
        assert_eq!(cli.match_status.as_deref(), Some("200,301"));
        assert_eq!(cli.timeout, 5);
        assert_eq!(cli.redirects, 5);
        assert_eq!(cli.delay, 2);
    }

    #[test]
    fn test_cli_defaults() {
        let args = vec!["furl", "http://example.com/FUZZ", "wordlist.txt"];
        let cli = Cli::parse_from(args);

        assert_eq!(cli.concurrency, 200); // default
        assert_eq!(cli.timeout, 4); // default
        assert_eq!(cli.redirects, 10); // default
        assert_eq!(cli.delay, 0); // default
    }

    #[test]
    fn test_cli_command_factory() {
        Cli::command().debug_assert();
    }

    #[test]
    fn test_parse_response_filters() {
        let filters = parse_response_filters(
            Some("200,301".to_string()),
            Some("100-200,300".to_string()),
            Some("10-20,50".to_string()),
            Some("400,500".to_string()),
            Some("50-100,200".to_string()),
            Some("5-10,30".to_string()),
        );

        assert_eq!(filters.status_matches.unwrap(), vec![200, 301]);
        assert_eq!(filters.size_matches.unwrap(), vec![(100, 200), (300, 300)]);
        assert_eq!(filters.line_matches.unwrap(), vec![(10, 20), (50, 50)]);
        assert_eq!(filters.status_filters.unwrap(), vec![400, 500]);
        assert_eq!(filters.size_filters.unwrap(), vec![(50, 100), (200, 200)]);
        assert_eq!(filters.line_filters.unwrap(), vec![(5, 10), (30, 30)]);
    }

    #[test]
    #[should_panic]
    fn test_invalid_match_status() {
        parse_response_filters(Some("99999".to_string()), None, None, None, None, None);
    }

    #[test]
    fn test_create_fuzzlist() {
        let target_url = "http://example.com/FUZZ";
        let wordlist = vec!["admin".to_string(), "login".to_string(), "test".to_string()];
        let fuzzlist = create_fuzzlist(target_url, wordlist).unwrap();

        assert_eq!(
            fuzzlist,
            vec![
                "http://example.com/admin".to_string(),
                "http://example.com/login".to_string(),
                "http://example.com/test".to_string()
            ]
        );
    }

    #[test]
    #[should_panic]
    fn test_invalid_fuzzlist() {
        let target_url = "http://example.com/";
        let wordlist = vec!["admin".to_string()];
        let _ = create_fuzzlist(target_url, wordlist).unwrap();
    }

    #[test]
    fn test_parse_filter_list() {
        assert_eq!(
            parse_filter_list(Some("200,301".to_string())),
            Some(vec![200, 301])
        );
        assert_eq!(parse_filter_list(None), None);
    }

    #[test]
    fn test_parse_range_filter() {
        assert_eq!(
            parse_range_filter(Some("100-200,300".to_string())),
            Some(vec![(100, 200), (300, 300)])
        );
        assert_eq!(
            parse_range_filter(Some("200".to_string())),
            Some(vec![(200, 200)])
        );
        assert_eq!(parse_range_filter(None), None);
    }
}
