use std::fmt::Display;

use dec_utils::dec_to_string_or_empty;
use rust_decimal::prelude::*;
//use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use serde_utc_time_ms::{de_string_to_utc_time_ms, se_time_ms_to_utc_z_string};
use time_ms_conversions::time_ms_to_utc_string;

#[derive(Debug, Deserialize, Serialize, Clone, Ord, Eq, PartialEq, PartialOrd)]
pub enum TaxBitRecType {
    Unknown,

    // Taxable Events
    Sale,
    Trade,
    Expense,

    // Non-taxable Events
    Buy,
    Income,

    #[serde(rename = "Transfer In")]
    TransferIn,

    #[serde(rename = "Transfer Out")]
    TransferOut,

    #[serde(rename = "Gift Received")]
    GiftReceived,

    #[serde(rename = "Gift Send")]
    GiftSent,
}

#[derive(Debug, Deserialize, Serialize, Clone, Ord, Eq, PartialEq, PartialOrd)]
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
    pub txs_type: TaxBitRecType,

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
            self.txs_type,
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
            txs_type: TaxBitRecType::Unknown,
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
        match self.txs_type {
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

#[cfg(test)]
mod test {

    //use time_ms_conversions::{dt_str_to_utc_time_ms, TzMassaging};

    //use super::*;
    //use rust_decimal::prelude::*;
    //use rust_decimal_macros::dec;

    use crate::{TaxBitRec, TaxBitRecType};

    #[test]
    fn test_new() {
        let tbr = TaxBitRec::new();
        assert_eq!(tbr.txs_type, TaxBitRecType::Unknown);
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
    #[should_panic]
    fn test_get_asset_panic() {
        let tbr = TaxBitRec::new();

        assert_eq!(tbr.txs_type, TaxBitRecType::Unknown);
        tbr.get_asset();
    }

    #[test]
    fn test_get_asset() {
        let mut tbr = TaxBitRec::new();

        tbr.txs_type = TaxBitRecType::Expense;
        tbr.sent_currency = "ABC".to_owned();
        assert_eq!(tbr.get_asset(), "ABC");

        tbr.txs_type = TaxBitRecType::TransferOut;
        tbr.sent_currency = "ABC".to_owned();
        assert_eq!(tbr.get_asset(), "ABC");

        tbr.txs_type = TaxBitRecType::GiftSent;
        tbr.sent_currency = "ABC".to_owned();
        assert_eq!(tbr.get_asset(), "ABC");

        tbr.txs_type = TaxBitRecType::Sale;
        tbr.sent_currency = "ABC".to_owned();
        assert_eq!(tbr.get_asset(), "ABC");

        tbr.txs_type = TaxBitRecType::Buy;
        tbr.received_currency = "ABC".to_owned();
        assert_eq!(tbr.get_asset(), "ABC");

        tbr.txs_type = TaxBitRecType::TransferIn;
        tbr.received_currency = "ABC".to_owned();
        assert_eq!(tbr.get_asset(), "ABC");

        tbr.txs_type = TaxBitRecType::Income;
        tbr.received_currency = "ABC".to_owned();
        assert_eq!(tbr.get_asset(), "ABC");

        tbr.txs_type = TaxBitRecType::GiftReceived;
        tbr.received_currency = "ABC".to_owned();
        assert_eq!(tbr.get_asset(), "ABC");

        tbr.txs_type = TaxBitRecType::Trade;
        tbr.received_currency = "ABC".to_owned();
        assert_eq!(tbr.get_asset(), "ABC");
    }
}
