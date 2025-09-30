use analyzer::get_analyzer;

#[test]
fn test_get_analyzer() {
    let analyzers_to_test = vec![
        ("native_coin_transfer", "ethereum"),
        ("native_coin_transfer", "solana"),
        ("token_transfer", "ethereum"),
        ("token_transfer", "solana"),
    ];

    for (analyzer_type, chain) in analyzers_to_test {
        let analyzer = get_analyzer(analyzer_type, chain);
        assert!(analyzer.is_some());
        analyzer.unwrap().analyze();
    }
}
