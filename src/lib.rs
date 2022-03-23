use std::fmt::Display;

use dec_utils::dec_to_string_or_empty;
use rust_decimal::prelude::*;
//use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use serde_utc_time_ms::{de_string_to_utc_time_ms, se_time_ms_to_utc_z_string};
use time_ms_conversions::time_ms_to_utc_string;

#[derive(Debug, Deserialize, Serialize, Clone, Ord, Eq, PartialEq, PartialOrd)]
// As the second field this will be used to order records with the same time
pub enum TaxBitRecType {
    Income,

    #[serde(rename = "Transfer In")]
    TransferIn,

    #[serde(rename = "Gift Received")]
    GiftReceived,

    Buy,
    Trade,
    Sale,
    Expense,

    #[serde(rename = "Transfer Out")]
    TransferOut,

    #[serde(rename = "Gift Send")]
    GiftSent,

    Unknown,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
// CSV Header
//   Date and Time, Transaction Type, Sent Quantity, Sent Currency, Sending Source,
//   Received Quantity, Received Currency, Receiving Destination, Fee, Fee Currency,
//   Exchange Transaction ID, Blockchain Transaction Hash
pub struct TaxBitRec {
    #[serde(rename = "Date and Time")]
    #[serde(deserialize_with = "de_string_to_utc_time_ms")]
    #[serde(serialize_with = "se_time_ms_to_utc_z_string")]
    pub time: i64,

    #[serde(rename = "Transaction Type")]
    pub type_txs: TaxBitRecType,

    #[serde(rename = "Sent Quantity")]
    pub sent_quantity: Option<Decimal>,

    #[serde(rename = "Sent Currency")]
    pub sent_currency: String,

    #[serde(rename = "Sending Source")]
    pub sending_source: String,

    #[serde(rename = "Received Quantity")]
    pub received_quantity: Option<Decimal>,

    #[serde(rename = "Received Currency")]
    pub received_currency: String,

    #[serde(rename = "Receiving Destination")]
    pub receiving_destination: String,

    #[serde(rename = "Fee")]
    pub fee_quantity: Option<Decimal>,

    #[serde(rename = "Fee Currency")]
    pub fee_currency: String,

    #[serde(rename = "Exchange Transaction ID")]
    pub exchange_transaction_id: String,

    #[serde(rename = "Blockchain Transaction Hash")]
    pub blockchain_transaction_hash: String,
}

impl Display for TaxBitRec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{:?},{},{},{},{},{},{},{},{},{},{}",
            time_ms_to_utc_string(self.time),
            self.type_txs,
            dec_to_string_or_empty(self.sent_quantity),
            self.sent_currency,
            self.sending_source,
            dec_to_string_or_empty(self.received_quantity),
            self.received_currency,
            self.receiving_destination,
            dec_to_string_or_empty(self.fee_quantity),
            self.fee_currency,
            self.exchange_transaction_id,
            self.blockchain_transaction_hash,
        )
    }
}

impl TaxBitRec {
    pub fn new() -> TaxBitRec {
        TaxBitRec {
            time: 0i64,
            type_txs: TaxBitRecType::Unknown,
            sent_quantity: None,
            sent_currency: "".to_owned(),
            sending_source: "".to_owned(),
            received_quantity: None,
            received_currency: "".to_owned(),
            receiving_destination: "".to_owned(),
            fee_quantity: None,
            fee_currency: "".to_owned(),
            exchange_transaction_id: "".to_owned(),
            blockchain_transaction_hash: "".to_owned(),
        }
    }

    pub fn get_asset(&self) -> &str {
        match self.type_txs {
            TaxBitRecType::Expense
            | TaxBitRecType::TransferOut
            | TaxBitRecType::GiftSent
            | TaxBitRecType::Sale => self.sent_currency.as_str(),
            TaxBitRecType::Buy
            | TaxBitRecType::TransferIn
            | TaxBitRecType::Income
            | TaxBitRecType::GiftReceived
            | TaxBitRecType::Trade => self.received_currency.as_str(),
            TaxBitRecType::Unknown => panic!("SNH"),
        }
    }
}

impl Default for TaxBitRec {
    fn default() -> Self {
        Self::new()
    }
}

impl Eq for TaxBitRec {}

impl PartialEq for TaxBitRec {
    fn eq(&self, other: &Self) -> bool {
        println!("eq");
        self.time == other.time
            && self.exchange_transaction_id == other.exchange_transaction_id
            && self.blockchain_transaction_hash == other.blockchain_transaction_hash
            && self.type_txs == other.type_txs
            && self.received_currency == other.received_currency
            && self.sent_currency == other.sent_currency
            && self.fee_currency == other.fee_currency
            && self.receiving_destination == other.receiving_destination
            && self.sending_source == other.sending_source
            && self.received_quantity == other.received_quantity
            && self.sent_quantity == other.sent_quantity
            && self.fee_quantity == other.fee_quantity
    }
}

