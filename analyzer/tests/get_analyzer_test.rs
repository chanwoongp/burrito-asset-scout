use analyzer::{get_analyzer, AnalyzerType};

#[test]
fn test_get_analyzer() {
    let analyzers_to_test = vec![
        (AnalyzerType::CoinTransfer, "ethereum"),
        (AnalyzerType::CoinTransfer, "solana"),
        (AnalyzerType::TokenTransfer, "ethereum"),
        (AnalyzerType::TokenTransfer, "solana"),
    ];

    for (analyzer_type, chain) in analyzers_to_test {
        let analyzer = get_analyzer(analyzer_type, chain);
        assert!(analyzer.is_some());
        analyzer.unwrap().analyze();
    }
}