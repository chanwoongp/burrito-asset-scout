use provider::Chain;
use analyzer::{get_analyzer, AnalyzerType};
use provider::types::{AnyBlockData, AnyBlockHeader, BlockData as GenericBlockData};
use provider::{ethereum, solana};

#[test]
fn test_get_analyzer() {
    let analyzers_to_test = vec![
        (AnalyzerType::CoinTransfer, Chain::Ethereum),
        (AnalyzerType::CoinTransfer, Chain::Solana),
        (AnalyzerType::TokenTransfer, Chain::Ethereum),
        (AnalyzerType::TokenTransfer, Chain::Solana),
    ];

    for (analyzer_type, chain) in analyzers_to_test {
        let analyzer = get_analyzer(analyzer_type, chain);
        assert!(analyzer.is_some());
        let analyzer = analyzer.unwrap();
        match chain {
            Chain::Ethereum => {
                let header = AnyBlockHeader::Ethereum(ethereum::BlockHeader {
                    hash: "0xdeadbeef".to_string(),
                    parent_hash: "0xparent".to_string(),
                    number: 1,
                });
                let data = AnyBlockData::Ethereum(GenericBlockData::<ethereum::BlockData> {
                    block_header: ethereum::BlockHeader {
                        hash: "0xdeadbeef".to_string(),
                        parent_hash: "0xparent".to_string(),
                        number: 1,
                    },
                    transactions: vec![],
                });
                analyzer.analyze(&header, &data);
            }
            Chain::Solana => {
                let header = AnyBlockHeader::Solana(solana::BlockHeader {
                    block_height: 222,
                    block_time: 0,
                    blockhash: "So11111111111111111111111111111111111111112".to_string(),
                    parent_lot: 0,
                    previous_blockhash: "So00000000000000000000000000000000000000000".to_string(),
                });
                let data = AnyBlockData::Solana(GenericBlockData::<solana::BlockData> {
                    block_header: solana::BlockHeader {
                        block_height: 222,
                        block_time: 0,
                        blockhash: "So11111111111111111111111111111111111111112".to_string(),
                        parent_lot: 0,
                        previous_blockhash: "So00000000000000000000000000000000000000000".to_string(),
                    },
                    transactions: vec![],
                });
                analyzer.analyze(&header, &data);
            }
        }
    }
}