impl PartialOrd for TaxBitRec {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        println!("partial_cmp");
        match self.time.partial_cmp(&other.time) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self
            .exchange_transaction_id
            .partial_cmp(&other.exchange_transaction_id)
        {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self
            .blockchain_transaction_hash
            .partial_cmp(&other.blockchain_transaction_hash)
        {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.type_txs.partial_cmp(&other.type_txs) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.received_currency.partial_cmp(&other.received_currency) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.sent_currency.partial_cmp(&other.sent_currency) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.fee_currency.partial_cmp(&other.fee_currency) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self
            .receiving_destination
            .partial_cmp(&other.receiving_destination)
        {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.sending_source.partial_cmp(&other.sending_source) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.received_quantity.partial_cmp(&other.received_quantity) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.sent_quantity.partial_cmp(&other.sent_quantity) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }

        self.fee_quantity.partial_cmp(&other.fee_quantity)
    }
}

impl Ord for TaxBitRec {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.partial_cmp(other) {
            Some(ord) => ord,
            None => panic!("SNH"),
        }
    }
}

#[cfg(test)]
mod test {

    //use time_ms_conversions::{dt_str_to_utc_time_ms, TzMassaging};

    //use super::*;
    //use rust_decimal::prelude::*;
    use rust_decimal_macros::dec;

    use crate::{TaxBitRec, TaxBitRecType};

    #[test]
    fn test_new() {
        let tbr = TaxBitRec::new();
        assert_eq!(tbr.type_txs, TaxBitRecType::Unknown);
        assert_eq!(tbr.sent_quantity, None);
        assert_eq!(tbr.sent_currency, "".to_owned());
        assert_eq!(tbr.sending_source, "".to_owned());
        assert_eq!(tbr.received_quantity, None);
        assert_eq!(tbr.received_currency, "".to_owned());
        assert_eq!(tbr.receiving_destination, "".to_owned());
        assert_eq!(tbr.fee_quantity, None);
        assert_eq!(tbr.fee_currency, "".to_owned());
        assert_eq!(tbr.exchange_transaction_id, "".to_owned());
        assert_eq!(tbr.blockchain_transaction_hash, "".to_owned());
    }

    #[test]
    fn test_default() {
        let tbr_default = TaxBitRec::default();
        let tbr_new = TaxBitRec::new();
        assert_eq!(tbr_default, tbr_new);
    }

    #[test]
    fn test_eqne() {
        let mut tbr = TaxBitRec::default();
        let mut tbr_other = TaxBitRec::default();
        assert!(tbr == tbr_other);

        // The order is important so we go though all the paths,
        // so we modifiy the last test first
        tbr.fee_quantity = Some(dec!(0));
        tbr_other.fee_quantity = Some(dec!(1));
        assert!(tbr != tbr_other);

        tbr.sent_quantity = Some(dec!(0));
        tbr_other.sent_quantity = Some(dec!(1));
        assert!(tbr != tbr_other);

        tbr.received_quantity = Some(dec!(0));
        tbr_other.received_quantity = Some(dec!(1));
        assert!(tbr != tbr_other);

        tbr.sending_source = "a".to_owned();
        tbr_other.sending_source = "b".to_owned();
        assert!(tbr != tbr_other);

        tbr.receiving_destination = "a".to_owned();
        tbr_other.receiving_destination = "b".to_owned();
        assert!(tbr != tbr_other);

        tbr.fee_currency = "a".to_owned();
        tbr_other.fee_currency = "b".to_owned();
        assert!(tbr != tbr_other);

        tbr.sent_currency = "a".to_owned();
        tbr_other.sent_currency = "b".to_owned();
        assert!(tbr != tbr_other);

        tbr.received_currency = "a".to_owned();
        tbr_other.received_currency = "b".to_owned();
        assert!(tbr != tbr_other);

        tbr.type_txs = TaxBitRecType::Expense;
        tbr_other.type_txs = TaxBitRecType::Buy;
        assert!(tbr != tbr_other);

        tbr.blockchain_transaction_hash = "a".to_owned();
        tbr_other.blockchain_transaction_hash = "b".to_owned();
        assert!(tbr != tbr_other);

        tbr.exchange_transaction_id = "a".to_owned();
        tbr_other.exchange_transaction_id = "b".to_owned();
        assert!(tbr != tbr_other);

        tbr.time = 0;
        tbr_other.time = 1;
        assert!(tbr != tbr_other);
    }

    #[test]
    fn test_partial_ord() {
        let mut tbr = TaxBitRec::default();
        let mut tbr_other = TaxBitRec::default();

        assert!(tbr <= tbr_other);

        // The order is important so we go though all the paths,
        // so we modifiy the last test first
        tbr.fee_quantity = Some(dec!(0));
        tbr_other.fee_quantity = Some(dec!(1));
        assert!(tbr < tbr_other);

        tbr.sent_quantity = Some(dec!(0));
        tbr_other.sent_quantity = Some(dec!(1));
        assert!(tbr < tbr_other);

        tbr.received_quantity = Some(dec!(0));
        tbr_other.received_quantity = Some(dec!(1));
        assert!(tbr < tbr_other);

        tbr.sending_source = "a".to_owned();
        tbr_other.sending_source = "b".to_owned();
        assert!(tbr < tbr_other);

        tbr.receiving_destination = "a".to_owned();
        tbr_other.receiving_destination = "b".to_owned();
        assert!(tbr < tbr_other);

        tbr.fee_currency = "a".to_owned();
        tbr_other.fee_currency = "b".to_owned();
        assert!(tbr < tbr_other);

        tbr.sent_currency = "a".to_owned();
        tbr_other.sent_currency = "b".to_owned();
        assert!(tbr < tbr_other);

        tbr.received_currency = "a".to_owned();
        tbr_other.received_currency = "b".to_owned();
        assert!(tbr < tbr_other);

        tbr.type_txs = TaxBitRecType::Buy;
        tbr_other.type_txs = TaxBitRecType::Expense;
        assert!(tbr < tbr_other);

        tbr.blockchain_transaction_hash = "a".to_owned();
        tbr_other.blockchain_transaction_hash = "b".to_owned();
        assert!(tbr < tbr_other);

        tbr.exchange_transaction_id = "a".to_owned();
        tbr_other.exchange_transaction_id = "b".to_owned();
        assert!(tbr < tbr_other);

        tbr.time = 0;
        tbr_other.time = 1;
        assert!(tbr < tbr_other);
    }

    #[test]
    fn test_ord() {
        let tbr = TaxBitRec::default();
        let tbr_other = TaxBitRec::default();
        assert_eq!(tbr.cmp(&tbr_other), core::cmp::Ordering::Equal);
    }

    #[test]
    #[should_panic]
    fn test_ord_panic() {
        let mut tbr = TaxBitRec::default();
        let mut tbr_other = TaxBitRec::default();

        // Panic when a field is None and the same field in other is Some
        tbr.received_quantity = None;
        tbr_other.received_quantity = Some(dec!(1));
        assert_eq!(tbr.cmp(&tbr_other), core::cmp::Ordering::Equal);
    }

    #[test]
    #[should_panic]
    fn test_get_asset_panic() {
        let tbr = TaxBitRec::new();

        assert_eq!(tbr.type_txs, TaxBitRecType::Unknown);
        tbr.get_asset();
    }

    #[test]
    fn test_get_asset() {
        let mut tbr = TaxBitRec::new();

        tbr.type_txs = TaxBitRecType::Expense;
        tbr.sent_currency = "ABC".to_owned();
        assert_eq!(tbr.get_asset(), "ABC");

        tbr.type_txs = TaxBitRecType::TransferOut;
        tbr.sent_currency = "ABC".to_owned();
        assert_eq!(tbr.get_asset(), "ABC");

        tbr.type_txs = TaxBitRecType::GiftSent;
        tbr.sent_currency = "ABC".to_owned();
        assert_eq!(tbr.get_asset(), "ABC");

        tbr.type_txs = TaxBitRecType::Sale;
        tbr.sent_currency = "ABC".to_owned();
        assert_eq!(tbr.get_asset(), "ABC");

        tbr.type_txs = TaxBitRecType::Buy;
        tbr.received_currency = "ABC".to_owned();
        assert_eq!(tbr.get_asset(), "ABC");

        tbr.type_txs = TaxBitRecType::TransferIn;
        tbr.received_currency = "ABC".to_owned();
        assert_eq!(tbr.get_asset(), "ABC");

        tbr.type_txs = TaxBitRecType::Income;
        tbr.received_currency = "ABC".to_owned();
        assert_eq!(tbr.get_asset(), "ABC");

        tbr.type_txs = TaxBitRecType::GiftReceived;
        tbr.received_currency = "ABC".to_owned();
        assert_eq!(tbr.get_asset(), "ABC");

        tbr.type_txs = TaxBitRecType::Trade;
        tbr.received_currency = "ABC".to_owned();
        assert_eq!(tbr.get_asset(), "ABC");
    }
}
