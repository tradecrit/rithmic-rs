pub mod messages;

/// PB_OFFSET = 100000 , is the offset added for each MNM field id
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageType {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestLogin {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, optional, tag = "153634")]
    pub template_version: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "131003")]
    pub user: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "130004")]
    pub password: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "130002")]
    pub app_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "131803")]
    pub app_version: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "153628")]
    pub system_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration = "request_login::SysInfraType", optional, tag = "153621")]
    pub infra_type: ::core::option::Option<i32>,
    #[prost(string, repeated, tag = "144108")]
    pub mac_addr: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "144021")]
    pub os_version: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "144020")]
    pub os_platform: ::core::option::Option<::prost::alloc::string::String>,
    /// applicable only for TICKER_PLANT infra_type
    #[prost(bool, optional, tag = "153644")]
    pub aggregated_quotes: ::core::option::Option<bool>,
}
/// Nested message and enum types in `RequestLogin`.
pub mod request_login {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum SysInfraType {
        TickerPlant = 1,
        OrderPlant = 2,
        HistoryPlant = 3,
        PnlPlant = 4,
        RepositoryPlant = 5,
    }
    impl SysInfraType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::TickerPlant => "TICKER_PLANT",
                Self::OrderPlant => "ORDER_PLANT",
                Self::HistoryPlant => "HISTORY_PLANT",
                Self::PnlPlant => "PNL_PLANT",
                Self::RepositoryPlant => "REPOSITORY_PLANT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TICKER_PLANT" => Some(Self::TickerPlant),
                "ORDER_PLANT" => Some(Self::OrderPlant),
                "HISTORY_PLANT" => Some(Self::HistoryPlant),
                "PNL_PLANT" => Some(Self::PnlPlant),
                "REPOSITORY_PLANT" => Some(Self::RepositoryPlant),
                _ => None,
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseLogin {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, optional, tag = "153634")]
    pub template_version: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154013")]
    pub fcm_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154014")]
    pub ib_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154712")]
    pub country_code: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154713")]
    pub state_code: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "153428")]
    pub unique_user_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(double, optional, tag = "153633")]
    pub heartbeat_interval: ::core::option::Option<f64>,
}
/// PB_OFFSET = 100000 , is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestLogout {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// PB_OFFSET = 100000 , is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseLogout {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_RESPONSE_CODE
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestReferenceData {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
}
/// presence bits defined here is also referred in response_get_instrument_by_underlying.proto and response_search_symbols.proto
/// make sure both these proto files are always same.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseReferenceData {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "149138")]
    pub presence_bits: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "154571")]
    pub clear_bits: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110114")]
    pub exchange_symbol: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "100003")]
    pub symbol_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "157095")]
    pub trading_symbol: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "157096")]
    pub trading_exchange: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "100749")]
    pub product_code: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110116")]
    pub instrument_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "101026")]
    pub underlying_symbol: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "100067")]
    pub expiration_date: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154382")]
    pub currency: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "100109")]
    pub put_call_indicator: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154167")]
    pub tick_size_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154390")]
    pub price_display_format: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154844")]
    pub is_tradable: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154952")]
    pub is_underlying_for_binary_contrats: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(double, optional, tag = "100066")]
    pub strike_price: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "154384")]
    pub ftoq_price: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "154385")]
    pub qtof_price: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "154386")]
    pub min_qprice_change: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "154387")]
    pub min_fprice_change: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "154389")]
    pub single_point_value: ::core::option::Option<f64>,
}
/// Nested message and enum types in `ResponseReferenceData`.
pub mod response_reference_data {
    /// bit constants are defined using enum
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PresenceBits {
        ExchangeSymbol = 1,
        SymbolName = 2,
        ProductCode = 4,
        InstrumentType = 8,
        UnderlyingSymbol = 16,
        ExpirationDate = 32,
        Currency = 64,
        PutCallIndicator = 128,
        StrikePrice = 256,
        FpriceToQprice = 512,
        QpriceToFprice = 1024,
        MinQpriceChange = 2048,
        MinFrpiceChange = 4096,
        SinglePointValue = 8192,
        TickSizeType = 16384,
        PriceDisplayFormat = 32768,
        IsTradable = 65536,
        TradingSymbol = 131072,
        TradingExchange = 262144,
        IsUnderlyingForBinaryContracts = 8388608,
    }
    impl PresenceBits {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::ExchangeSymbol => "EXCHANGE_SYMBOL",
                Self::SymbolName => "SYMBOL_NAME",
                Self::ProductCode => "PRODUCT_CODE",
                Self::InstrumentType => "INSTRUMENT_TYPE",
                Self::UnderlyingSymbol => "UNDERLYING_SYMBOL",
                Self::ExpirationDate => "EXPIRATION_DATE",
                Self::Currency => "CURRENCY",
                Self::PutCallIndicator => "PUT_CALL_INDICATOR",
                Self::StrikePrice => "STRIKE_PRICE",
                Self::FpriceToQprice => "FPRICE_TO_QPRICE",
                Self::QpriceToFprice => "QPRICE_TO_FPRICE",
                Self::MinQpriceChange => "MIN_QPRICE_CHANGE",
                Self::MinFrpiceChange => "MIN_FRPICE_CHANGE",
                Self::SinglePointValue => "SINGLE_POINT_VALUE",
                Self::TickSizeType => "TICK_SIZE_TYPE",
                Self::PriceDisplayFormat => "PRICE_DISPLAY_FORMAT",
                Self::IsTradable => "IS_TRADABLE",
                Self::TradingSymbol => "TRADING_SYMBOL",
                Self::TradingExchange => "TRADING_EXCHANGE",
                Self::IsUnderlyingForBinaryContracts => {
                    "IS_UNDERLYING_FOR_BINARY_CONTRACTS"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "EXCHANGE_SYMBOL" => Some(Self::ExchangeSymbol),
                "SYMBOL_NAME" => Some(Self::SymbolName),
                "PRODUCT_CODE" => Some(Self::ProductCode),
                "INSTRUMENT_TYPE" => Some(Self::InstrumentType),
                "UNDERLYING_SYMBOL" => Some(Self::UnderlyingSymbol),
                "EXPIRATION_DATE" => Some(Self::ExpirationDate),
                "CURRENCY" => Some(Self::Currency),
                "PUT_CALL_INDICATOR" => Some(Self::PutCallIndicator),
                "STRIKE_PRICE" => Some(Self::StrikePrice),
                "FPRICE_TO_QPRICE" => Some(Self::FpriceToQprice),
                "QPRICE_TO_FPRICE" => Some(Self::QpriceToFprice),
                "MIN_QPRICE_CHANGE" => Some(Self::MinQpriceChange),
                "MIN_FRPICE_CHANGE" => Some(Self::MinFrpiceChange),
                "SINGLE_POINT_VALUE" => Some(Self::SinglePointValue),
                "TICK_SIZE_TYPE" => Some(Self::TickSizeType),
                "PRICE_DISPLAY_FORMAT" => Some(Self::PriceDisplayFormat),
                "IS_TRADABLE" => Some(Self::IsTradable),
                "TRADING_SYMBOL" => Some(Self::TradingSymbol),
                "TRADING_EXCHANGE" => Some(Self::TradingExchange),
                "IS_UNDERLYING_FOR_BINARY_CONTRACTS" => {
                    Some(Self::IsUnderlyingForBinaryContracts)
                }
                _ => None,
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestRithmicSystemInfo {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseRithmicSystemInfo {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "153628")]
    pub system_name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, repeated, packed = "false", tag = "153649")]
    pub has_aggregated_quotes: ::prost::alloc::vec::Vec<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestRithmicSystemGatewayInfo {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "153628")]
    pub system_name: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseRithmicSystemGatewayInfo {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "153628")]
    pub system_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "153640")]
    pub gateway_name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "153641")]
    pub gateway_uri: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestHeartbeat {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_SECONDS_SINCE_BOE
    #[prost(int32, optional, tag = "150100")]
    pub ssboe: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_USECS
    #[prost(int32, optional, tag = "150101")]
    pub usecs: ::core::option::Option<i32>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseHeartbeat {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_RESPONSE_CODE
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_SECONDS_SINCE_BOE
    #[prost(int32, optional, tag = "150100")]
    pub ssboe: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_USECS
    #[prost(int32, optional, tag = "150101")]
    pub usecs: ::core::option::Option<i32>,
}
/// PB_OFFSET = 100000 , is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reject {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_RESPONSE_CODE
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// PB_OFFSET = 100000 , is the offset added for each MNM field id
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct ForcedLogout {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserAccountUpdate {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(enumeration = "user_account_update::UpdateType", optional, tag = "154288")]
    pub update_type: ::core::option::Option<i32>,
    #[prost(enumeration = "user_account_update::AccessType", optional, tag = "154000")]
    pub access_type: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "154247")]
    pub source_user_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "131003")]
    pub user: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154013")]
    pub fcm_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154014")]
    pub ib_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154008")]
    pub account_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154002")]
    pub account_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "150100")]
    pub ssboe: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "150101")]
    pub usecs: ::core::option::Option<i32>,
}
/// Nested message and enum types in `UserAccountUpdate`.
pub mod user_account_update {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum UpdateType {
        Add = 1,
        Remove = 2,
    }
    impl UpdateType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Add => "ADD",
                Self::Remove => "REMOVE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ADD" => Some(Self::Add),
                "REMOVE" => Some(Self::Remove),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum AccessType {
        ReadOnly = 0,
        ReadWrite = 1,
    }
    impl AccessType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::ReadOnly => "READ_ONLY",
                Self::ReadWrite => "READ_WRITE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "READ_ONLY" => Some(Self::ReadOnly),
                "READ_WRITE" => Some(Self::ReadWrite),
                _ => None,
            }
        }
    }
}
/// update bits and Request enum defined here is also referred in request_subscribe_by_underlying.proto
/// make sure both these proto files are always same.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestMarketDataUpdate {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(
        enumeration = "request_market_data_update::Request",
        optional,
        tag = "100000"
    )]
    pub request: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "154211")]
    pub update_bits: ::core::option::Option<u32>,
}
/// Nested message and enum types in `RequestMarketDataUpdate`.
pub mod request_market_data_update {
    /// bit constants are defined using enum
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum UpdateBits {
        LastTrade = 1,
        Bbo = 2,
        OrderBook = 4,
        Open = 8,
        OpeningIndicator = 16,
        HighLow = 32,
        HighBidLowAsk = 64,
        Close = 128,
        ClosingIndicator = 256,
        Settlement = 512,
        MarketMode = 1024,
        OpenInterest = 2048,
        MarginRate = 4096,
        HighPriceLimit = 8192,
        LowPriceLimit = 16384,
        ProjectedSettlement = 32768,
        AdjustedClose = 65536,
    }
    impl UpdateBits {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::LastTrade => "LAST_TRADE",
                Self::Bbo => "BBO",
                Self::OrderBook => "ORDER_BOOK",
                Self::Open => "OPEN",
                Self::OpeningIndicator => "OPENING_INDICATOR",
                Self::HighLow => "HIGH_LOW",
                Self::HighBidLowAsk => "HIGH_BID_LOW_ASK",
                Self::Close => "CLOSE",
                Self::ClosingIndicator => "CLOSING_INDICATOR",
                Self::Settlement => "SETTLEMENT",
                Self::MarketMode => "MARKET_MODE",
                Self::OpenInterest => "OPEN_INTEREST",
                Self::MarginRate => "MARGIN_RATE",
                Self::HighPriceLimit => "HIGH_PRICE_LIMIT",
                Self::LowPriceLimit => "LOW_PRICE_LIMIT",
                Self::ProjectedSettlement => "PROJECTED_SETTLEMENT",
                Self::AdjustedClose => "ADJUSTED_CLOSE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "LAST_TRADE" => Some(Self::LastTrade),
                "BBO" => Some(Self::Bbo),
                "ORDER_BOOK" => Some(Self::OrderBook),
                "OPEN" => Some(Self::Open),
                "OPENING_INDICATOR" => Some(Self::OpeningIndicator),
                "HIGH_LOW" => Some(Self::HighLow),
                "HIGH_BID_LOW_ASK" => Some(Self::HighBidLowAsk),
                "CLOSE" => Some(Self::Close),
                "CLOSING_INDICATOR" => Some(Self::ClosingIndicator),
                "SETTLEMENT" => Some(Self::Settlement),
                "MARKET_MODE" => Some(Self::MarketMode),
                "OPEN_INTEREST" => Some(Self::OpenInterest),
                "MARGIN_RATE" => Some(Self::MarginRate),
                "HIGH_PRICE_LIMIT" => Some(Self::HighPriceLimit),
                "LOW_PRICE_LIMIT" => Some(Self::LowPriceLimit),
                "PROJECTED_SETTLEMENT" => Some(Self::ProjectedSettlement),
                "ADJUSTED_CLOSE" => Some(Self::AdjustedClose),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Request {
        Subscribe = 1,
        Unsubscribe = 2,
    }
    impl Request {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Subscribe => "SUBSCRIBE",
                Self::Unsubscribe => "UNSUBSCRIBE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SUBSCRIBE" => Some(Self::Subscribe),
                "UNSUBSCRIBE" => Some(Self::Unsubscribe),
                _ => None,
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseMarketDataUpdate {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestAuxilliaryReferenceData {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_SYMBOL
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_EXCHANGE
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
}
/// presence bits defined here is also referred in response_get_instrument_by_underlying.proto and response_search_symbols.proto
/// make sure both these proto files are always same.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseAuxilliaryReferenceData {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_RESPONSE_CODE
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_PRICING_INDICATOR
    #[prost(uint32, optional, tag = "149138")]
    pub presence_bits: ::core::option::Option<u32>,
    /// PB_OFFSET + MNM_DISPLAY_INDICATOR
    #[prost(uint32, optional, tag = "154571")]
    pub clear_bits: ::core::option::Option<u32>,
    /// PB_OFFSET + MNM_SYMBOL
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_EXCHANGE
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_SETTLEMENT_METHOD
    #[prost(string, optional, tag = "153294")]
    pub settlement_method: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_FIRST_NOTICE_DATE
    #[prost(string, optional, tag = "154932")]
    pub first_notice_date: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_LAST_NOTICE_DATE
    #[prost(string, optional, tag = "154933")]
    pub last_notice_date: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_FIRST_TRADING_DATE
    #[prost(string, optional, tag = "154996")]
    pub first_trading_date: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_LAST_TRADING_DATE
    #[prost(string, optional, tag = "154236")]
    pub last_trading_date: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_FIRST_DELIVERY_DATE
    #[prost(string, optional, tag = "154994")]
    pub first_delivery_date: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_LAST_DELIVERY_DATE
    #[prost(string, optional, tag = "154995")]
    pub last_delivery_date: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_FIRST_POSITION_DATE
    #[prost(string, optional, tag = "154997")]
    pub first_position_date: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_LAST_POSITION_DATE
    #[prost(string, optional, tag = "154998")]
    pub last_position_date: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_UNIT_OF_MEASURE
    #[prost(string, optional, tag = "157023")]
    pub unit_of_measure: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_UNIT_OF_MEASURE_QTY
    #[prost(double, optional, tag = "157024")]
    pub unit_of_measure_qty: ::core::option::Option<f64>,
}
/// Nested message and enum types in `ResponseAuxilliaryReferenceData`.
pub mod response_auxilliary_reference_data {
    /// bit constants are defined using enum
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PresenceBits {
        SettlementMethod = 1,
        FirstNoticeDate = 2,
        LastNoticeDate = 4,
        FirstTradingDate = 8,
        LastTradingDate = 16,
        FirstDeliveryDate = 32,
        LastDeliveryDate = 64,
        FirstPositionDate = 128,
        LastPositionDate = 256,
        UnitOfMeasure = 512,
        UnitOfMeasureQty = 1024,
    }
    impl PresenceBits {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::SettlementMethod => "SETTLEMENT_METHOD",
                Self::FirstNoticeDate => "FIRST_NOTICE_DATE",
                Self::LastNoticeDate => "LAST_NOTICE_DATE",
                Self::FirstTradingDate => "FIRST_TRADING_DATE",
                Self::LastTradingDate => "LAST_TRADING_DATE",
                Self::FirstDeliveryDate => "FIRST_DELIVERY_DATE",
                Self::LastDeliveryDate => "LAST_DELIVERY_DATE",
                Self::FirstPositionDate => "FIRST_POSITION_DATE",
                Self::LastPositionDate => "LAST_POSITION_DATE",
                Self::UnitOfMeasure => "UNIT_OF_MEASURE",
                Self::UnitOfMeasureQty => "UNIT_OF_MEASURE_QTY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SETTLEMENT_METHOD" => Some(Self::SettlementMethod),
                "FIRST_NOTICE_DATE" => Some(Self::FirstNoticeDate),
                "LAST_NOTICE_DATE" => Some(Self::LastNoticeDate),
                "FIRST_TRADING_DATE" => Some(Self::FirstTradingDate),
                "LAST_TRADING_DATE" => Some(Self::LastTradingDate),
                "FIRST_DELIVERY_DATE" => Some(Self::FirstDeliveryDate),
                "LAST_DELIVERY_DATE" => Some(Self::LastDeliveryDate),
                "FIRST_POSITION_DATE" => Some(Self::FirstPositionDate),
                "LAST_POSITION_DATE" => Some(Self::LastPositionDate),
                "UNIT_OF_MEASURE" => Some(Self::UnitOfMeasure),
                "UNIT_OF_MEASURE_QTY" => Some(Self::UnitOfMeasureQty),
                _ => None,
            }
        }
    }
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestGiveTickSizeTypeTable {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_TICK_SIZE_TYPE
    #[prost(string, optional, tag = "154167")]
    pub tick_size_type: ::core::option::Option<::prost::alloc::string::String>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseGiveTickSizeTypeTable {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_REQUEST_HANDLER_RESPONSE_CODE
    #[prost(string, repeated, tag = "132764")]
    pub rq_handler_rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_RESPONSE_CODE
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_PRICING_INDICATOR
    #[prost(uint32, optional, tag = "149138")]
    pub presence_bits: ::core::option::Option<u32>,
    /// PB_OFFSET + MNM_TICK_SIZE_TYPE
    #[prost(string, optional, tag = "154167")]
    pub tick_size_type: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_TICK_SIZE_FP_OPERATOR
    #[prost(string, optional, tag = "154170")]
    pub tick_size_fp_operator: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_TICK_SIZE_LP_OPERATOR
    #[prost(string, optional, tag = "154171")]
    pub tick_size_lp_operator: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_MIN_FPRICE_CHANGE
    #[prost(double, optional, tag = "154387")]
    pub min_fprice_change: ::core::option::Option<f64>,
    /// PB_OFFSET + MNM_TICK_SIZE_FIRST_PRICE
    #[prost(double, optional, tag = "154168")]
    pub tick_size_first_price: ::core::option::Option<f64>,
    /// PB_OFFSET + MNM_TICK_SIZE_LAST_PRICE
    #[prost(double, optional, tag = "154169")]
    pub tick_size_last_price: ::core::option::Option<f64>,
}
/// Nested message and enum types in `ResponseGiveTickSizeTypeTable`.
pub mod response_give_tick_size_type_table {
    /// bit constants are defined using enum
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PresenceBits {
        TickSizeFirstPrice = 1,
        TickSizeLastPrice = 2,
        TickSizeFpOperator = 4,
        TickSizeLpOperator = 8,
    }
    impl PresenceBits {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::TickSizeFirstPrice => "TICK_SIZE_FIRST_PRICE",
                Self::TickSizeLastPrice => "TICK_SIZE_LAST_PRICE",
                Self::TickSizeFpOperator => "TICK_SIZE_FP_OPERATOR",
                Self::TickSizeLpOperator => "TICK_SIZE_LP_OPERATOR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TICK_SIZE_FIRST_PRICE" => Some(Self::TickSizeFirstPrice),
                "TICK_SIZE_LAST_PRICE" => Some(Self::TickSizeLastPrice),
                "TICK_SIZE_FP_OPERATOR" => Some(Self::TickSizeFpOperator),
                "TICK_SIZE_LP_OPERATOR" => Some(Self::TickSizeLpOperator),
                _ => None,
            }
        }
    }
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestGetInstrumentByUnderlying {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_UNDERLYING_SYMBOL
    #[prost(string, optional, tag = "101026")]
    pub underlying_symbol: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_EXCHANGE
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_EXPIRATION_DATE
    #[prost(string, optional, tag = "100067")]
    pub expiration_date: ::core::option::Option<::prost::alloc::string::String>,
}
/// presence bits field defined here is an exact copy of response_reference_data.proto
/// Make sure they are always the same in both proto files.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseGetInstrumentByUnderlying {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_REQUEST_HANDLER_RESPONSE_CODE
    #[prost(string, repeated, tag = "132764")]
    pub rq_handler_rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_RESPONSE_CODE
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_PRICING_INDICATOR
    #[prost(uint32, optional, tag = "149138")]
    pub presence_bits: ::core::option::Option<u32>,
    /// PB_OFFSET + MNM_DISPLAY_INDICATOR
    #[prost(uint32, optional, tag = "154571")]
    pub clear_bits: ::core::option::Option<u32>,
    /// PB_OFFSET + MNM_SYMBOL
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_EXCHANGE
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_EXCHANGE_SYMBOL
    #[prost(string, optional, tag = "110114")]
    pub exchange_symbol: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_SYMBOL_NAME
    #[prost(string, optional, tag = "100003")]
    pub symbol_name: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_PRODUCT_CODE
    #[prost(string, optional, tag = "100749")]
    pub product_code: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_INSTRUMENT_TYPE
    #[prost(string, optional, tag = "110116")]
    pub instrument_type: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_UNDERLYING_SYMBOL
    #[prost(string, optional, tag = "101026")]
    pub underlying_symbol: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_EXPIRATION_DATE
    #[prost(string, optional, tag = "100067")]
    pub expiration_date: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_SYMBOL_CURRENCY
    #[prost(string, optional, tag = "154382")]
    pub currency: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_PUT_CALL_INDICATOR
    #[prost(string, optional, tag = "100109")]
    pub put_call_indicator: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_TICK_SIZE_TYPE
    #[prost(string, optional, tag = "154167")]
    pub tick_size_type: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_PRICE_DISPLAY_FORMAT
    #[prost(string, optional, tag = "154390")]
    pub price_display_format: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_STRIKE_PRICE
    #[prost(double, optional, tag = "100066")]
    pub strike_price: ::core::option::Option<f64>,
    /// PB_OFFSET + MNM_FPRICE_TO_QPRICE
    #[prost(double, optional, tag = "154384")]
    pub ftoq_price: ::core::option::Option<f64>,
    /// PB_OFFSET + MNM_QPRICE_TO_FPRICE
    #[prost(double, optional, tag = "154385")]
    pub qtof_price: ::core::option::Option<f64>,
    /// PB_OFFSET + MNM_MIN_QPRICE_CHANGE
    #[prost(double, optional, tag = "154386")]
    pub min_qprice_change: ::core::option::Option<f64>,
    /// PB_OFFSET + MNM_MIN_FPRICE_CHANGE
    #[prost(double, optional, tag = "154387")]
    pub min_fprice_change: ::core::option::Option<f64>,
    /// PB_OFFSET + MNM_SINGLE_POINT_VALUE
    #[prost(double, optional, tag = "154389")]
    pub single_point_value: ::core::option::Option<f64>,
}
/// Nested message and enum types in `ResponseGetInstrumentByUnderlying`.
pub mod response_get_instrument_by_underlying {
    /// bit constants are defined using enum
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PresenceBits {
        ExchangeSymbol = 1,
        SymbolName = 2,
        ProductCode = 4,
        InstrumentType = 8,
        UnderlyingSymbol = 16,
        ExpirationDate = 32,
        Currency = 64,
        PutCallIndicator = 128,
        StrikePrice = 256,
        FpriceToQprice = 512,
        QpriceToFprice = 1024,
        MinQpriceChange = 2048,
        MinFrpiceChange = 4096,
        SinglePointValue = 8192,
        TickSizeType = 16384,
        PriceDisplayFormat = 32768,
    }
    impl PresenceBits {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::ExchangeSymbol => "EXCHANGE_SYMBOL",
                Self::SymbolName => "SYMBOL_NAME",
                Self::ProductCode => "PRODUCT_CODE",
                Self::InstrumentType => "INSTRUMENT_TYPE",
                Self::UnderlyingSymbol => "UNDERLYING_SYMBOL",
                Self::ExpirationDate => "EXPIRATION_DATE",
                Self::Currency => "CURRENCY",
                Self::PutCallIndicator => "PUT_CALL_INDICATOR",
                Self::StrikePrice => "STRIKE_PRICE",
                Self::FpriceToQprice => "FPRICE_TO_QPRICE",
                Self::QpriceToFprice => "QPRICE_TO_FPRICE",
                Self::MinQpriceChange => "MIN_QPRICE_CHANGE",
                Self::MinFrpiceChange => "MIN_FRPICE_CHANGE",
                Self::SinglePointValue => "SINGLE_POINT_VALUE",
                Self::TickSizeType => "TICK_SIZE_TYPE",
                Self::PriceDisplayFormat => "PRICE_DISPLAY_FORMAT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "EXCHANGE_SYMBOL" => Some(Self::ExchangeSymbol),
                "SYMBOL_NAME" => Some(Self::SymbolName),
                "PRODUCT_CODE" => Some(Self::ProductCode),
                "INSTRUMENT_TYPE" => Some(Self::InstrumentType),
                "UNDERLYING_SYMBOL" => Some(Self::UnderlyingSymbol),
                "EXPIRATION_DATE" => Some(Self::ExpirationDate),
                "CURRENCY" => Some(Self::Currency),
                "PUT_CALL_INDICATOR" => Some(Self::PutCallIndicator),
                "STRIKE_PRICE" => Some(Self::StrikePrice),
                "FPRICE_TO_QPRICE" => Some(Self::FpriceToQprice),
                "QPRICE_TO_FPRICE" => Some(Self::QpriceToFprice),
                "MIN_QPRICE_CHANGE" => Some(Self::MinQpriceChange),
                "MIN_FRPICE_CHANGE" => Some(Self::MinFrpiceChange),
                "SINGLE_POINT_VALUE" => Some(Self::SinglePointValue),
                "TICK_SIZE_TYPE" => Some(Self::TickSizeType),
                "PRICE_DISPLAY_FORMAT" => Some(Self::PriceDisplayFormat),
                _ => None,
            }
        }
    }
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseGetInstrumentByUnderlyingKeys {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_RESPONSE_CODE
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_UNDERLYING_SYMBOL
    #[prost(string, repeated, tag = "101026")]
    pub underlying_symbol: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_EXCHANGE
    #[prost(string, repeated, tag = "110101")]
    pub exchange: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_EXPIRATION_DATE
    #[prost(string, repeated, tag = "100067")]
    pub expiration_date: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// update bits and Request enum field defined here is an exact copy of request_market_data_update.proto
/// Make sure they are always the same in both proto files.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestMarketDataUpdateByUnderlying {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_UNDERLYING_SYMBOL
    #[prost(string, optional, tag = "101026")]
    pub underlying_symbol: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_EXCHANGE
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_EXPIRATION_DATE
    #[prost(string, optional, tag = "100067")]
    pub expiration_date: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_REQUEST
    #[prost(
        enumeration = "request_market_data_update_by_underlying::Request",
        optional,
        tag = "100000"
    )]
    pub request: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_MESSAGE_ID
    #[prost(uint32, optional, tag = "154211")]
    pub update_bits: ::core::option::Option<u32>,
}
/// Nested message and enum types in `RequestMarketDataUpdateByUnderlying`.
pub mod request_market_data_update_by_underlying {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum UpdateBits {
        LastTrade = 1,
        Bbo = 2,
        OrderBook = 4,
        Open = 8,
        OpeningIndicator = 16,
        HighLow = 32,
        HighBidLowAsk = 64,
        Close = 128,
        ClosingIndicator = 256,
        Settlement = 512,
        MarketMode = 1024,
        OpenInterest = 2048,
        MarginRate = 4096,
        HighPriceLimit = 8192,
        LowPriceLimit = 16384,
        ProjectedSettlement = 32768,
    }
    impl UpdateBits {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::LastTrade => "LAST_TRADE",
                Self::Bbo => "BBO",
                Self::OrderBook => "ORDER_BOOK",
                Self::Open => "OPEN",
                Self::OpeningIndicator => "OPENING_INDICATOR",
                Self::HighLow => "HIGH_LOW",
                Self::HighBidLowAsk => "HIGH_BID_LOW_ASK",
                Self::Close => "CLOSE",
                Self::ClosingIndicator => "CLOSING_INDICATOR",
                Self::Settlement => "SETTLEMENT",
                Self::MarketMode => "MARKET_MODE",
                Self::OpenInterest => "OPEN_INTEREST",
                Self::MarginRate => "MARGIN_RATE",
                Self::HighPriceLimit => "HIGH_PRICE_LIMIT",
                Self::LowPriceLimit => "LOW_PRICE_LIMIT",
                Self::ProjectedSettlement => "PROJECTED_SETTLEMENT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "LAST_TRADE" => Some(Self::LastTrade),
                "BBO" => Some(Self::Bbo),
                "ORDER_BOOK" => Some(Self::OrderBook),
                "OPEN" => Some(Self::Open),
                "OPENING_INDICATOR" => Some(Self::OpeningIndicator),
                "HIGH_LOW" => Some(Self::HighLow),
                "HIGH_BID_LOW_ASK" => Some(Self::HighBidLowAsk),
                "CLOSE" => Some(Self::Close),
                "CLOSING_INDICATOR" => Some(Self::ClosingIndicator),
                "SETTLEMENT" => Some(Self::Settlement),
                "MARKET_MODE" => Some(Self::MarketMode),
                "OPEN_INTEREST" => Some(Self::OpenInterest),
                "MARGIN_RATE" => Some(Self::MarginRate),
                "HIGH_PRICE_LIMIT" => Some(Self::HighPriceLimit),
                "LOW_PRICE_LIMIT" => Some(Self::LowPriceLimit),
                "PROJECTED_SETTLEMENT" => Some(Self::ProjectedSettlement),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Request {
        Subscribe = 1,
        Unsubscribe = 2,
    }
    impl Request {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Subscribe => "SUBSCRIBE",
                Self::Unsubscribe => "UNSUBSCRIBE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SUBSCRIBE" => Some(Self::Subscribe),
                "UNSUBSCRIBE" => Some(Self::Unsubscribe),
                _ => None,
            }
        }
    }
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseMarketDataUpdateByUnderlying {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_RESPONSE_CODE
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestSearchSymbols {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_TEXT
    #[prost(string, optional, tag = "120008")]
    pub search_text: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_EXCHANGE
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_PRODUCT_CODE
    #[prost(string, optional, tag = "100749")]
    pub product_code: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_INSTRUMENT_TYPE
    #[prost(
        enumeration = "request_search_symbols::InstrumentType",
        optional,
        tag = "110116"
    )]
    pub instrument_type: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_SEARCH_PATTERN
    #[prost(enumeration = "request_search_symbols::Pattern", optional, tag = "155008")]
    pub pattern: ::core::option::Option<i32>,
}
/// Nested message and enum types in `RequestSearchSymbols`.
pub mod request_search_symbols {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Pattern {
        Equals = 1,
        Contains = 2,
    }
    impl Pattern {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Equals => "EQUALS",
                Self::Contains => "CONTAINS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "EQUALS" => Some(Self::Equals),
                "CONTAINS" => Some(Self::Contains),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum InstrumentType {
        Future = 1,
        FutureOption = 2,
        FutureStrategy = 3,
        Equity = 4,
        EquityOption = 5,
        EquityStrategy = 6,
        Index = 7,
        IndexOption = 8,
        Spread = 9,
        Synthetic = 10,
    }
    impl InstrumentType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Future => "FUTURE",
                Self::FutureOption => "FUTURE_OPTION",
                Self::FutureStrategy => "FUTURE_STRATEGY",
                Self::Equity => "EQUITY",
                Self::EquityOption => "EQUITY_OPTION",
                Self::EquityStrategy => "EQUITY_STRATEGY",
                Self::Index => "INDEX",
                Self::IndexOption => "INDEX_OPTION",
                Self::Spread => "SPREAD",
                Self::Synthetic => "SYNTHETIC",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FUTURE" => Some(Self::Future),
                "FUTURE_OPTION" => Some(Self::FutureOption),
                "FUTURE_STRATEGY" => Some(Self::FutureStrategy),
                "EQUITY" => Some(Self::Equity),
                "EQUITY_OPTION" => Some(Self::EquityOption),
                "EQUITY_STRATEGY" => Some(Self::EquityStrategy),
                "INDEX" => Some(Self::Index),
                "INDEX_OPTION" => Some(Self::IndexOption),
                "SPREAD" => Some(Self::Spread),
                "SYNTHETIC" => Some(Self::Synthetic),
                _ => None,
            }
        }
    }
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseSearchSymbols {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_REQUEST_HANDLER_RESPONSE_CODE
    #[prost(string, repeated, tag = "132764")]
    pub rq_handler_rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_RESPONSE_CODE
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_SYMBOL
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_EXCHANGE
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_SYMBOL_NAME
    #[prost(string, optional, tag = "100003")]
    pub symbol_name: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_PRODUCT_CODE
    #[prost(string, optional, tag = "100749")]
    pub product_code: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_INSTRUMENT_TYPE
    #[prost(string, optional, tag = "110116")]
    pub instrument_type: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_EXPIRATION_DATE
    #[prost(string, optional, tag = "100067")]
    pub expiration_date: ::core::option::Option<::prost::alloc::string::String>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestProductCodes {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_EXCHANGE
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_TOI_PRODUCTS_ONLY
    #[prost(bool, optional, tag = "153499")]
    pub give_toi_products_only: ::core::option::Option<bool>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseProductCodes {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_REQUEST_HANDLER_RESPONSE_CODE
    #[prost(string, repeated, tag = "132764")]
    pub rq_handler_rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_RESPONSE_CODE
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_EXCHANGE
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_SYMBOL_NAME
    #[prost(string, optional, tag = "100003")]
    pub symbol_name: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_PRODUCT_CODE
    #[prost(string, optional, tag = "100749")]
    pub product_code: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_TIMEZONE_TIME_OF_INTEREST
    #[prost(string, optional, tag = "154682")]
    pub timezone_time_of_interest: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    /// PB_OFFSET + MNM_BEGIN_TIME_OF_INTEREST_MSM
    #[prost(int32, optional, tag = "154680")]
    pub begin_time_of_interest_msm: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_END_TIME_OF_INTEREST_MSM
    #[prost(int32, optional, tag = "154681")]
    pub end_time_of_interest_msm: ::core::option::Option<i32>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestFrontMonthContract {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_SYMBOL
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_EXCHANGE
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_SUBSCRIBE_FLAG
    #[prost(bool, optional, tag = "154352")]
    pub need_updates: ::core::option::Option<bool>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseFrontMonthContract {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_RESPONSE_CODE
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_SYMBOL
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_EXCHANGE
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_STATUS_INDICATOR
    #[prost(bool, optional, tag = "149166")]
    pub is_front_month_symbol: ::core::option::Option<bool>,
    /// PB_OFFSET + MNM_SYMBOL_NAME
    #[prost(string, optional, tag = "100003")]
    pub symbol_name: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_TRADING_SYMBOL
    #[prost(string, optional, tag = "157095")]
    pub trading_symbol: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_TRADING_EXCHANGE
    #[prost(string, optional, tag = "157096")]
    pub trading_exchange: ::core::option::Option<::prost::alloc::string::String>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestDepthByOrderSnapshot {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_SYMBOL
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_EXCHANGE
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_MARKET_DEPTH_PRICE
    #[prost(double, optional, tag = "154405")]
    pub depth_price: ::core::option::Option<f64>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseDepthByOrderSnapshot {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_REQUEST_HANDLER_RESPONSE_CODE
    #[prost(string, repeated, tag = "132764")]
    pub rq_handler_rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_RESPONSE_CODE
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_EXCHANGE
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_SYMBOL
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_SEQUENCE_NUMBER
    #[prost(uint64, optional, tag = "112002")]
    pub sequence_number: ::core::option::Option<u64>,
    /// PB_OFFSET + MNM_MARKET_DEPTH_SIDE
    #[prost(
        enumeration = "response_depth_by_order_snapshot::TransactionType",
        optional,
        tag = "153612"
    )]
    pub depth_side: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_MARKET_DEPTH_PRICE
    #[prost(double, optional, tag = "154405")]
    pub depth_price: ::core::option::Option<f64>,
    /// PB_OFFSET + MNM_MARKET_DEPTH_SIZE
    #[prost(int32, repeated, packed = "false", tag = "154406")]
    pub depth_size: ::prost::alloc::vec::Vec<i32>,
    /// PB_OFFSET + MNM_MARKET_DEPTH_ORDER_PRIORITY
    #[prost(uint64, repeated, packed = "false", tag = "153613")]
    pub depth_order_priority: ::prost::alloc::vec::Vec<u64>,
    /// PB_OFFSET + MNM_EXCH_ORD_ID
    #[prost(string, repeated, tag = "149238")]
    pub exchange_order_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `ResponseDepthByOrderSnapshot`.
pub mod response_depth_by_order_snapshot {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum TransactionType {
        Buy = 1,
        Sell = 2,
    }
    impl TransactionType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Buy => "BUY",
                Self::Sell => "SELL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "BUY" => Some(Self::Buy),
                "SELL" => Some(Self::Sell),
                _ => None,
            }
        }
    }
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestDepthByOrderUpdates {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_REQUEST
    #[prost(
        enumeration = "request_depth_by_order_updates::Request",
        optional,
        tag = "100000"
    )]
    pub request: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_SYMBOL
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_EXCHANGE
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_MARKET_DEPTH_PRICE
    #[prost(double, optional, tag = "154405")]
    pub depth_price: ::core::option::Option<f64>,
}
/// Nested message and enum types in `RequestDepthByOrderUpdates`.
pub mod request_depth_by_order_updates {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Request {
        Subscribe = 1,
        Unsubscribe = 2,
    }
    impl Request {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Subscribe => "SUBSCRIBE",
                Self::Unsubscribe => "UNSUBSCRIBE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SUBSCRIBE" => Some(Self::Subscribe),
                "UNSUBSCRIBE" => Some(Self::Unsubscribe),
                _ => None,
            }
        }
    }
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseDepthByOrderUpdates {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_RESPONSE_CODE
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestGetVolumeAtPrice {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_SYMBOL
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_EXCHANGE
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseGetVolumeAtPrice {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_REQUEST_HANDLER_RESPONSE_CODE
    #[prost(string, repeated, tag = "132764")]
    pub rq_handler_rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_RESPONSE_CODE
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_SYMBOL
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_EXCHANGE
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_TRADE_PRICE
    #[prost(double, repeated, packed = "false", tag = "100006")]
    pub trade_price: ::prost::alloc::vec::Vec<f64>,
    /// PB_OFFSET + MNM_VOLUME_AT_PRICE
    #[prost(int32, repeated, packed = "false", tag = "156980")]
    pub volume_at_price: ::prost::alloc::vec::Vec<i32>,
    /// PB_OFFSET + MNM_SECONDS_SINCE_BOE
    #[prost(int32, optional, tag = "150100")]
    pub ssboe: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_USECS
    #[prost(int32, optional, tag = "150101")]
    pub usecs: ::core::option::Option<i32>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BestBidOffer {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_SYMBOL
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_EXCHANGE
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_PRICING_INDICATOR
    #[prost(uint32, optional, tag = "149138")]
    pub presence_bits: ::core::option::Option<u32>,
    /// PB_OFFSET + MNM_DISPLAY_INDICATOR
    #[prost(uint32, optional, tag = "154571")]
    pub clear_bits: ::core::option::Option<u32>,
    /// PB_OFFSET + MNM_UPDATE_TYPE
    #[prost(bool, optional, tag = "110121")]
    pub is_snapshot: ::core::option::Option<bool>,
    /// PB_OFFSET + MNM_BID_PRICE
    #[prost(double, optional, tag = "100022")]
    pub bid_price: ::core::option::Option<f64>,
    /// PB_OFFSET + MNM_BID_SIZE
    #[prost(int32, optional, tag = "100030")]
    pub bid_size: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_BID_NO_OF_ORDERS
    #[prost(int32, optional, tag = "154403")]
    pub bid_orders: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_BID_IMPLICIT_SIZE
    #[prost(int32, optional, tag = "154867")]
    pub bid_implicit_size: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "100266")]
    pub bid_time: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_ASK_PRICE
    #[prost(double, optional, tag = "100025")]
    pub ask_price: ::core::option::Option<f64>,
    /// PB_OFFSET + MNM_ASK_SIZE
    #[prost(int32, optional, tag = "100031")]
    pub ask_size: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_ASK_NO_OF_ORDERS
    #[prost(int32, optional, tag = "154404")]
    pub ask_orders: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_ASK_IMPLICIT_SIZE
    #[prost(int32, optional, tag = "154868")]
    pub ask_implicit_size: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "100267")]
    pub ask_time: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_LEAN_PRICE
    #[prost(double, optional, tag = "154909")]
    pub lean_price: ::core::option::Option<f64>,
    /// PB_OFFSET + MNM_SECONDS_SINCE_BOE
    #[prost(int32, optional, tag = "150100")]
    pub ssboe: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_USECS
    #[prost(int32, optional, tag = "150101")]
    pub usecs: ::core::option::Option<i32>,
}
/// Nested message and enum types in `BestBidOffer`.
pub mod best_bid_offer {
    /// below enum is just for reference only, not used in this message
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PresenceBits {
        Bid = 1,
        Ask = 2,
        LeanPrice = 4,
    }
    impl PresenceBits {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Bid => "BID",
                Self::Ask => "ASK",
                Self::LeanPrice => "LEAN_PRICE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "BID" => Some(Self::Bid),
                "ASK" => Some(Self::Ask),
                "LEAN_PRICE" => Some(Self::LeanPrice),
                _ => None,
            }
        }
    }
}
/// PB_OFFSET = 100000 , is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderBook {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_SYMBOL
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_EXCHANGE
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_PRICING_INDICATOR
    #[prost(uint32, optional, tag = "149138")]
    pub presence_bits: ::core::option::Option<u32>,
    /// PB_OFFSET + MNM_MARKET_DEPTH_UPDATE_TYPE
    #[prost(enumeration = "order_book::UpdateType", optional, tag = "157608")]
    pub update_type: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_MARKET_DEPTH_BID_PRICE
    #[prost(double, repeated, packed = "false", tag = "154282")]
    pub bid_price: ::prost::alloc::vec::Vec<f64>,
    /// PB_OFFSET + MNM_MARKET_DEPTH_BID_SIZE
    #[prost(int32, repeated, packed = "false", tag = "154283")]
    pub bid_size: ::prost::alloc::vec::Vec<i32>,
    /// PB_OFFSET + MNM_MARKET_DEPTH_BID_NO_OF_ORDERS
    #[prost(int32, repeated, packed = "false", tag = "154401")]
    pub bid_orders: ::prost::alloc::vec::Vec<i32>,
    /// PB_OFFSET + MNM_MARKET_DEPTH_IMPLICIT_BID_SIZE
    #[prost(int32, repeated, packed = "false", tag = "154412")]
    pub impl_bid_size: ::prost::alloc::vec::Vec<i32>,
    /// PB_OFFSET + MNM_MARKET_DEPTH_ASK_PRICE
    #[prost(double, repeated, packed = "false", tag = "154284")]
    pub ask_price: ::prost::alloc::vec::Vec<f64>,
    /// PB_OFFSET + MNM_MARKET_DEPTH_ASK_SIZE
    #[prost(int32, repeated, packed = "false", tag = "154285")]
    pub ask_size: ::prost::alloc::vec::Vec<i32>,
    /// PB_OFFSET + MNM_MARKET_DEPTH_ASK_NO_OF_ORDERS
    #[prost(int32, repeated, packed = "false", tag = "154402")]
    pub ask_orders: ::prost::alloc::vec::Vec<i32>,
    /// PB_OFFSET + MNM_MARKET_DEPTH_IMPLICIT_ASK_SIZE
    #[prost(int32, repeated, packed = "false", tag = "154415")]
    pub impl_ask_size: ::prost::alloc::vec::Vec<i32>,
    /// PB_OFFSET + MNM_SECONDS_SINCE_BOE
    #[prost(int32, optional, tag = "150100")]
    pub ssboe: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_USECS
    #[prost(int32, optional, tag = "150101")]
    pub usecs: ::core::option::Option<i32>,
}
/// Nested message and enum types in `OrderBook`.
pub mod order_book {
    /// below enum is just for reference only, not used in this message
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PresenceBits {
        Bid = 1,
        Ask = 2,
    }
    impl PresenceBits {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Bid => "BID",
                Self::Ask => "ASK",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "BID" => Some(Self::Bid),
                "ASK" => Some(Self::Ask),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum UpdateType {
        ClearOrderBook = 1,
        NoBook = 2,
        SnapshotImage = 3,
        Begin = 4,
        Middle = 5,
        End = 6,
        Solo = 7,
    }
    impl UpdateType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::ClearOrderBook => "CLEAR_ORDER_BOOK",
                Self::NoBook => "NO_BOOK",
                Self::SnapshotImage => "SNAPSHOT_IMAGE",
                Self::Begin => "BEGIN",
                Self::Middle => "MIDDLE",
                Self::End => "END",
                Self::Solo => "SOLO",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CLEAR_ORDER_BOOK" => Some(Self::ClearOrderBook),
                "NO_BOOK" => Some(Self::NoBook),
                "SNAPSHOT_IMAGE" => Some(Self::SnapshotImage),
                "BEGIN" => Some(Self::Begin),
                "MIDDLE" => Some(Self::Middle),
                "END" => Some(Self::End),
                "SOLO" => Some(Self::Solo),
                _ => None,
            }
        }
    }
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LastTrade {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_SYMBOL
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_EXCHANGE
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_PRICING_INDICATOR
    #[prost(uint32, optional, tag = "149138")]
    pub presence_bits: ::core::option::Option<u32>,
    /// PB_OFFSET + MNM_DISPLAY_INDICATOR
    #[prost(uint32, optional, tag = "154571")]
    pub clear_bits: ::core::option::Option<u32>,
    /// PB_OFFSET + MNM_UPDATE_TYPE
    #[prost(bool, optional, tag = "110121")]
    pub is_snapshot: ::core::option::Option<bool>,
    /// PB_OFFSET + MNM_TRADE_PRICE
    #[prost(double, optional, tag = "100006")]
    pub trade_price: ::core::option::Option<f64>,
    /// PB_OFFSET + MNM_TRADE_SIZE
    #[prost(int32, optional, tag = "100178")]
    pub trade_size: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_TRANSACTION_TYPE
    #[prost(enumeration = "last_trade::TransactionType", optional, tag = "112003")]
    pub aggressor: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_EXCH_ORD_ID
    #[prost(string, optional, tag = "149238")]
    pub exchange_order_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_AGGRESSOR_EXCH_ORD_ID
    #[prost(string, optional, tag = "154641")]
    pub aggressor_exchange_order_id: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    /// PB_OFFSET + MNM_NET_CHANGE
    #[prost(double, optional, tag = "100011")]
    pub net_change: ::core::option::Option<f64>,
    /// PB_OFFSET + MNM_PERCENT_CHANGE
    #[prost(double, optional, tag = "100056")]
    pub percent_change: ::core::option::Option<f64>,
    /// PB_OFFSET + MNM_TRADE_VOLUME
    #[prost(uint64, optional, tag = "100032")]
    pub volume: ::core::option::Option<u64>,
    /// PB_OFFSET + MNM_VWAP
    #[prost(double, optional, tag = "101379")]
    pub vwap: ::core::option::Option<f64>,
    #[prost(string, optional, tag = "100379")]
    pub trade_time: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_SECONDS_SINCE_BOE
    #[prost(int32, optional, tag = "150100")]
    pub ssboe: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_USECS
    #[prost(int32, optional, tag = "150101")]
    pub usecs: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_SOURCE_SSBOE
    #[prost(int32, optional, tag = "150400")]
    pub source_ssboe: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_SOURCE_USECS
    #[prost(int32, optional, tag = "150401")]
    pub source_usecs: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_SOURCE_NSECS
    #[prost(int32, optional, tag = "150404")]
    pub source_nsecs: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_JOP_SSBOE
    #[prost(int32, optional, tag = "150600")]
    pub jop_ssboe: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_JOP_NSECS
    #[prost(int32, optional, tag = "150604")]
    pub jop_nsecs: ::core::option::Option<i32>,
}
/// Nested message and enum types in `LastTrade`.
pub mod last_trade {
    /// below enum is just for reference only, not used in this message
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PresenceBits {
        LastTrade = 1,
        NetChange = 2,
        PrecentChange = 4,
        Volume = 8,
        Vwap = 16,
    }
    impl PresenceBits {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::LastTrade => "LAST_TRADE",
                Self::NetChange => "NET_CHANGE",
                Self::PrecentChange => "PRECENT_CHANGE",
                Self::Volume => "VOLUME",
                Self::Vwap => "VWAP",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "LAST_TRADE" => Some(Self::LastTrade),
                "NET_CHANGE" => Some(Self::NetChange),
                "PRECENT_CHANGE" => Some(Self::PrecentChange),
                "VOLUME" => Some(Self::Volume),
                "VWAP" => Some(Self::Vwap),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum TransactionType {
        Buy = 1,
        Sell = 2,
    }
    impl TransactionType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Buy => "BUY",
                Self::Sell => "SELL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "BUY" => Some(Self::Buy),
                "SELL" => Some(Self::Sell),
                _ => None,
            }
        }
    }
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeStatistics {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_SYMBOL
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_EXCHANGE
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_PRICING_INDICATOR
    #[prost(uint32, optional, tag = "149138")]
    pub presence_bits: ::core::option::Option<u32>,
    /// PB_OFFSET + MNM_DISPLAY_INDICATOR
    #[prost(uint32, optional, tag = "154571")]
    pub clear_bits: ::core::option::Option<u32>,
    /// PB_OFFSET + MNM_UPDATE_TYPE
    #[prost(bool, optional, tag = "110121")]
    pub is_snapshot: ::core::option::Option<bool>,
    /// PB_OFFSET + MNM_OPEN_PRICE
    #[prost(double, optional, tag = "100019")]
    pub open_price: ::core::option::Option<f64>,
    /// PB_OFFSET + MNM_HIGH_PRICE
    #[prost(double, optional, tag = "100012")]
    pub high_price: ::core::option::Option<f64>,
    /// PB_OFFSET + MNM_LOW_PRICE
    #[prost(double, optional, tag = "100013")]
    pub low_price: ::core::option::Option<f64>,
    /// PB_OFFSET + MNM_SECONDS_SINCE_BOE
    #[prost(int32, optional, tag = "150100")]
    pub ssboe: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_USECS
    #[prost(int32, optional, tag = "150101")]
    pub usecs: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_SOURCE_SSBOE
    #[prost(int32, optional, tag = "150400")]
    pub source_ssboe: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_SOURCE_USECS
    #[prost(int32, optional, tag = "150401")]
    pub source_usecs: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_SOURCE_NSECS
    #[prost(int32, optional, tag = "150404")]
    pub source_nsecs: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_JOP_SSBOE
    #[prost(int32, optional, tag = "150600")]
    pub jop_ssboe: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_JOP_NSECS
    #[prost(int32, optional, tag = "150604")]
    pub jop_nsecs: ::core::option::Option<i32>,
}
/// Nested message and enum types in `TradeStatistics`.
pub mod trade_statistics {
    /// below enum is just for reference only, not used in this message
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PresenceBits {
        Open = 1,
        High = 2,
        Low = 4,
    }
    impl PresenceBits {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Open => "OPEN",
                Self::High => "HIGH",
                Self::Low => "LOW",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OPEN" => Some(Self::Open),
                "HIGH" => Some(Self::High),
                "LOW" => Some(Self::Low),
                _ => None,
            }
        }
    }
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuoteStatistics {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_SYMBOL
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_EXCHANGE
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_PRICING_INDICATOR
    #[prost(uint32, optional, tag = "149138")]
    pub presence_bits: ::core::option::Option<u32>,
    /// PB_OFFSET + MNM_DISPLAY_INDICATOR
    #[prost(uint32, optional, tag = "154571")]
    pub clear_bits: ::core::option::Option<u32>,
    /// PB_OFFSET + MNM_UPDATE_TYPE
    #[prost(bool, optional, tag = "110121")]
    pub is_snapshot: ::core::option::Option<bool>,
    /// PB_OFFSET + MNM_HIGHEST_BID_PRICE
    #[prost(double, optional, tag = "154195")]
    pub highest_bid_price: ::core::option::Option<f64>,
    /// PB_OFFSET + MNM_LOWEST_ASK_PRICE
    #[prost(double, optional, tag = "154197")]
    pub lowest_ask_price: ::core::option::Option<f64>,
    /// PB_OFFSET + MNM_SECONDS_SINCE_BOE
    #[prost(int32, optional, tag = "150100")]
    pub ssboe: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_USECS
    #[prost(int32, optional, tag = "150101")]
    pub usecs: ::core::option::Option<i32>,
}
/// Nested message and enum types in `QuoteStatistics`.
pub mod quote_statistics {
    /// below enum is just for reference only, not used in this message
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PresenceBits {
        HighestBid = 1,
        LowestAsk = 2,
    }
    impl PresenceBits {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::HighestBid => "HIGHEST_BID",
                Self::LowestAsk => "LOWEST_ASK",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "HIGHEST_BID" => Some(Self::HighestBid),
                "LOWEST_ASK" => Some(Self::LowestAsk),
                _ => None,
            }
        }
    }
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndicatorPrices {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_SYMBOL
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_EXCHANGE
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_PRICING_INDICATOR
    #[prost(uint32, optional, tag = "149138")]
    pub presence_bits: ::core::option::Option<u32>,
    /// PB_OFFSET + MNM_DISPLAY_INDICATOR
    #[prost(uint32, optional, tag = "154571")]
    pub clear_bits: ::core::option::Option<u32>,
    /// PB_OFFSET + MNM_UPDATE_TYPE
    #[prost(bool, optional, tag = "110121")]
    pub is_snapshot: ::core::option::Option<bool>,
    /// PB_OFFSET + MNM_OPENING_INDICATOR
    #[prost(double, optional, tag = "154522")]
    pub opening_indicator: ::core::option::Option<f64>,
    /// PB_OFFSET + MNM_CLOSING_INDICATOR
    #[prost(double, optional, tag = "154064")]
    pub closing_indicator: ::core::option::Option<f64>,
    /// PB_OFFSET + MNM_SECONDS_SINCE_BOE
    #[prost(int32, optional, tag = "150100")]
    pub ssboe: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_USECS
    #[prost(int32, optional, tag = "150101")]
    pub usecs: ::core::option::Option<i32>,
}
/// Nested message and enum types in `IndicatorPrices`.
pub mod indicator_prices {
    /// below enum is just for reference only, not used in this message
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PresenceBits {
        OpeningIndicator = 1,
        ClosingIndicator = 2,
    }
    impl PresenceBits {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::OpeningIndicator => "OPENING_INDICATOR",
                Self::ClosingIndicator => "CLOSING_INDICATOR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OPENING_INDICATOR" => Some(Self::OpeningIndicator),
                "CLOSING_INDICATOR" => Some(Self::ClosingIndicator),
                _ => None,
            }
        }
    }
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenInterest {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_SYMBOL
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_EXCHANGE
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_UPDATE_TYPE
    #[prost(bool, optional, tag = "110121")]
    pub is_snapshot: ::core::option::Option<bool>,
    /// PB_OFFSET + MNM_DISPLAY_INDICATOR
    #[prost(bool, optional, tag = "154571")]
    pub should_clear: ::core::option::Option<bool>,
    /// PB_OFFSET + MNM_OPEN_INTEREST
    #[prost(uint64, optional, tag = "100064")]
    pub open_interest: ::core::option::Option<u64>,
    /// PB_OFFSET + MNM_SECONDS_SINCE_BOE
    #[prost(int32, optional, tag = "150100")]
    pub ssboe: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_USECS
    #[prost(int32, optional, tag = "150101")]
    pub usecs: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndOfDayPrices {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "149138")]
    pub presence_bits: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "154571")]
    pub clear_bits: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "110121")]
    pub is_snapshot: ::core::option::Option<bool>,
    #[prost(double, optional, tag = "100021")]
    pub close_price: ::core::option::Option<f64>,
    #[prost(string, optional, tag = "100079")]
    pub close_date: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(double, optional, tag = "154124")]
    pub adjusted_close_price: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "100070")]
    pub settlement_price: ::core::option::Option<f64>,
    #[prost(string, optional, tag = "154132")]
    pub settlement_date: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154637")]
    pub settlement_price_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(double, optional, tag = "155005")]
    pub projected_settlement_price: ::core::option::Option<f64>,
    #[prost(int32, optional, tag = "150100")]
    pub ssboe: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "150101")]
    pub usecs: ::core::option::Option<i32>,
}
/// Nested message and enum types in `EndOfDayPrices`.
pub mod end_of_day_prices {
    /// below enum is just for reference only, not used in this message
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PresenceBits {
        Close = 1,
        Settlement = 2,
        ProjectedSettlement = 4,
        AdjustedClose = 8,
    }
    impl PresenceBits {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Close => "CLOSE",
                Self::Settlement => "SETTLEMENT",
                Self::ProjectedSettlement => "PROJECTED_SETTLEMENT",
                Self::AdjustedClose => "ADJUSTED_CLOSE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CLOSE" => Some(Self::Close),
                "SETTLEMENT" => Some(Self::Settlement),
                "PROJECTED_SETTLEMENT" => Some(Self::ProjectedSettlement),
                "ADJUSTED_CLOSE" => Some(Self::AdjustedClose),
                _ => None,
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketMode {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154106")]
    pub market_mode: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154838")]
    pub halt_reason: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "100016")]
    pub trade_date: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "150100")]
    pub ssboe: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "150101")]
    pub usecs: ::core::option::Option<i32>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FrontMonthContractUpdate {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_SYMBOL
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_EXCHANGE
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_STATUS_INDICATOR
    #[prost(bool, optional, tag = "149166")]
    pub is_front_month_symbol: ::core::option::Option<bool>,
    /// PB_OFFSET + MNM_SYMBOL_NAME
    #[prost(string, optional, tag = "100003")]
    pub symbol_name: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_TRADING_SYMBOL
    #[prost(string, optional, tag = "157095")]
    pub trading_symbol: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_TRADING_EXCHANGE
    #[prost(string, optional, tag = "157096")]
    pub trading_exchange: ::core::option::Option<::prost::alloc::string::String>,
}
/// PB_OFFSET = 100000 , is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DepthByOrder {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_SYMBOL
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_EXCHANGE
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_SEQUENCE_NUMBER
    #[prost(uint64, optional, tag = "112002")]
    pub sequence_number: ::core::option::Option<u64>,
    /// PB_OFFSET + MNM_UPDATE_TYPE
    #[prost(
        enumeration = "depth_by_order::UpdateType",
        repeated,
        packed = "false",
        tag = "110121"
    )]
    pub update_type: ::prost::alloc::vec::Vec<i32>,
    /// PB_OFFSET + MNM_MARKET_DEPTH_SIDE
    #[prost(
        enumeration = "depth_by_order::TransactionType",
        repeated,
        packed = "false",
        tag = "153612"
    )]
    pub transaction_type: ::prost::alloc::vec::Vec<i32>,
    /// PB_OFFSET + MNM_MARKET_DEPTH_PRICE
    #[prost(double, repeated, packed = "false", tag = "154405")]
    pub depth_price: ::prost::alloc::vec::Vec<f64>,
    /// PB_OFFSET + MNM_PREVIOUS_MARKET_DEPTH_PRICE
    #[prost(double, repeated, packed = "false", tag = "154906")]
    pub prev_depth_price: ::prost::alloc::vec::Vec<f64>,
    /// PB_OFFSET + MNM_PREVIOUS_MARKET_DEPTH_PRICE_FLAG
    #[prost(bool, repeated, packed = "false", tag = "154930")]
    pub prev_depth_price_flag: ::prost::alloc::vec::Vec<bool>,
    /// PB_OFFSET + MNM_MARKET_DEPTH_SIZE
    #[prost(int32, repeated, packed = "false", tag = "154406")]
    pub depth_size: ::prost::alloc::vec::Vec<i32>,
    /// PB_OFFSET + MNM_MARKET_DEPTH_ORDER_PRIORITY
    #[prost(uint64, repeated, packed = "false", tag = "153613")]
    pub depth_order_priority: ::prost::alloc::vec::Vec<u64>,
    /// PB_OFFSET + MNM_EXCH_ORD_ID
    #[prost(string, repeated, tag = "149238")]
    pub exchange_order_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_SECONDS_SINCE_BOE
    #[prost(int32, optional, tag = "150100")]
    pub ssboe: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_USECS
    #[prost(int32, optional, tag = "150101")]
    pub usecs: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_SOURCE_SSBOE
    #[prost(int32, optional, tag = "150400")]
    pub source_ssboe: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_SOURCE_USECS
    #[prost(int32, optional, tag = "150401")]
    pub source_usecs: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_SOURCE_NSECS
    #[prost(int32, optional, tag = "150404")]
    pub source_nsecs: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_JOP_SSBOE
    #[prost(int32, optional, tag = "150600")]
    pub jop_ssboe: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_JOP_NSECS
    #[prost(int32, optional, tag = "150604")]
    pub jop_nsecs: ::core::option::Option<i32>,
}
/// Nested message and enum types in `DepthByOrder`.
pub mod depth_by_order {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum TransactionType {
        Buy = 1,
        Sell = 2,
    }
    impl TransactionType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Buy => "BUY",
                Self::Sell => "SELL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "BUY" => Some(Self::Buy),
                "SELL" => Some(Self::Sell),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum UpdateType {
        New = 1,
        Change = 2,
        Delete = 3,
    }
    impl UpdateType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::New => "NEW",
                Self::Change => "CHANGE",
                Self::Delete => "DELETE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NEW" => Some(Self::New),
                "CHANGE" => Some(Self::Change),
                "DELETE" => Some(Self::Delete),
                _ => None,
            }
        }
    }
}
/// PB_OFFSET = 100000 , is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DepthByOrderEndEvent {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_SYMBOL
    #[prost(string, repeated, tag = "110100")]
    pub symbol: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_EXCHANGE
    #[prost(string, repeated, tag = "110101")]
    pub exchange: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_SEQUENCE_NUMBER
    #[prost(uint64, optional, tag = "112002")]
    pub sequence_number: ::core::option::Option<u64>,
    /// PB_OFFSET + MNM_SECONDS_SINCE_BOE
    #[prost(int32, optional, tag = "150100")]
    pub ssboe: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_USECS
    #[prost(int32, optional, tag = "150101")]
    pub usecs: ::core::option::Option<i32>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SymbolMarginRate {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_SYMBOL
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_EXCHANGE
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_UPDATE_TYPE
    #[prost(bool, optional, tag = "110121")]
    pub is_snapshot: ::core::option::Option<bool>,
    /// PB_OFFSET + MNM_MARGIN_RATE
    #[prost(double, optional, tag = "154103")]
    pub margin_rate: ::core::option::Option<f64>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderPriceLimits {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_SYMBOL
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_EXCHANGE
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_PRICING_INDICATOR
    #[prost(uint32, optional, tag = "149138")]
    pub presence_bits: ::core::option::Option<u32>,
    /// PB_OFFSET + MNM_DISPLAY_INDICATOR
    #[prost(uint32, optional, tag = "154571")]
    pub clear_bits: ::core::option::Option<u32>,
    /// PB_OFFSET + MNM_UPDATE_TYPE
    #[prost(bool, optional, tag = "110121")]
    pub is_snapshot: ::core::option::Option<bool>,
    /// PB_OFFSET + MNM_HIGH_PRICE_LIMIT
    #[prost(double, optional, tag = "154079")]
    pub high_price_limit: ::core::option::Option<f64>,
    /// PB_OFFSET + MNM_LOW_PRICE_LIMIT
    #[prost(double, optional, tag = "154101")]
    pub low_price_limit: ::core::option::Option<f64>,
    /// PB_OFFSET + MNM_SECONDS_SINCE_BOE
    #[prost(int32, optional, tag = "150100")]
    pub ssboe: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_USECS
    #[prost(int32, optional, tag = "150101")]
    pub usecs: ::core::option::Option<i32>,
}
/// Nested message and enum types in `OrderPriceLimits`.
pub mod order_price_limits {
    /// below enum is just for reference only, not used in this message
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PresenceBits {
        HighPriceLimit = 1,
        LowPriceLimit = 2,
    }
    impl PresenceBits {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::HighPriceLimit => "HIGH_PRICE_LIMIT",
                Self::LowPriceLimit => "LOW_PRICE_LIMIT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "HIGH_PRICE_LIMIT" => Some(Self::HighPriceLimit),
                "LOW_PRICE_LIMIT" => Some(Self::LowPriceLimit),
                _ => None,
            }
        }
    }
}
/// PB_OFFSET = 100000 , is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestLoginInfo {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseLoginInfo {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_RESPONSE_CODE
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_FCM_ID
    #[prost(string, optional, tag = "154013")]
    pub fcm_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_IB_ID
    #[prost(string, optional, tag = "154014")]
    pub ib_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_FIRST_NAME
    #[prost(string, optional, tag = "154216")]
    pub first_name: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_LAST_NAME
    #[prost(string, optional, tag = "154217")]
    pub last_name: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_USER_TYPE
    #[prost(enumeration = "response_login_info::UserType", optional, tag = "154036")]
    pub user_type: ::core::option::Option<i32>,
}
/// Nested message and enum types in `ResponseLoginInfo`.
pub mod response_login_info {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum UserType {
        Admin = 0,
        Fcm = 1,
        Ib = 2,
        Trader = 3,
    }
    impl UserType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Admin => "USER_TYPE_ADMIN",
                Self::Fcm => "USER_TYPE_FCM",
                Self::Ib => "USER_TYPE_IB",
                Self::Trader => "USER_TYPE_TRADER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "USER_TYPE_ADMIN" => Some(Self::Admin),
                "USER_TYPE_FCM" => Some(Self::Fcm),
                "USER_TYPE_IB" => Some(Self::Ib),
                "USER_TYPE_TRADER" => Some(Self::Trader),
                _ => None,
            }
        }
    }
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestAccountList {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_FCM_ID
    #[prost(string, optional, tag = "154013")]
    pub fcm_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_IB_ID
    #[prost(string, optional, tag = "154014")]
    pub ib_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_USER_TYPE
    #[prost(enumeration = "request_account_list::UserType", optional, tag = "154036")]
    pub user_type: ::core::option::Option<i32>,
}
/// Nested message and enum types in `RequestAccountList`.
pub mod request_account_list {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum UserType {
        Fcm = 1,
        Ib = 2,
        Trader = 3,
    }
    impl UserType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Fcm => "USER_TYPE_FCM",
                Self::Ib => "USER_TYPE_IB",
                Self::Trader => "USER_TYPE_TRADER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "USER_TYPE_FCM" => Some(Self::Fcm),
                "USER_TYPE_IB" => Some(Self::Ib),
                "USER_TYPE_TRADER" => Some(Self::Trader),
                _ => None,
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseAccountList {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "132764")]
    pub rq_handler_rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154013")]
    pub fcm_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154014")]
    pub ib_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154008")]
    pub account_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154002")]
    pub account_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154383")]
    pub account_currency: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "131035")]
    pub account_auto_liquidate: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "131040")]
    pub auto_liq_threshold_current_value: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestAccountRmsInfo {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_FCM_ID
    #[prost(string, optional, tag = "154013")]
    pub fcm_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_IB_ID
    #[prost(string, optional, tag = "154014")]
    pub ib_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_USER_TYPE
    #[prost(
        enumeration = "request_account_rms_info::UserType",
        optional,
        tag = "154036"
    )]
    pub user_type: ::core::option::Option<i32>,
}
/// Nested message and enum types in `RequestAccountRmsInfo`.
pub mod request_account_rms_info {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum UserType {
        Fcm = 1,
        Ib = 2,
        Trader = 3,
    }
    impl UserType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Fcm => "USER_TYPE_FCM",
                Self::Ib => "USER_TYPE_IB",
                Self::Trader => "USER_TYPE_TRADER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "USER_TYPE_FCM" => Some(Self::Fcm),
                "USER_TYPE_IB" => Some(Self::Ib),
                "USER_TYPE_TRADER" => Some(Self::Trader),
                _ => None,
            }
        }
    }
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseAccountRmsInfo {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_REQUEST_HANDLER_RESPONSE_CODE
    #[prost(string, repeated, tag = "132764")]
    pub rq_handler_rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_RESPONSE_CODE
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_FIELD_PRESENCE_BITS
    #[prost(uint32, optional, tag = "153622")]
    pub presence_bits: ::core::option::Option<u32>,
    /// PB_OFFSET + MNM_FCM_ID
    #[prost(string, optional, tag = "154013")]
    pub fcm_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_IB_ID
    #[prost(string, optional, tag = "154014")]
    pub ib_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_ACCOUNT_ID
    #[prost(string, optional, tag = "154008")]
    pub account_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_ACCOUNT_CURRENCY
    #[prost(string, optional, tag = "154383")]
    pub currency: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_ACCOUNT_STATUS
    #[prost(string, optional, tag = "154003")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_ALGORITHM
    #[prost(string, optional, tag = "150142")]
    pub algorithm: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_ACCOUNT_AUTO_LIQUIDATE_CRITERIA
    #[prost(string, optional, tag = "131036")]
    pub auto_liquidate_criteria: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_ACCOUNT_AUTO_LIQUIDATE
    #[prost(
        enumeration = "response_account_rms_info::AutoLiquidateFlag",
        optional,
        tag = "131035"
    )]
    pub auto_liquidate: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_DISABLE_ON_AUTO_LIQUIDATE_FLAG
    #[prost(
        enumeration = "response_account_rms_info::AutoLiquidateFlag",
        optional,
        tag = "131038"
    )]
    pub disable_on_auto_liquidate: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_ACCOUNT_AUTO_LIQUIDATE_THRESHOLD
    #[prost(double, optional, tag = "131037")]
    pub auto_liquidate_threshold: ::core::option::Option<f64>,
    /// PB_OFFSET + MNM_AUTO_LIQ_MAX_MIN_ACCOUNT_BALANCE
    #[prost(double, optional, tag = "131039")]
    pub auto_liquidate_max_min_account_balance: ::core::option::Option<f64>,
    /// PB_OFFSET + MNM_LOSS_LIMIT
    #[prost(double, optional, tag = "154019")]
    pub loss_limit: ::core::option::Option<f64>,
    /// PB_OFFSET + MNM_MINIMUM_ACCOUNT_BALANCE
    #[prost(double, optional, tag = "156968")]
    pub min_account_balance: ::core::option::Option<f64>,
    /// PB_OFFSET + MNM_MIN_MARGIN_BALANCE
    #[prost(double, optional, tag = "156976")]
    pub min_margin_balance: ::core::option::Option<f64>,
    /// PB_OFFSET + MNM_DEFAULT_COMMISSION
    #[prost(double, optional, tag = "153368")]
    pub default_commission: ::core::option::Option<f64>,
    /// PB_OFFSET + MNM_BUY_LIMIT
    #[prost(int32, optional, tag = "154009")]
    pub buy_limit: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_MAX_LIMIT_QUAN
    #[prost(int32, optional, tag = "110105")]
    pub max_order_quantity: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_SELL_LIMIT
    #[prost(int32, optional, tag = "154035")]
    pub sell_limit: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_CHECK_MIN_ACCT_BALANCE
    #[prost(bool, optional, tag = "156972")]
    pub check_min_account_balance: ::core::option::Option<bool>,
}
/// Nested message and enum types in `ResponseAccountRmsInfo`.
pub mod response_account_rms_info {
    /// below enum is just for reference only, not used in this message
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PresenceBits {
        BuyLimit = 1,
        SellLimit = 2,
        LossLimit = 4,
        MaxOrderQuantity = 8,
        MinAccountBalance = 16,
        MinMarginBalance = 32,
        Algorithm = 64,
        Status = 128,
        Currency = 256,
        DefaultCommission = 512,
    }
    impl PresenceBits {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::BuyLimit => "BUY_LIMIT",
                Self::SellLimit => "SELL_LIMIT",
                Self::LossLimit => "LOSS_LIMIT",
                Self::MaxOrderQuantity => "MAX_ORDER_QUANTITY",
                Self::MinAccountBalance => "MIN_ACCOUNT_BALANCE",
                Self::MinMarginBalance => "MIN_MARGIN_BALANCE",
                Self::Algorithm => "ALGORITHM",
                Self::Status => "STATUS",
                Self::Currency => "CURRENCY",
                Self::DefaultCommission => "DEFAULT_COMMISSION",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "BUY_LIMIT" => Some(Self::BuyLimit),
                "SELL_LIMIT" => Some(Self::SellLimit),
                "LOSS_LIMIT" => Some(Self::LossLimit),
                "MAX_ORDER_QUANTITY" => Some(Self::MaxOrderQuantity),
                "MIN_ACCOUNT_BALANCE" => Some(Self::MinAccountBalance),
                "MIN_MARGIN_BALANCE" => Some(Self::MinMarginBalance),
                "ALGORITHM" => Some(Self::Algorithm),
                "STATUS" => Some(Self::Status),
                "CURRENCY" => Some(Self::Currency),
                "DEFAULT_COMMISSION" => Some(Self::DefaultCommission),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum AutoLiquidateFlag {
        Enabled = 1,
        Disabled = 2,
    }
    impl AutoLiquidateFlag {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Enabled => "ENABLED",
                Self::Disabled => "DISABLED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ENABLED" => Some(Self::Enabled),
                "DISABLED" => Some(Self::Disabled),
                _ => None,
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestAccountRmsUpdates {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154013")]
    pub fcm_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154014")]
    pub ib_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154008")]
    pub account_id: ::core::option::Option<::prost::alloc::string::String>,
    /// values can be either 'subscribe' or 'unsubscribe'
    #[prost(string, optional, tag = "100000")]
    pub request: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "154211")]
    pub update_bits: ::core::option::Option<i32>,
}
/// Nested message and enum types in `RequestAccountRmsUpdates`.
pub mod request_account_rms_updates {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum UpdateBits {
        AutoLiqThresholdCurrentValue = 1,
    }
    impl UpdateBits {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::AutoLiqThresholdCurrentValue => "AUTO_LIQ_THRESHOLD_CURRENT_VALUE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "AUTO_LIQ_THRESHOLD_CURRENT_VALUE" => {
                    Some(Self::AutoLiqThresholdCurrentValue)
                }
                _ => None,
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseAccountRmsUpdates {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestProductRmsInfo {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_FCM_ID
    #[prost(string, optional, tag = "154013")]
    pub fcm_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_IB_ID
    #[prost(string, optional, tag = "154014")]
    pub ib_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_ACCOUNT_ID
    #[prost(string, optional, tag = "154008")]
    pub account_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseProductRmsInfo {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_REQUEST_HANDLER_RESPONSE_CODE
    #[prost(string, repeated, tag = "132764")]
    pub rq_handler_rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_RESPONSE_CODE
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_FIELD_PRESENCE_BITS
    #[prost(uint32, optional, tag = "153622")]
    pub presence_bits: ::core::option::Option<u32>,
    /// PB_OFFSET + MNM_FCM_ID
    #[prost(string, optional, tag = "154013")]
    pub fcm_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_IB_ID
    #[prost(string, optional, tag = "154014")]
    pub ib_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_ACCOUNT_ID
    #[prost(string, optional, tag = "154008")]
    pub account_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_PRODUCT_CODE
    #[prost(string, optional, tag = "100749")]
    pub product_code: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_LOSS_LIMIT
    #[prost(double, optional, tag = "154019")]
    pub loss_limit: ::core::option::Option<f64>,
    /// PB_OFFSET +	MNM_COMMISSION_FILL_RATE
    #[prost(double, optional, tag = "156969")]
    pub commission_fill_rate: ::core::option::Option<f64>,
    /// PB_OFFSET + MNM_BUY_MARGIN_RATE
    #[prost(double, optional, tag = "157003")]
    pub buy_margin_rate: ::core::option::Option<f64>,
    /// PB_OFFSET + MNM_SELL_MARGIN_RATE
    #[prost(double, optional, tag = "157004")]
    pub sell_margin_rate: ::core::option::Option<f64>,
    /// PB_OFFSET + MNM_BUY_LIMIT
    #[prost(int32, optional, tag = "154009")]
    pub buy_limit: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_MAX_LIMIT_QUAN
    #[prost(int32, optional, tag = "110105")]
    pub max_order_quantity: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_SELL_LIMIT
    #[prost(int32, optional, tag = "154035")]
    pub sell_limit: ::core::option::Option<i32>,
}
/// Nested message and enum types in `ResponseProductRmsInfo`.
pub mod response_product_rms_info {
    /// below enum is just for reference only, not used in this message
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PresenceBits {
        BuyLimit = 1,
        SellLimit = 2,
        LossLimit = 4,
        MaxOrderQuantity = 8,
        BuyMarginRate = 16,
        SellMarginRate = 32,
        CommissionFillRate = 64,
    }
    impl PresenceBits {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::BuyLimit => "BUY_LIMIT",
                Self::SellLimit => "SELL_LIMIT",
                Self::LossLimit => "LOSS_LIMIT",
                Self::MaxOrderQuantity => "MAX_ORDER_QUANTITY",
                Self::BuyMarginRate => "BUY_MARGIN_RATE",
                Self::SellMarginRate => "SELL_MARGIN_RATE",
                Self::CommissionFillRate => "COMMISSION_FILL_RATE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "BUY_LIMIT" => Some(Self::BuyLimit),
                "SELL_LIMIT" => Some(Self::SellLimit),
                "LOSS_LIMIT" => Some(Self::LossLimit),
                "MAX_ORDER_QUANTITY" => Some(Self::MaxOrderQuantity),
                "BUY_MARGIN_RATE" => Some(Self::BuyMarginRate),
                "SELL_MARGIN_RATE" => Some(Self::SellMarginRate),
                "COMMISSION_FILL_RATE" => Some(Self::CommissionFillRate),
                _ => None,
            }
        }
    }
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestSubscribeForOrderUpdates {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_FCM_ID
    #[prost(string, optional, tag = "154013")]
    pub fcm_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_IB_ID
    #[prost(string, optional, tag = "154014")]
    pub ib_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_ACCOUNT_ID
    #[prost(string, optional, tag = "154008")]
    pub account_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseSubscribeForOrderUpdates {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_RESPONSE_CODE
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestTradeRoutes {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_SUBSCRIBE_FLAG
    #[prost(bool, optional, tag = "154352")]
    pub subscribe_for_updates: ::core::option::Option<bool>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseTradeRoutes {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_REQUEST_HANDLER_RESPONSE_CODE
    #[prost(string, repeated, tag = "132764")]
    pub rq_handler_rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_RESPONSE_CODE
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_FCM_ID
    #[prost(string, optional, tag = "154013")]
    pub fcm_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_IB_ID
    #[prost(string, optional, tag = "154014")]
    pub ib_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_EXCHANGE
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_TRADE_ROUTE
    #[prost(string, optional, tag = "112016")]
    pub trade_route: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_SERVICE_STATE
    #[prost(string, optional, tag = "131407")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_DEFAULT_ROUTE
    #[prost(bool, optional, tag = "154689")]
    pub is_default: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestNewOrder {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154119")]
    pub user_tag: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154629")]
    pub window_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154013")]
    pub fcm_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154014")]
    pub ib_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154008")]
    pub account_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "112004")]
    pub quantity: ::core::option::Option<i32>,
    #[prost(double, optional, tag = "110306")]
    pub price: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "149247")]
    pub trigger_price: ::core::option::Option<f64>,
    #[prost(
        enumeration = "request_new_order::TransactionType",
        optional,
        tag = "112003"
    )]
    pub transaction_type: ::core::option::Option<i32>,
    #[prost(enumeration = "request_new_order::Duration", optional, tag = "112005")]
    pub duration: ::core::option::Option<i32>,
    #[prost(enumeration = "request_new_order::PriceType", optional, tag = "112008")]
    pub price_type: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "112016")]
    pub trade_route: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration = "request_new_order::OrderPlacement", optional, tag = "154710")]
    pub manual_or_auto: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "157063")]
    pub trailing_stop: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "157064")]
    pub trail_by_ticks: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "157065")]
    pub trail_by_price_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "154487")]
    pub release_at_ssboe: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "154549")]
    pub release_at_usecs: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "157085")]
    pub cancel_at_ssboe: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "157086")]
    pub cancel_at_usecs: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "154488")]
    pub cancel_after_secs: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "154451")]
    pub if_touched_symbol: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154452")]
    pub if_touched_exchange: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration = "request_new_order::Condition", optional, tag = "154453")]
    pub if_touched_condition: ::core::option::Option<i32>,
    #[prost(enumeration = "request_new_order::PriceField", optional, tag = "154454")]
    pub if_touched_price_field: ::core::option::Option<i32>,
    #[prost(double, optional, tag = "153632")]
    pub if_touched_price: ::core::option::Option<f64>,
}
/// Nested message and enum types in `RequestNewOrder`.
pub mod request_new_order {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum TransactionType {
        Buy = 1,
        Sell = 2,
    }
    impl TransactionType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Buy => "BUY",
                Self::Sell => "SELL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "BUY" => Some(Self::Buy),
                "SELL" => Some(Self::Sell),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum OrderPlacement {
        Manual = 1,
        Auto = 2,
    }
    impl OrderPlacement {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Manual => "MANUAL",
                Self::Auto => "AUTO",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MANUAL" => Some(Self::Manual),
                "AUTO" => Some(Self::Auto),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Duration {
        Day = 1,
        Gtc = 2,
        Ioc = 3,
        Fok = 4,
    }
    impl Duration {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Day => "DAY",
                Self::Gtc => "GTC",
                Self::Ioc => "IOC",
                Self::Fok => "FOK",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DAY" => Some(Self::Day),
                "GTC" => Some(Self::Gtc),
                "IOC" => Some(Self::Ioc),
                "FOK" => Some(Self::Fok),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PriceType {
        Limit = 1,
        Market = 2,
        StopLimit = 3,
        StopMarket = 4,
        MarketIfTouched = 5,
        LimitIfTouched = 6,
    }
    impl PriceType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Limit => "LIMIT",
                Self::Market => "MARKET",
                Self::StopLimit => "STOP_LIMIT",
                Self::StopMarket => "STOP_MARKET",
                Self::MarketIfTouched => "MARKET_IF_TOUCHED",
                Self::LimitIfTouched => "LIMIT_IF_TOUCHED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "LIMIT" => Some(Self::Limit),
                "MARKET" => Some(Self::Market),
                "STOP_LIMIT" => Some(Self::StopLimit),
                "STOP_MARKET" => Some(Self::StopMarket),
                "MARKET_IF_TOUCHED" => Some(Self::MarketIfTouched),
                "LIMIT_IF_TOUCHED" => Some(Self::LimitIfTouched),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PriceField {
        BidPrice = 1,
        OfferPrice = 2,
        TradePrice = 3,
        LeanPrice = 4,
    }
    impl PriceField {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::BidPrice => "BID_PRICE",
                Self::OfferPrice => "OFFER_PRICE",
                Self::TradePrice => "TRADE_PRICE",
                Self::LeanPrice => "LEAN_PRICE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "BID_PRICE" => Some(Self::BidPrice),
                "OFFER_PRICE" => Some(Self::OfferPrice),
                "TRADE_PRICE" => Some(Self::TradePrice),
                "LEAN_PRICE" => Some(Self::LeanPrice),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Condition {
        EqualTo = 1,
        NotEqualTo = 2,
        GreaterThan = 3,
        GreaterThanEqualTo = 4,
        LesserThan = 5,
        LesserThanEqualTo = 6,
    }
    impl Condition {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::EqualTo => "EQUAL_TO",
                Self::NotEqualTo => "NOT_EQUAL_TO",
                Self::GreaterThan => "GREATER_THAN",
                Self::GreaterThanEqualTo => "GREATER_THAN_EQUAL_TO",
                Self::LesserThan => "LESSER_THAN",
                Self::LesserThanEqualTo => "LESSER_THAN_EQUAL_TO",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "EQUAL_TO" => Some(Self::EqualTo),
                "NOT_EQUAL_TO" => Some(Self::NotEqualTo),
                "GREATER_THAN" => Some(Self::GreaterThan),
                "GREATER_THAN_EQUAL_TO" => Some(Self::GreaterThanEqualTo),
                "LESSER_THAN" => Some(Self::LesserThan),
                "LESSER_THAN_EQUAL_TO" => Some(Self::LesserThanEqualTo),
                _ => None,
            }
        }
    }
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseNewOrder {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_USER_ORIGIN
    #[prost(string, optional, tag = "154119")]
    pub user_tag: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_REQUEST_HANDLER_RESPONSE_CODE
    #[prost(string, repeated, tag = "132764")]
    pub rq_handler_rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_RESPONSE_CODE
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_BASKET_ID
    #[prost(string, optional, tag = "110300")]
    pub basket_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_SECONDS_SINCE_BOE
    #[prost(int32, optional, tag = "150100")]
    pub ssboe: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_USECS
    #[prost(int32, optional, tag = "150101")]
    pub usecs: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestModifyOrder {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154629")]
    pub window_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154013")]
    pub fcm_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154014")]
    pub ib_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154008")]
    pub account_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110300")]
    pub basket_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "112004")]
    pub quantity: ::core::option::Option<i32>,
    #[prost(double, optional, tag = "110306")]
    pub price: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "149247")]
    pub trigger_price: ::core::option::Option<f64>,
    #[prost(enumeration = "request_modify_order::PriceType", optional, tag = "112008")]
    pub price_type: ::core::option::Option<i32>,
    #[prost(
        enumeration = "request_modify_order::OrderPlacement",
        optional,
        tag = "154710"
    )]
    pub manual_or_auto: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "157063")]
    pub trailing_stop: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "157064")]
    pub trail_by_ticks: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "154451")]
    pub if_touched_symbol: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154452")]
    pub if_touched_exchange: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration = "request_modify_order::Condition", optional, tag = "154453")]
    pub if_touched_condition: ::core::option::Option<i32>,
    #[prost(enumeration = "request_modify_order::PriceField", optional, tag = "154454")]
    pub if_touched_price_field: ::core::option::Option<i32>,
    #[prost(double, optional, tag = "153632")]
    pub if_touched_price: ::core::option::Option<f64>,
}
/// Nested message and enum types in `RequestModifyOrder`.
pub mod request_modify_order {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PriceType {
        Limit = 1,
        Market = 2,
        StopLimit = 3,
        StopMarket = 4,
        MarketIfTouched = 5,
        LimitIfTouched = 6,
    }
    impl PriceType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Limit => "LIMIT",
                Self::Market => "MARKET",
                Self::StopLimit => "STOP_LIMIT",
                Self::StopMarket => "STOP_MARKET",
                Self::MarketIfTouched => "MARKET_IF_TOUCHED",
                Self::LimitIfTouched => "LIMIT_IF_TOUCHED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "LIMIT" => Some(Self::Limit),
                "MARKET" => Some(Self::Market),
                "STOP_LIMIT" => Some(Self::StopLimit),
                "STOP_MARKET" => Some(Self::StopMarket),
                "MARKET_IF_TOUCHED" => Some(Self::MarketIfTouched),
                "LIMIT_IF_TOUCHED" => Some(Self::LimitIfTouched),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PriceField {
        BidPrice = 1,
        OfferPrice = 2,
        TradePrice = 3,
        LeanPrice = 4,
    }
    impl PriceField {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::BidPrice => "BID_PRICE",
                Self::OfferPrice => "OFFER_PRICE",
                Self::TradePrice => "TRADE_PRICE",
                Self::LeanPrice => "LEAN_PRICE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "BID_PRICE" => Some(Self::BidPrice),
                "OFFER_PRICE" => Some(Self::OfferPrice),
                "TRADE_PRICE" => Some(Self::TradePrice),
                "LEAN_PRICE" => Some(Self::LeanPrice),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Condition {
        EqualTo = 1,
        NotEqualTo = 2,
        GreaterThan = 3,
        GreaterThanEqualTo = 4,
        LesserThan = 5,
        LesserThanEqualTo = 6,
    }
    impl Condition {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::EqualTo => "EQUAL_TO",
                Self::NotEqualTo => "NOT_EQUAL_TO",
                Self::GreaterThan => "GREATER_THAN",
                Self::GreaterThanEqualTo => "GREATER_THAN_EQUAL_TO",
                Self::LesserThan => "LESSER_THAN",
                Self::LesserThanEqualTo => "LESSER_THAN_EQUAL_TO",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "EQUAL_TO" => Some(Self::EqualTo),
                "NOT_EQUAL_TO" => Some(Self::NotEqualTo),
                "GREATER_THAN" => Some(Self::GreaterThan),
                "GREATER_THAN_EQUAL_TO" => Some(Self::GreaterThanEqualTo),
                "LESSER_THAN" => Some(Self::LesserThan),
                "LESSER_THAN_EQUAL_TO" => Some(Self::LesserThanEqualTo),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum OrderPlacement {
        Manual = 1,
        Auto = 2,
    }
    impl OrderPlacement {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Manual => "MANUAL",
                Self::Auto => "AUTO",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MANUAL" => Some(Self::Manual),
                "AUTO" => Some(Self::Auto),
                _ => None,
            }
        }
    }
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseModifyOrder {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_REQUEST_HANDLER_RESPONSE_CODE
    #[prost(string, repeated, tag = "132764")]
    pub rq_handler_rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_RESPONSE_CODE
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_BASKET_ID
    #[prost(string, optional, tag = "110300")]
    pub basket_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_SECONDS_SINCE_BOE
    #[prost(int32, optional, tag = "150100")]
    pub ssboe: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_USECS
    #[prost(int32, optional, tag = "150101")]
    pub usecs: ::core::option::Option<i32>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestModifyOrderReferenceData {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_USER_ORIGIN
    #[prost(string, optional, tag = "154119")]
    pub user_tag: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_FCM_ID
    #[prost(string, optional, tag = "154013")]
    pub fcm_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_IB_ID
    #[prost(string, optional, tag = "154014")]
    pub ib_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_ACCOUNT_ID
    #[prost(string, optional, tag = "154008")]
    pub account_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_BASKET_ID
    #[prost(string, optional, tag = "110300")]
    pub basket_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseModifyOrderReferenceData {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_RESPONSE_CODE
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestCancelOrder {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154629")]
    pub window_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154013")]
    pub fcm_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154014")]
    pub ib_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154008")]
    pub account_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110300")]
    pub basket_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(
        enumeration = "request_cancel_order::OrderPlacement",
        optional,
        tag = "154710"
    )]
    pub manual_or_auto: ::core::option::Option<i32>,
}
/// Nested message and enum types in `RequestCancelOrder`.
pub mod request_cancel_order {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum OrderPlacement {
        Manual = 1,
        Auto = 2,
    }
    impl OrderPlacement {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Manual => "MANUAL",
                Self::Auto => "AUTO",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MANUAL" => Some(Self::Manual),
                "AUTO" => Some(Self::Auto),
                _ => None,
            }
        }
    }
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseCancelOrder {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_REQUEST_HANDLER_RESPONSE_CODE
    #[prost(string, repeated, tag = "132764")]
    pub rq_handler_rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_RESPONSE_CODE
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_BASKET_ID
    #[prost(string, optional, tag = "110300")]
    pub basket_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_SECONDS_SINCE_BOE
    #[prost(int32, optional, tag = "150100")]
    pub ssboe: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_USECS
    #[prost(int32, optional, tag = "150101")]
    pub usecs: ::core::option::Option<i32>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestCancelAllOrders {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_FCM_ID
    #[prost(string, optional, tag = "154013")]
    pub fcm_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_IB_ID
    #[prost(string, optional, tag = "154014")]
    pub ib_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_ACCOUNT_ID
    #[prost(string, optional, tag = "154008")]
    pub account_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_USER_TYPE
    #[prost(
        enumeration = "request_cancel_all_orders::UserType",
        optional,
        tag = "154036"
    )]
    pub user_type: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_MANUAL_OR_AUTO
    #[prost(
        enumeration = "request_cancel_all_orders::OrderPlacement",
        optional,
        tag = "154710"
    )]
    pub manual_or_auto: ::core::option::Option<i32>,
}
/// Nested message and enum types in `RequestCancelAllOrders`.
pub mod request_cancel_all_orders {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum OrderPlacement {
        Manual = 1,
        Auto = 2,
    }
    impl OrderPlacement {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Manual => "MANUAL",
                Self::Auto => "AUTO",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MANUAL" => Some(Self::Manual),
                "AUTO" => Some(Self::Auto),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum UserType {
        Admin = 0,
        Fcm = 1,
        Ib = 2,
        Trader = 3,
    }
    impl UserType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Admin => "USER_TYPE_ADMIN",
                Self::Fcm => "USER_TYPE_FCM",
                Self::Ib => "USER_TYPE_IB",
                Self::Trader => "USER_TYPE_TRADER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "USER_TYPE_ADMIN" => Some(Self::Admin),
                "USER_TYPE_FCM" => Some(Self::Fcm),
                "USER_TYPE_IB" => Some(Self::Ib),
                "USER_TYPE_TRADER" => Some(Self::Trader),
                _ => None,
            }
        }
    }
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseCancelAllOrders {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_RESPONSE_CODE
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestShowOrders {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_FCM_ID
    #[prost(string, optional, tag = "154013")]
    pub fcm_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_IB_ID
    #[prost(string, optional, tag = "154014")]
    pub ib_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_ACCOUNT_ID
    #[prost(string, optional, tag = "154008")]
    pub account_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseShowOrders {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_RESPONSE_CODE
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestShowOrderHistory {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_FCM_ID
    #[prost(string, optional, tag = "154013")]
    pub fcm_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_IB_ID
    #[prost(string, optional, tag = "154014")]
    pub ib_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_ACCOUNT_ID
    #[prost(string, optional, tag = "154008")]
    pub account_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_BASKET_ID
    #[prost(string, optional, tag = "110300")]
    pub basket_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseShowOrderHistory {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_RESPONSE_CODE
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestShowOrderHistorySummary {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_FCM_ID
    #[prost(string, optional, tag = "154013")]
    pub fcm_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_IB_ID
    #[prost(string, optional, tag = "154014")]
    pub ib_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_ACCOUNT_ID
    #[prost(string, optional, tag = "154008")]
    pub account_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_DATE
    #[prost(string, optional, tag = "110615")]
    pub date: ::core::option::Option<::prost::alloc::string::String>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseShowOrderHistorySummary {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_RESPONSE_CODE
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestShowOrderHistoryDetail {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_FCM_ID
    #[prost(string, optional, tag = "154013")]
    pub fcm_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_IB_ID
    #[prost(string, optional, tag = "154014")]
    pub ib_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_ACCOUNT_ID
    #[prost(string, optional, tag = "154008")]
    pub account_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_BASKET_ID
    #[prost(string, optional, tag = "110300")]
    pub basket_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_DATE
    #[prost(string, optional, tag = "110615")]
    pub date: ::core::option::Option<::prost::alloc::string::String>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseShowOrderHistoryDetail {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_RESPONSE_CODE
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// PB_OFFSET = 100000 , is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestShowOrderHistoryDates {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseShowOrderHistoryDates {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_REQUEST_HANDLER_RESPONSE_CODE
    #[prost(string, repeated, tag = "132764")]
    pub rq_handler_rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_RESPONSE_CODE
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_DATE
    #[prost(string, repeated, tag = "110615")]
    pub date: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestOcoOrder {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "154119")]
    pub user_tag: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "154629")]
    pub window_name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154013")]
    pub fcm_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154014")]
    pub ib_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154008")]
    pub account_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "110100")]
    pub symbol: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "110101")]
    pub exchange: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int32, repeated, packed = "false", tag = "112004")]
    pub quantity: ::prost::alloc::vec::Vec<i32>,
    #[prost(double, repeated, packed = "false", tag = "110306")]
    pub price: ::prost::alloc::vec::Vec<f64>,
    #[prost(double, repeated, packed = "false", tag = "149247")]
    pub trigger_price: ::prost::alloc::vec::Vec<f64>,
    #[prost(
        enumeration = "request_oco_order::TransactionType",
        repeated,
        packed = "false",
        tag = "112003"
    )]
    pub transaction_type: ::prost::alloc::vec::Vec<i32>,
    #[prost(
        enumeration = "request_oco_order::Duration",
        repeated,
        packed = "false",
        tag = "112005"
    )]
    pub duration: ::prost::alloc::vec::Vec<i32>,
    #[prost(
        enumeration = "request_oco_order::PriceType",
        repeated,
        packed = "false",
        tag = "112008"
    )]
    pub price_type: ::prost::alloc::vec::Vec<i32>,
    #[prost(string, repeated, tag = "112016")]
    pub trade_route: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(
        enumeration = "request_oco_order::OrderPlacement",
        repeated,
        packed = "false",
        tag = "154710"
    )]
    pub manual_or_auto: ::prost::alloc::vec::Vec<i32>,
    #[prost(bool, repeated, packed = "false", tag = "157063")]
    pub trailing_stop: ::prost::alloc::vec::Vec<bool>,
    #[prost(int32, repeated, packed = "false", tag = "157064")]
    pub trail_by_ticks: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "157065")]
    pub trail_by_price_id: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, optional, tag = "157085")]
    pub cancel_at_ssboe: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "157086")]
    pub cancel_at_usecs: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "154488")]
    pub cancel_after_secs: ::core::option::Option<i32>,
}
/// Nested message and enum types in `RequestOCOOrder`.
pub mod request_oco_order {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum TransactionType {
        Buy = 1,
        Sell = 2,
    }
    impl TransactionType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Buy => "BUY",
                Self::Sell => "SELL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "BUY" => Some(Self::Buy),
                "SELL" => Some(Self::Sell),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Duration {
        Day = 1,
        Gtc = 2,
        Ioc = 3,
        Fok = 4,
    }
    impl Duration {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Day => "DAY",
                Self::Gtc => "GTC",
                Self::Ioc => "IOC",
                Self::Fok => "FOK",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DAY" => Some(Self::Day),
                "GTC" => Some(Self::Gtc),
                "IOC" => Some(Self::Ioc),
                "FOK" => Some(Self::Fok),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PriceType {
        Limit = 1,
        Market = 2,
        StopLimit = 3,
        StopMarket = 4,
    }
    impl PriceType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Limit => "LIMIT",
                Self::Market => "MARKET",
                Self::StopLimit => "STOP_LIMIT",
                Self::StopMarket => "STOP_MARKET",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "LIMIT" => Some(Self::Limit),
                "MARKET" => Some(Self::Market),
                "STOP_LIMIT" => Some(Self::StopLimit),
                "STOP_MARKET" => Some(Self::StopMarket),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum OrderPlacement {
        Manual = 1,
        Auto = 2,
    }
    impl OrderPlacement {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Manual => "MANUAL",
                Self::Auto => "AUTO",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MANUAL" => Some(Self::Manual),
                "AUTO" => Some(Self::Auto),
                _ => None,
            }
        }
    }
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseOcoOrder {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_USER_ORIGIN
    #[prost(string, repeated, tag = "154119")]
    pub user_tag: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_REQUEST_HANDLER_RESPONSE_CODE
    #[prost(string, repeated, tag = "132764")]
    pub rq_handler_rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_RESPONSE_CODE
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_BASKET_ID
    #[prost(string, repeated, tag = "110300")]
    pub basket_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_SECONDS_SINCE_BOE
    #[prost(int32, repeated, packed = "false", tag = "150100")]
    pub ssboe: ::prost::alloc::vec::Vec<i32>,
    /// PB_OFFSET + MNM_USECS
    #[prost(int32, repeated, packed = "false", tag = "150101")]
    pub usecs: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestBracketOrder {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154119")]
    pub user_tag: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154629")]
    pub window_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154013")]
    pub fcm_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154014")]
    pub ib_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154008")]
    pub account_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "112004")]
    pub quantity: ::core::option::Option<i32>,
    #[prost(double, optional, tag = "110306")]
    pub price: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "149247")]
    pub trigger_price: ::core::option::Option<f64>,
    #[prost(
        enumeration = "request_bracket_order::TransactionType",
        optional,
        tag = "112003"
    )]
    pub transaction_type: ::core::option::Option<i32>,
    #[prost(enumeration = "request_bracket_order::Duration", optional, tag = "112005")]
    pub duration: ::core::option::Option<i32>,
    #[prost(enumeration = "request_bracket_order::PriceType", optional, tag = "112008")]
    pub price_type: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "112016")]
    pub trade_route: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(
        enumeration = "request_bracket_order::OrderPlacement",
        optional,
        tag = "154710"
    )]
    pub manual_or_auto: ::core::option::Option<i32>,
    #[prost(enumeration = "request_bracket_order::UserType", optional, tag = "154036")]
    pub user_type: ::core::option::Option<i32>,
    #[prost(
        enumeration = "request_bracket_order::BracketType",
        optional,
        tag = "157087"
    )]
    pub bracket_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "157170")]
    pub break_even_ticks: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "157172")]
    pub break_even_trigger_ticks: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "154457")]
    pub target_quantity: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "154456")]
    pub target_ticks: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "154459")]
    pub stop_quantity: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "154458")]
    pub stop_ticks: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, optional, tag = "157124")]
    pub trailing_stop_trigger_ticks: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "157062")]
    pub trailing_stop_by_last_trade_price: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "157151")]
    pub target_market_order_if_touched: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "154857")]
    pub stop_market_on_reject: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "157145")]
    pub target_market_at_ssboe: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "157146")]
    pub target_market_at_usecs: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "157147")]
    pub stop_market_at_ssboe: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "157148")]
    pub stop_market_at_usecs: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "157149")]
    pub target_market_order_after_secs: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "154487")]
    pub release_at_ssboe: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "154549")]
    pub release_at_usecs: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "157085")]
    pub cancel_at_ssboe: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "157086")]
    pub cancel_at_usecs: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "154488")]
    pub cancel_after_secs: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "154451")]
    pub if_touched_symbol: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154452")]
    pub if_touched_exchange: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration = "request_bracket_order::Condition", optional, tag = "154453")]
    pub if_touched_condition: ::core::option::Option<i32>,
    #[prost(enumeration = "request_bracket_order::PriceField", optional, tag = "154454")]
    pub if_touched_price_field: ::core::option::Option<i32>,
    #[prost(double, optional, tag = "153632")]
    pub if_touched_price: ::core::option::Option<f64>,
}
/// Nested message and enum types in `RequestBracketOrder`.
pub mod request_bracket_order {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum UserType {
        Admin = 0,
        Fcm = 1,
        Ib = 2,
        Trader = 3,
    }
    impl UserType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Admin => "USER_TYPE_ADMIN",
                Self::Fcm => "USER_TYPE_FCM",
                Self::Ib => "USER_TYPE_IB",
                Self::Trader => "USER_TYPE_TRADER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "USER_TYPE_ADMIN" => Some(Self::Admin),
                "USER_TYPE_FCM" => Some(Self::Fcm),
                "USER_TYPE_IB" => Some(Self::Ib),
                "USER_TYPE_TRADER" => Some(Self::Trader),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum BracketType {
        StopOnly = 1,
        TargetOnly = 2,
        TargetAndStop = 3,
        StopOnlyStatic = 4,
        TargetOnlyStatic = 5,
        TargetAndStopStatic = 6,
    }
    impl BracketType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::StopOnly => "STOP_ONLY",
                Self::TargetOnly => "TARGET_ONLY",
                Self::TargetAndStop => "TARGET_AND_STOP",
                Self::StopOnlyStatic => "STOP_ONLY_STATIC",
                Self::TargetOnlyStatic => "TARGET_ONLY_STATIC",
                Self::TargetAndStopStatic => "TARGET_AND_STOP_STATIC",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STOP_ONLY" => Some(Self::StopOnly),
                "TARGET_ONLY" => Some(Self::TargetOnly),
                "TARGET_AND_STOP" => Some(Self::TargetAndStop),
                "STOP_ONLY_STATIC" => Some(Self::StopOnlyStatic),
                "TARGET_ONLY_STATIC" => Some(Self::TargetOnlyStatic),
                "TARGET_AND_STOP_STATIC" => Some(Self::TargetAndStopStatic),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum TransactionType {
        Buy = 1,
        Sell = 2,
    }
    impl TransactionType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Buy => "BUY",
                Self::Sell => "SELL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "BUY" => Some(Self::Buy),
                "SELL" => Some(Self::Sell),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Duration {
        Day = 1,
        Gtc = 2,
        Ioc = 3,
        Fok = 4,
    }
    impl Duration {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Day => "DAY",
                Self::Gtc => "GTC",
                Self::Ioc => "IOC",
                Self::Fok => "FOK",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DAY" => Some(Self::Day),
                "GTC" => Some(Self::Gtc),
                "IOC" => Some(Self::Ioc),
                "FOK" => Some(Self::Fok),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PriceType {
        Limit = 1,
        Market = 2,
        StopLimit = 3,
        StopMarket = 4,
        MarketIfTouched = 5,
        LimitIfTouched = 6,
    }
    impl PriceType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Limit => "LIMIT",
                Self::Market => "MARKET",
                Self::StopLimit => "STOP_LIMIT",
                Self::StopMarket => "STOP_MARKET",
                Self::MarketIfTouched => "MARKET_IF_TOUCHED",
                Self::LimitIfTouched => "LIMIT_IF_TOUCHED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "LIMIT" => Some(Self::Limit),
                "MARKET" => Some(Self::Market),
                "STOP_LIMIT" => Some(Self::StopLimit),
                "STOP_MARKET" => Some(Self::StopMarket),
                "MARKET_IF_TOUCHED" => Some(Self::MarketIfTouched),
                "LIMIT_IF_TOUCHED" => Some(Self::LimitIfTouched),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum OrderPlacement {
        Manual = 1,
        Auto = 2,
    }
    impl OrderPlacement {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Manual => "MANUAL",
                Self::Auto => "AUTO",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MANUAL" => Some(Self::Manual),
                "AUTO" => Some(Self::Auto),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PriceField {
        BidPrice = 1,
        OfferPrice = 2,
        TradePrice = 3,
        LeanPrice = 4,
    }
    impl PriceField {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::BidPrice => "BID_PRICE",
                Self::OfferPrice => "OFFER_PRICE",
                Self::TradePrice => "TRADE_PRICE",
                Self::LeanPrice => "LEAN_PRICE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "BID_PRICE" => Some(Self::BidPrice),
                "OFFER_PRICE" => Some(Self::OfferPrice),
                "TRADE_PRICE" => Some(Self::TradePrice),
                "LEAN_PRICE" => Some(Self::LeanPrice),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Condition {
        EqualTo = 1,
        NotEqualTo = 2,
        GreaterThan = 3,
        GreaterThanEqualTo = 4,
        LesserThan = 5,
        LesserThanEqualTo = 6,
    }
    impl Condition {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::EqualTo => "EQUAL_TO",
                Self::NotEqualTo => "NOT_EQUAL_TO",
                Self::GreaterThan => "GREATER_THAN",
                Self::GreaterThanEqualTo => "GREATER_THAN_EQUAL_TO",
                Self::LesserThan => "LESSER_THAN",
                Self::LesserThanEqualTo => "LESSER_THAN_EQUAL_TO",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "EQUAL_TO" => Some(Self::EqualTo),
                "NOT_EQUAL_TO" => Some(Self::NotEqualTo),
                "GREATER_THAN" => Some(Self::GreaterThan),
                "GREATER_THAN_EQUAL_TO" => Some(Self::GreaterThanEqualTo),
                "LESSER_THAN" => Some(Self::LesserThan),
                "LESSER_THAN_EQUAL_TO" => Some(Self::LesserThanEqualTo),
                _ => None,
            }
        }
    }
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseBracketOrder {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_USER_ORIGIN
    #[prost(string, optional, tag = "154119")]
    pub user_tag: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_REQUEST_HANDLER_RESPONSE_CODE
    #[prost(string, repeated, tag = "132764")]
    pub rq_handler_rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_RESPONSE_CODE
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_BASKET_ID
    #[prost(string, optional, tag = "110300")]
    pub basket_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_SECONDS_SINCE_BOE
    #[prost(int32, optional, tag = "150100")]
    pub ssboe: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_USECS
    #[prost(int32, optional, tag = "150101")]
    pub usecs: ::core::option::Option<i32>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestShowBrackets {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_FCM_ID
    #[prost(string, optional, tag = "154013")]
    pub fcm_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_IB_ID
    #[prost(string, optional, tag = "154014")]
    pub ib_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_ACCOUNT_ID
    #[prost(string, optional, tag = "154008")]
    pub account_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseShowBrackets {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_REQUEST_HANDLER_RESPONSE_CODE
    #[prost(string, repeated, tag = "132764")]
    pub rq_handler_rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_RESPONSE_CODE
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_BASKET_ID
    #[prost(string, optional, tag = "110300")]
    pub basket_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_TARGET_QUANTITY
    #[prost(string, optional, tag = "154457")]
    pub target_quantity: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_TARGET_QUANTITY_RELEASED
    #[prost(string, optional, tag = "154460")]
    pub target_quantity_released: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_TARGET_TICKS
    #[prost(string, optional, tag = "154456")]
    pub target_ticks: ::core::option::Option<::prost::alloc::string::String>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestShowBracketStops {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_FCM_ID
    #[prost(string, optional, tag = "154013")]
    pub fcm_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_IB_ID
    #[prost(string, optional, tag = "154014")]
    pub ib_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_ACCOUNT_ID
    #[prost(string, optional, tag = "154008")]
    pub account_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseShowBracketStops {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_REQUEST_HANDLER_RESPONSE_CODE
    #[prost(string, repeated, tag = "132764")]
    pub rq_handler_rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_RESPONSE_CODE
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_BASKET_ID
    #[prost(string, optional, tag = "110300")]
    pub basket_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_STOP_QUANTITY
    #[prost(string, optional, tag = "154459")]
    pub stop_quantity: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_STOP_QUANTITY_RELEASED
    #[prost(string, optional, tag = "154466")]
    pub stop_quantity_released: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_STOP_TICKS
    #[prost(string, optional, tag = "154458")]
    pub stop_ticks: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_BRACKET_TRAILING_FIELD_ID
    #[prost(string, optional, tag = "157062")]
    pub bracket_trailing_field_id: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    /// PB_OFFSET + MNM_TRAILING_STOP_TRIGGER_TICKS
    #[prost(string, optional, tag = "157124")]
    pub trailing_stop_trigger_ticks: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestUpdateTargetBracketLevel {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_FCM_ID
    #[prost(string, optional, tag = "154013")]
    pub fcm_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_IB_ID
    #[prost(string, optional, tag = "154014")]
    pub ib_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_ACCOUNT_ID
    #[prost(string, optional, tag = "154008")]
    pub account_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_BASKET_ID
    #[prost(string, optional, tag = "110300")]
    pub basket_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_LEVEL
    #[prost(int32, optional, tag = "154244")]
    pub level: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_TARGET_TICKS
    #[prost(int32, optional, tag = "154456")]
    pub target_ticks: ::core::option::Option<i32>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseUpdateTargetBracketLevel {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_RESPONSE_CODE
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestUpdateStopBracketLevel {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_FCM_ID
    #[prost(string, optional, tag = "154013")]
    pub fcm_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_IB_ID
    #[prost(string, optional, tag = "154014")]
    pub ib_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_ACCOUNT_ID
    #[prost(string, optional, tag = "154008")]
    pub account_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_BASKET_ID
    #[prost(string, optional, tag = "110300")]
    pub basket_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_LEVEL
    #[prost(int32, optional, tag = "154244")]
    pub level: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_STOP_TICKS
    #[prost(int32, optional, tag = "154458")]
    pub stop_ticks: ::core::option::Option<i32>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseUpdateStopBracketLevel {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_RESPONSE_CODE
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestSubscribeToBracketUpdates {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_FCM_ID
    #[prost(string, optional, tag = "154013")]
    pub fcm_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_IB_ID
    #[prost(string, optional, tag = "154014")]
    pub ib_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_ACCOUNT_ID
    #[prost(string, optional, tag = "154008")]
    pub account_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseSubscribeToBracketUpdates {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_RESPONSE_CODE
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// PB_OFFSET = 100000 , is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestListExchangePermissions {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_USER_TARGET
    #[prost(string, optional, tag = "154220")]
    pub user: ::core::option::Option<::prost::alloc::string::String>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseListExchangePermissions {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_REQUEST_HANDLER_RESPONSE_CODE
    #[prost(string, repeated, tag = "132764")]
    pub rq_handler_rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_RESPONSE_CODE
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_EXCHANGE
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_ENTITLEMENT_FLAG
    #[prost(
        enumeration = "response_list_exchange_permissions::EntitlementFlag",
        optional,
        tag = "153400"
    )]
    pub entitlement_flag: ::core::option::Option<i32>,
}
/// Nested message and enum types in `ResponseListExchangePermissions`.
pub mod response_list_exchange_permissions {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum EntitlementFlag {
        Enabled = 1,
        Disabled = 2,
    }
    impl EntitlementFlag {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Enabled => "ENABLED",
                Self::Disabled => "DISABLED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ENABLED" => Some(Self::Enabled),
                "DISABLED" => Some(Self::Disabled),
                _ => None,
            }
        }
    }
}
/// PB_OFFSET = 100000 , is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestLinkOrders {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_FCM_ID
    #[prost(string, repeated, tag = "154013")]
    pub fcm_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_IB_ID
    #[prost(string, repeated, tag = "154014")]
    pub ib_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_ACCOUNT_ID
    #[prost(string, repeated, tag = "154008")]
    pub account_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_BASKET_ID
    #[prost(string, repeated, tag = "110300")]
    pub basket_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// PB_OFFSET = 100000 , is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseLinkOrders {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_RESPONSE_CODE
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestEasyToBorrowList {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_REQUEST
    #[prost(
        enumeration = "request_easy_to_borrow_list::Request",
        optional,
        tag = "100000"
    )]
    pub request: ::core::option::Option<i32>,
}
/// Nested message and enum types in `RequestEasyToBorrowList`.
pub mod request_easy_to_borrow_list {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Request {
        Subscribe = 1,
        Unsubscribe = 2,
    }
    impl Request {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Subscribe => "SUBSCRIBE",
                Self::Unsubscribe => "UNSUBSCRIBE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SUBSCRIBE" => Some(Self::Subscribe),
                "UNSUBSCRIBE" => Some(Self::Unsubscribe),
                _ => None,
            }
        }
    }
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseEasyToBorrowList {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_REQUEST_HANDLER_RESPONSE_CODE
    #[prost(string, repeated, tag = "132764")]
    pub rq_handler_rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_RESPONSE_CODE
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_BROKER_DEALER
    #[prost(string, optional, tag = "154612")]
    pub broker_dealer: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_SYMBOL
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_SYMBOL_NAME
    #[prost(string, optional, tag = "100003")]
    pub symbol_name: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_TOTAL_AVAILABLE_QTY
    #[prost(int32, optional, tag = "154613")]
    pub qty_available: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_TOTAL_USED_QTY
    #[prost(int32, optional, tag = "154614")]
    pub qty_needed: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_SHORT_LIST_INDICATOR
    #[prost(bool, optional, tag = "110353")]
    pub borrowable: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestOrderSessionConfig {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// used only when the server has to fetch refdata from the system instead from it's own database.
    #[prost(bool, optional, tag = "157750")]
    pub should_defer_request: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseOrderSessionConfig {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestExitPosition {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154629")]
    pub window_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154013")]
    pub fcm_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154014")]
    pub ib_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154008")]
    pub account_id: ::core::option::Option<::prost::alloc::string::String>,
    /// optional field, if set, exchange field should also be set.
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    /// optional field, if set, symbol field should also be set.
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    /// optional field
    #[prost(string, optional, tag = "154698")]
    pub trading_algorithm: ::core::option::Option<::prost::alloc::string::String>,
    /// required field
    #[prost(
        enumeration = "request_exit_position::OrderPlacement",
        optional,
        tag = "154710"
    )]
    pub manual_or_auto: ::core::option::Option<i32>,
}
/// Nested message and enum types in `RequestExitPosition`.
pub mod request_exit_position {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum OrderPlacement {
        Manual = 1,
        Auto = 2,
    }
    impl OrderPlacement {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Manual => "MANUAL",
                Self::Auto => "AUTO",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MANUAL" => Some(Self::Manual),
                "AUTO" => Some(Self::Auto),
                _ => None,
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseExitPosition {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "132764")]
    pub rq_handler_rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestReplayExecutions {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154013")]
    pub fcm_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154014")]
    pub ib_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154008")]
    pub account_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "153002")]
    pub start_index: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "153003")]
    pub finish_index: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseReplayExecutions {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeRoute {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_FCM_ID
    #[prost(string, optional, tag = "154013")]
    pub fcm_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_IB_ID
    #[prost(string, optional, tag = "154014")]
    pub ib_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_EXCHANGE
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_TRADE_ROUTE
    #[prost(string, optional, tag = "112016")]
    pub trade_route: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_SERVICE_STATE
    #[prost(string, optional, tag = "131407")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_DEFAULT_ROUTE
    #[prost(bool, optional, tag = "154689")]
    pub is_default: ::core::option::Option<bool>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BracketUpdates {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_FCM_ID
    #[prost(string, optional, tag = "154013")]
    pub fcm_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_IB_ID
    #[prost(string, optional, tag = "154014")]
    pub ib_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_ACCOUNT_ID
    #[prost(string, optional, tag = "154008")]
    pub account_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_BASKET_ID
    #[prost(string, optional, tag = "110300")]
    pub basket_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_STOP_TICKS
    #[prost(int32, optional, tag = "154458")]
    pub stop_ticks: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_STOP_QUANTITY
    #[prost(int32, optional, tag = "154459")]
    pub stop_quantity: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_STOP_QUANTITY_RELEASED
    #[prost(int32, optional, tag = "154466")]
    pub stop_quantity_released: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_TARGET_TICKS
    #[prost(int32, optional, tag = "154456")]
    pub target_ticks: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_TARGET_QUANTITY
    #[prost(int32, optional, tag = "154457")]
    pub target_quantity: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_TARGET_QUANTITY_RELEASED
    #[prost(int32, optional, tag = "154460")]
    pub target_quantity_released: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RithmicOrderNotification {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, optional, tag = "154119")]
    pub user_tag: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(
        enumeration = "rithmic_order_notification::NotifyType",
        optional,
        tag = "153625"
    )]
    pub notify_type: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "110121")]
    pub is_snapshot: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "110303")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110300")]
    pub basket_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154497")]
    pub original_basket_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110358")]
    pub linked_basket_ids: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154013")]
    pub fcm_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154014")]
    pub ib_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "131003")]
    pub user_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154008")]
    pub account_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "112021")]
    pub trade_exchange: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "112016")]
    pub trade_route: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "149238")]
    pub exchange_order_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110116")]
    pub instrument_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "149273")]
    pub completion_reason: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "112004")]
    pub quantity: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "112027")]
    pub quan_release_pending: ::core::option::Option<i32>,
    #[prost(double, optional, tag = "110306")]
    pub price: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "149247")]
    pub trigger_price: ::core::option::Option<f64>,
    #[prost(
        enumeration = "rithmic_order_notification::TransactionType",
        optional,
        tag = "112003"
    )]
    pub transaction_type: ::core::option::Option<i32>,
    #[prost(
        enumeration = "rithmic_order_notification::Duration",
        optional,
        tag = "112005"
    )]
    pub duration: ::core::option::Option<i32>,
    #[prost(
        enumeration = "rithmic_order_notification::PriceType",
        optional,
        tag = "112008"
    )]
    pub price_type: ::core::option::Option<i32>,
    #[prost(
        enumeration = "rithmic_order_notification::PriceType",
        optional,
        tag = "154770"
    )]
    pub orig_price_type: ::core::option::Option<i32>,
    #[prost(
        enumeration = "rithmic_order_notification::OrderPlacement",
        optional,
        tag = "154710"
    )]
    pub manual_or_auto: ::core::option::Option<i32>,
    #[prost(
        enumeration = "rithmic_order_notification::BracketType",
        optional,
        tag = "157087"
    )]
    pub bracket_type: ::core::option::Option<i32>,
    #[prost(double, optional, tag = "110322")]
    pub avg_fill_price: ::core::option::Option<f64>,
    #[prost(int32, optional, tag = "154111")]
    pub total_fill_size: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "154112")]
    pub total_unfilled_size: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "157064")]
    pub trail_by_ticks: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "157065")]
    pub trail_by_price_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "112002")]
    pub sequence_number: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "149263")]
    pub orig_sequence_number: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "149264")]
    pub cor_sequence_number: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154382")]
    pub currency: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154172")]
    pub country_code: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "120008")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "120028")]
    pub report_text: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154806")]
    pub remarks: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154629")]
    pub window_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154671")]
    pub originator_window_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "157085")]
    pub cancel_at_ssboe: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "157086")]
    pub cancel_at_usecs: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "154488")]
    pub cancel_after_secs: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "150100")]
    pub ssboe: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "150101")]
    pub usecs: ::core::option::Option<i32>,
}
/// Nested message and enum types in `RithmicOrderNotification`.
pub mod rithmic_order_notification {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum NotifyType {
        OrderRcvdFromClnt = 1,
        ModifyRcvdFromClnt = 2,
        CancelRcvdFromClnt = 3,
        OpenPending = 4,
        ModifyPending = 5,
        CancelPending = 6,
        OrderRcvdByExchGtwy = 7,
        ModifyRcvdByExchGtwy = 8,
        CancelRcvdByExchGtwy = 9,
        OrderSentToExch = 10,
        ModifySentToExch = 11,
        CancelSentToExch = 12,
        Open = 13,
        Modified = 14,
        Complete = 15,
        ModificationFailed = 16,
        CancellationFailed = 17,
        TriggerPending = 18,
        Generic = 19,
        LinkOrdersFailed = 20,
    }
    impl NotifyType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::OrderRcvdFromClnt => "ORDER_RCVD_FROM_CLNT",
                Self::ModifyRcvdFromClnt => "MODIFY_RCVD_FROM_CLNT",
                Self::CancelRcvdFromClnt => "CANCEL_RCVD_FROM_CLNT",
                Self::OpenPending => "OPEN_PENDING",
                Self::ModifyPending => "MODIFY_PENDING",
                Self::CancelPending => "CANCEL_PENDING",
                Self::OrderRcvdByExchGtwy => "ORDER_RCVD_BY_EXCH_GTWY",
                Self::ModifyRcvdByExchGtwy => "MODIFY_RCVD_BY_EXCH_GTWY",
                Self::CancelRcvdByExchGtwy => "CANCEL_RCVD_BY_EXCH_GTWY",
                Self::OrderSentToExch => "ORDER_SENT_TO_EXCH",
                Self::ModifySentToExch => "MODIFY_SENT_TO_EXCH",
                Self::CancelSentToExch => "CANCEL_SENT_TO_EXCH",
                Self::Open => "OPEN",
                Self::Modified => "MODIFIED",
                Self::Complete => "COMPLETE",
                Self::ModificationFailed => "MODIFICATION_FAILED",
                Self::CancellationFailed => "CANCELLATION_FAILED",
                Self::TriggerPending => "TRIGGER_PENDING",
                Self::Generic => "GENERIC",
                Self::LinkOrdersFailed => "LINK_ORDERS_FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ORDER_RCVD_FROM_CLNT" => Some(Self::OrderRcvdFromClnt),
                "MODIFY_RCVD_FROM_CLNT" => Some(Self::ModifyRcvdFromClnt),
                "CANCEL_RCVD_FROM_CLNT" => Some(Self::CancelRcvdFromClnt),
                "OPEN_PENDING" => Some(Self::OpenPending),
                "MODIFY_PENDING" => Some(Self::ModifyPending),
                "CANCEL_PENDING" => Some(Self::CancelPending),
                "ORDER_RCVD_BY_EXCH_GTWY" => Some(Self::OrderRcvdByExchGtwy),
                "MODIFY_RCVD_BY_EXCH_GTWY" => Some(Self::ModifyRcvdByExchGtwy),
                "CANCEL_RCVD_BY_EXCH_GTWY" => Some(Self::CancelRcvdByExchGtwy),
                "ORDER_SENT_TO_EXCH" => Some(Self::OrderSentToExch),
                "MODIFY_SENT_TO_EXCH" => Some(Self::ModifySentToExch),
                "CANCEL_SENT_TO_EXCH" => Some(Self::CancelSentToExch),
                "OPEN" => Some(Self::Open),
                "MODIFIED" => Some(Self::Modified),
                "COMPLETE" => Some(Self::Complete),
                "MODIFICATION_FAILED" => Some(Self::ModificationFailed),
                "CANCELLATION_FAILED" => Some(Self::CancellationFailed),
                "TRIGGER_PENDING" => Some(Self::TriggerPending),
                "GENERIC" => Some(Self::Generic),
                "LINK_ORDERS_FAILED" => Some(Self::LinkOrdersFailed),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum TransactionType {
        Buy = 1,
        Sell = 2,
        Ss = 3,
    }
    impl TransactionType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Buy => "BUY",
                Self::Sell => "SELL",
                Self::Ss => "SS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "BUY" => Some(Self::Buy),
                "SELL" => Some(Self::Sell),
                "SS" => Some(Self::Ss),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Duration {
        Day = 1,
        Gtc = 2,
        Ioc = 3,
        Fok = 4,
    }
    impl Duration {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Day => "DAY",
                Self::Gtc => "GTC",
                Self::Ioc => "IOC",
                Self::Fok => "FOK",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DAY" => Some(Self::Day),
                "GTC" => Some(Self::Gtc),
                "IOC" => Some(Self::Ioc),
                "FOK" => Some(Self::Fok),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PriceType {
        Limit = 1,
        Market = 2,
        StopLimit = 3,
        StopMarket = 4,
    }
    impl PriceType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Limit => "LIMIT",
                Self::Market => "MARKET",
                Self::StopLimit => "STOP_LIMIT",
                Self::StopMarket => "STOP_MARKET",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "LIMIT" => Some(Self::Limit),
                "MARKET" => Some(Self::Market),
                "STOP_LIMIT" => Some(Self::StopLimit),
                "STOP_MARKET" => Some(Self::StopMarket),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum BracketType {
        StopOnly = 1,
        TargetOnly = 2,
        TargetAndStop = 3,
        StopOnlyStatic = 4,
        TargetOnlyStatic = 5,
        TargetAndStopStatic = 6,
    }
    impl BracketType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::StopOnly => "STOP_ONLY",
                Self::TargetOnly => "TARGET_ONLY",
                Self::TargetAndStop => "TARGET_AND_STOP",
                Self::StopOnlyStatic => "STOP_ONLY_STATIC",
                Self::TargetOnlyStatic => "TARGET_ONLY_STATIC",
                Self::TargetAndStopStatic => "TARGET_AND_STOP_STATIC",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STOP_ONLY" => Some(Self::StopOnly),
                "TARGET_ONLY" => Some(Self::TargetOnly),
                "TARGET_AND_STOP" => Some(Self::TargetAndStop),
                "STOP_ONLY_STATIC" => Some(Self::StopOnlyStatic),
                "TARGET_ONLY_STATIC" => Some(Self::TargetOnlyStatic),
                "TARGET_AND_STOP_STATIC" => Some(Self::TargetAndStopStatic),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum OrderPlacement {
        Manual = 1,
        Auto = 2,
    }
    impl OrderPlacement {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Manual => "MANUAL",
                Self::Auto => "AUTO",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MANUAL" => Some(Self::Manual),
                "AUTO" => Some(Self::Auto),
                _ => None,
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExchangeOrderNotification {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, optional, tag = "154119")]
    pub user_tag: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(
        enumeration = "exchange_order_notification::NotifyType",
        optional,
        tag = "153625"
    )]
    pub notify_type: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "110121")]
    pub is_snapshot: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "149373")]
    pub is_rithmic_internal_msg: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "120001")]
    pub report_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110303")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110300")]
    pub basket_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154497")]
    pub original_basket_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110358")]
    pub linked_basket_ids: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154013")]
    pub fcm_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154014")]
    pub ib_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "131003")]
    pub user_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154008")]
    pub account_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "112021")]
    pub trade_exchange: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "112016")]
    pub trade_route: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "149238")]
    pub exchange_order_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "153647")]
    pub tp_exchange_order_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110116")]
    pub instrument_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "112004")]
    pub quantity: ::core::option::Option<i32>,
    #[prost(double, optional, tag = "110306")]
    pub price: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "149247")]
    pub trigger_price: ::core::option::Option<f64>,
    #[prost(
        enumeration = "exchange_order_notification::TransactionType",
        optional,
        tag = "112003"
    )]
    pub transaction_type: ::core::option::Option<i32>,
    #[prost(
        enumeration = "exchange_order_notification::Duration",
        optional,
        tag = "112005"
    )]
    pub duration: ::core::option::Option<i32>,
    #[prost(
        enumeration = "exchange_order_notification::PriceType",
        optional,
        tag = "112008"
    )]
    pub price_type: ::core::option::Option<i32>,
    #[prost(
        enumeration = "exchange_order_notification::PriceType",
        optional,
        tag = "154770"
    )]
    pub orig_price_type: ::core::option::Option<i32>,
    #[prost(
        enumeration = "exchange_order_notification::OrderPlacement",
        optional,
        tag = "154710"
    )]
    pub manual_or_auto: ::core::option::Option<i32>,
    #[prost(
        enumeration = "exchange_order_notification::BracketType",
        optional,
        tag = "157087"
    )]
    pub bracket_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "110329")]
    pub confirmed_size: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "110326")]
    pub confirmed_time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110327")]
    pub confirmed_date: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110330")]
    pub confirmed_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "149267")]
    pub modified_size: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "149268")]
    pub modified_time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "149269")]
    pub modified_date: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "149244")]
    pub modify_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "110318")]
    pub cancelled_size: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "110319")]
    pub cancelled_time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110320")]
    pub cancelled_date: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110321")]
    pub cancelled_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(double, optional, tag = "110307")]
    pub fill_price: ::core::option::Option<f64>,
    #[prost(int32, optional, tag = "110308")]
    pub fill_size: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "110309")]
    pub fill_time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110310")]
    pub fill_date: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110311")]
    pub fill_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(double, optional, tag = "110322")]
    pub avg_fill_price: ::core::option::Option<f64>,
    #[prost(int32, optional, tag = "154111")]
    pub total_fill_size: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "154112")]
    pub total_unfilled_size: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "149266")]
    pub trigger_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "157064")]
    pub trail_by_ticks: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "157065")]
    pub trail_by_price_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "112002")]
    pub sequence_number: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "149263")]
    pub orig_sequence_number: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "149264")]
    pub cor_sequence_number: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154382")]
    pub currency: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154172")]
    pub country_code: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "120008")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "120028")]
    pub report_text: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154806")]
    pub remarks: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154629")]
    pub window_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154671")]
    pub originator_window_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "157085")]
    pub cancel_at_ssboe: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "157086")]
    pub cancel_at_usecs: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "154488")]
    pub cancel_after_secs: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "150100")]
    pub ssboe: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "150101")]
    pub usecs: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "150405")]
    pub exch_receipt_ssboe: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "150406")]
    pub exch_receipt_nsecs: ::core::option::Option<i32>,
}
/// Nested message and enum types in `ExchangeOrderNotification`.
pub mod exchange_order_notification {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum NotifyType {
        Status = 1,
        Modify = 2,
        Cancel = 3,
        Trigger = 4,
        Fill = 5,
        Reject = 6,
        NotModified = 7,
        NotCancelled = 8,
        Generic = 9,
    }
    impl NotifyType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Status => "STATUS",
                Self::Modify => "MODIFY",
                Self::Cancel => "CANCEL",
                Self::Trigger => "TRIGGER",
                Self::Fill => "FILL",
                Self::Reject => "REJECT",
                Self::NotModified => "NOT_MODIFIED",
                Self::NotCancelled => "NOT_CANCELLED",
                Self::Generic => "GENERIC",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATUS" => Some(Self::Status),
                "MODIFY" => Some(Self::Modify),
                "CANCEL" => Some(Self::Cancel),
                "TRIGGER" => Some(Self::Trigger),
                "FILL" => Some(Self::Fill),
                "REJECT" => Some(Self::Reject),
                "NOT_MODIFIED" => Some(Self::NotModified),
                "NOT_CANCELLED" => Some(Self::NotCancelled),
                "GENERIC" => Some(Self::Generic),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum TransactionType {
        Buy = 1,
        Sell = 2,
        Ss = 3,
    }
    impl TransactionType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Buy => "BUY",
                Self::Sell => "SELL",
                Self::Ss => "SS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "BUY" => Some(Self::Buy),
                "SELL" => Some(Self::Sell),
                "SS" => Some(Self::Ss),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Duration {
        Day = 1,
        Gtc = 2,
        Ioc = 3,
        Fok = 4,
    }
    impl Duration {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Day => "DAY",
                Self::Gtc => "GTC",
                Self::Ioc => "IOC",
                Self::Fok => "FOK",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DAY" => Some(Self::Day),
                "GTC" => Some(Self::Gtc),
                "IOC" => Some(Self::Ioc),
                "FOK" => Some(Self::Fok),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PriceType {
        Limit = 1,
        Market = 2,
        StopLimit = 3,
        StopMarket = 4,
    }
    impl PriceType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Limit => "LIMIT",
                Self::Market => "MARKET",
                Self::StopLimit => "STOP_LIMIT",
                Self::StopMarket => "STOP_MARKET",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "LIMIT" => Some(Self::Limit),
                "MARKET" => Some(Self::Market),
                "STOP_LIMIT" => Some(Self::StopLimit),
                "STOP_MARKET" => Some(Self::StopMarket),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum BracketType {
        StopOnly = 1,
        TargetOnly = 2,
        TargetAndStop = 3,
        StopOnlyStatic = 4,
        TargetOnlyStatic = 5,
        TargetAndStopStatic = 6,
    }
    impl BracketType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::StopOnly => "STOP_ONLY",
                Self::TargetOnly => "TARGET_ONLY",
                Self::TargetAndStop => "TARGET_AND_STOP",
                Self::StopOnlyStatic => "STOP_ONLY_STATIC",
                Self::TargetOnlyStatic => "TARGET_ONLY_STATIC",
                Self::TargetAndStopStatic => "TARGET_AND_STOP_STATIC",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STOP_ONLY" => Some(Self::StopOnly),
                "TARGET_ONLY" => Some(Self::TargetOnly),
                "TARGET_AND_STOP" => Some(Self::TargetAndStop),
                "STOP_ONLY_STATIC" => Some(Self::StopOnlyStatic),
                "TARGET_ONLY_STATIC" => Some(Self::TargetOnlyStatic),
                "TARGET_AND_STOP_STATIC" => Some(Self::TargetAndStopStatic),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum OrderPlacement {
        Manual = 1,
        Auto = 2,
    }
    impl OrderPlacement {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Manual => "MANUAL",
                Self::Auto => "AUTO",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MANUAL" => Some(Self::Manual),
                "AUTO" => Some(Self::Auto),
                _ => None,
            }
        }
    }
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountListUpdates {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_FCM_ID
    #[prost(string, optional, tag = "154013")]
    pub fcm_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_IB_ID
    #[prost(string, optional, tag = "154014")]
    pub ib_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_ACCOUNT_ID
    #[prost(string, optional, tag = "154008")]
    pub account_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_AUTO_LIQ_THRESHOLD_CURRENT_VALUE
    #[prost(string, optional, tag = "131040")]
    pub auto_liq_threshold_current_value: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateEasyToBorrowList {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_BROKER_DEALER
    #[prost(string, optional, tag = "154612")]
    pub broker_dealer: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_SYMBOL
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_SYMBOL_NAME
    #[prost(string, optional, tag = "100003")]
    pub symbol_name: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_TOTAL_AVAILABLE_QTY
    #[prost(int32, optional, tag = "154613")]
    pub qty_available: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_TOTAL_USED_QTY
    #[prost(int32, optional, tag = "154614")]
    pub qty_needed: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_SHORT_LIST_INDICATOR
    #[prost(bool, optional, tag = "110353")]
    pub borrowable: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountRmsUpdates {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(int32, optional, tag = "154211")]
    pub update_bits: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "154013")]
    pub fcm_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154014")]
    pub ib_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154008")]
    pub account_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "131040")]
    pub auto_liq_threshold_current_value: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "131049")]
    pub auto_liq_peak_account_balance: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "131050")]
    pub auto_liq_peak_account_balance_ssboe: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
}
/// Nested message and enum types in `AccountRmsUpdates`.
pub mod account_rms_updates {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum UpdateBits {
        AutoLiqThresholdCurrentValue = 1,
    }
    impl UpdateBits {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::AutoLiqThresholdCurrentValue => "AUTO_LIQ_THRESHOLD_CURRENT_VALUE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "AUTO_LIQ_THRESHOLD_CURRENT_VALUE" => {
                    Some(Self::AutoLiqThresholdCurrentValue)
                }
                _ => None,
            }
        }
    }
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestPnLPositionUpdates {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_REQUEST
    #[prost(
        enumeration = "request_pn_l_position_updates::Request",
        optional,
        tag = "100000"
    )]
    pub request: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_FCM_ID
    #[prost(string, optional, tag = "154013")]
    pub fcm_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_IB_ID
    #[prost(string, optional, tag = "154014")]
    pub ib_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_ACCOUNT_ID
    #[prost(string, optional, tag = "154008")]
    pub account_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `RequestPnLPositionUpdates`.
pub mod request_pn_l_position_updates {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Request {
        Subscribe = 1,
        Unsubscribe = 2,
    }
    impl Request {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Subscribe => "SUBSCRIBE",
                Self::Unsubscribe => "UNSUBSCRIBE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SUBSCRIBE" => Some(Self::Subscribe),
                "UNSUBSCRIBE" => Some(Self::Unsubscribe),
                _ => None,
            }
        }
    }
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponsePnLPositionUpdates {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_RESPONSE_CODE
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestPnLPositionSnapshot {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_FCM_ID
    #[prost(string, optional, tag = "154013")]
    pub fcm_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_IB_ID
    #[prost(string, optional, tag = "154014")]
    pub ib_id: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_ACCOUNT_ID
    #[prost(string, optional, tag = "154008")]
    pub account_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponsePnLPositionSnapshot {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_RESPONSE_CODE
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountPnLPositionUpdate {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(bool, optional, tag = "110121")]
    pub is_snapshot: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "154013")]
    pub fcm_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154014")]
    pub ib_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154008")]
    pub account_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "154041")]
    pub fill_buy_qty: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "154042")]
    pub fill_sell_qty: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "154037")]
    pub order_buy_qty: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "154038")]
    pub order_sell_qty: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "154260")]
    pub buy_qty: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "154261")]
    pub sell_qty: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "157105")]
    pub open_long_options_value: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "157106")]
    pub open_short_options_value: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "157107")]
    pub closed_options_value: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "157111")]
    pub option_cash_reserved: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "157113")]
    pub rms_account_commission: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "156961")]
    pub open_position_pnl: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "156962")]
    pub open_position_quantity: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "156963")]
    pub closed_position_pnl: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "156964")]
    pub closed_position_quantity: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "156967")]
    pub net_quantity: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "156991")]
    pub excess_buy_margin: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "156977")]
    pub margin_balance: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "156976")]
    pub min_margin_balance: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "156968")]
    pub min_account_balance: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "156970")]
    pub account_balance: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "156971")]
    pub cash_on_hand: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "157118")]
    pub option_closed_pnl: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "156965")]
    pub percent_maximum_allowable_loss: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "157117")]
    pub option_open_pnl: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154262")]
    pub mtm_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "157015")]
    pub available_buying_power: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "157014")]
    pub used_buying_power: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "157013")]
    pub reserved_buying_power: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "156992")]
    pub excess_sell_margin: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "157954")]
    pub day_open_pnl: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "157955")]
    pub day_closed_pnl: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "157956")]
    pub day_pnl: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "157957")]
    pub day_open_pnl_offset: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "157958")]
    pub day_closed_pnl_offset: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "150100")]
    pub ssboe: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "150101")]
    pub usecs: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstrumentPnLPositionUpdate {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(bool, optional, tag = "110121")]
    pub is_snapshot: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "154013")]
    pub fcm_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154014")]
    pub ib_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154008")]
    pub account_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "100749")]
    pub product_code: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110116")]
    pub instrument_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "154041")]
    pub fill_buy_qty: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "154042")]
    pub fill_sell_qty: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "154037")]
    pub order_buy_qty: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "154038")]
    pub order_sell_qty: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "154260")]
    pub buy_qty: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "154261")]
    pub sell_qty: ::core::option::Option<i32>,
    #[prost(double, optional, tag = "154434")]
    pub avg_open_fill_price: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "157954")]
    pub day_open_pnl: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "157955")]
    pub day_closed_pnl: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "157956")]
    pub day_pnl: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "157957")]
    pub day_open_pnl_offset: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "157958")]
    pub day_closed_pnl_offset: ::core::option::Option<f64>,
    #[prost(string, optional, tag = "154263")]
    pub mtm_security: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "157105")]
    pub open_long_options_value: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "157106")]
    pub open_short_options_value: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "157107")]
    pub closed_options_value: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "157111")]
    pub option_cash_reserved: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "156961")]
    pub open_position_pnl: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "156962")]
    pub open_position_quantity: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "156963")]
    pub closed_position_pnl: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "156964")]
    pub closed_position_quantity: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "156967")]
    pub net_quantity: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "150100")]
    pub ssboe: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "150101")]
    pub usecs: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestTickBarReplay {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration = "request_tick_bar_replay::BarType", optional, tag = "119200")]
    pub bar_type: ::core::option::Option<i32>,
    #[prost(
        enumeration = "request_tick_bar_replay::BarSubType",
        optional,
        tag = "119208"
    )]
    pub bar_sub_type: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "148162")]
    pub bar_type_specifier: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "153002")]
    pub start_index: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "153003")]
    pub finish_index: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "154020")]
    pub user_max_count: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "119209")]
    pub custom_session_open_ssm: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "119210")]
    pub custom_session_close_ssm: ::core::option::Option<i32>,
    #[prost(
        enumeration = "request_tick_bar_replay::Direction",
        optional,
        tag = "149253"
    )]
    pub direction: ::core::option::Option<i32>,
    #[prost(
        enumeration = "request_tick_bar_replay::TimeOrder",
        optional,
        tag = "149307"
    )]
    pub time_order: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "153642")]
    pub resume_bars: ::core::option::Option<bool>,
}
/// Nested message and enum types in `RequestTickBarReplay`.
pub mod request_tick_bar_replay {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum BarType {
        TickBar = 1,
        RangeBar = 2,
        VolumeBar = 3,
    }
    impl BarType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::TickBar => "TICK_BAR",
                Self::RangeBar => "RANGE_BAR",
                Self::VolumeBar => "VOLUME_BAR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TICK_BAR" => Some(Self::TickBar),
                "RANGE_BAR" => Some(Self::RangeBar),
                "VOLUME_BAR" => Some(Self::VolumeBar),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum BarSubType {
        Regular = 1,
        Custom = 2,
    }
    impl BarSubType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Regular => "REGULAR",
                Self::Custom => "CUSTOM",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "REGULAR" => Some(Self::Regular),
                "CUSTOM" => Some(Self::Custom),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Direction {
        First = 1,
        Last = 2,
    }
    impl Direction {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::First => "FIRST",
                Self::Last => "LAST",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FIRST" => Some(Self::First),
                "LAST" => Some(Self::Last),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum TimeOrder {
        Forwards = 1,
        Backwards = 2,
    }
    impl TimeOrder {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Forwards => "FORWARDS",
                Self::Backwards => "BACKWARDS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FORWARDS" => Some(Self::Forwards),
                "BACKWARDS" => Some(Self::Backwards),
                _ => None,
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseTickBarReplay {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, optional, tag = "132758")]
    pub request_key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "132764")]
    pub rq_handler_rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration = "response_tick_bar_replay::BarType", optional, tag = "119200")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(
        enumeration = "response_tick_bar_replay::BarSubType",
        optional,
        tag = "119208"
    )]
    pub sub_type: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "148162")]
    pub type_specifier: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "119204")]
    pub num_trades: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "119205")]
    pub volume: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "119213")]
    pub bid_volume: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "119214")]
    pub ask_volume: ::core::option::Option<u64>,
    #[prost(double, optional, tag = "100019")]
    pub open_price: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "100021")]
    pub close_price: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "100012")]
    pub high_price: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "100013")]
    pub low_price: ::core::option::Option<f64>,
    #[prost(int32, optional, tag = "119209")]
    pub custom_session_open_ssm: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "119202")]
    pub data_bar_ssboe: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "119203")]
    pub data_bar_usecs: ::prost::alloc::vec::Vec<i32>,
}
/// Nested message and enum types in `ResponseTickBarReplay`.
pub mod response_tick_bar_replay {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum BarType {
        TickBar = 1,
        RangeBar = 2,
        VolumeBar = 3,
    }
    impl BarType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::TickBar => "TICK_BAR",
                Self::RangeBar => "RANGE_BAR",
                Self::VolumeBar => "VOLUME_BAR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TICK_BAR" => Some(Self::TickBar),
                "RANGE_BAR" => Some(Self::RangeBar),
                "VOLUME_BAR" => Some(Self::VolumeBar),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum BarSubType {
        Regular = 1,
        Custom = 2,
    }
    impl BarSubType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Regular => "REGULAR",
                Self::Custom => "CUSTOM",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "REGULAR" => Some(Self::Regular),
                "CUSTOM" => Some(Self::Custom),
                _ => None,
            }
        }
    }
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestTickBarUpdate {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_SYMBOL
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_EXCHANGE
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_REQUEST
    #[prost(enumeration = "request_tick_bar_update::Request", optional, tag = "100000")]
    pub request: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_DATA_BAR_TYPE
    #[prost(enumeration = "request_tick_bar_update::BarType", optional, tag = "119200")]
    pub bar_type: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_DATA_BAR_SUB_TYPE
    #[prost(
        enumeration = "request_tick_bar_update::BarSubType",
        optional,
        tag = "119208"
    )]
    pub bar_sub_type: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_CATEGORY_SPECIFIC_INFO
    #[prost(string, optional, tag = "148162")]
    pub bar_type_specifier: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_CUSTOM_SESSION_OPEN_SSM
    #[prost(int32, optional, tag = "119209")]
    pub custom_session_open_ssm: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_CUSTOM_SESSION_CLOSE_SSM
    #[prost(int32, optional, tag = "119210")]
    pub custom_session_close_ssm: ::core::option::Option<i32>,
}
/// Nested message and enum types in `RequestTickBarUpdate`.
pub mod request_tick_bar_update {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum BarType {
        TickBar = 1,
        RangeBar = 2,
        VolumeBar = 3,
    }
    impl BarType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::TickBar => "TICK_BAR",
                Self::RangeBar => "RANGE_BAR",
                Self::VolumeBar => "VOLUME_BAR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TICK_BAR" => Some(Self::TickBar),
                "RANGE_BAR" => Some(Self::RangeBar),
                "VOLUME_BAR" => Some(Self::VolumeBar),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum BarSubType {
        Regular = 1,
        Custom = 2,
    }
    impl BarSubType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Regular => "REGULAR",
                Self::Custom => "CUSTOM",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "REGULAR" => Some(Self::Regular),
                "CUSTOM" => Some(Self::Custom),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Request {
        Subscribe = 1,
        Unsubscribe = 2,
    }
    impl Request {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Subscribe => "SUBSCRIBE",
                Self::Unsubscribe => "UNSUBSCRIBE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SUBSCRIBE" => Some(Self::Subscribe),
                "UNSUBSCRIBE" => Some(Self::Unsubscribe),
                _ => None,
            }
        }
    }
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseTickBarUpdate {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_RESPONSE_CODE
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestTimeBarReplay {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration = "request_time_bar_replay::BarType", optional, tag = "119200")]
    pub bar_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "119112")]
    pub bar_type_period: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "153002")]
    pub start_index: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "153003")]
    pub finish_index: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "154020")]
    pub user_max_count: ::core::option::Option<i32>,
    #[prost(
        enumeration = "request_time_bar_replay::Direction",
        optional,
        tag = "149253"
    )]
    pub direction: ::core::option::Option<i32>,
    #[prost(
        enumeration = "request_time_bar_replay::TimeOrder",
        optional,
        tag = "149307"
    )]
    pub time_order: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "153642")]
    pub resume_bars: ::core::option::Option<bool>,
}
/// Nested message and enum types in `RequestTimeBarReplay`.
pub mod request_time_bar_replay {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum BarType {
        SecondBar = 1,
        MinuteBar = 2,
        DailyBar = 3,
        WeeklyBar = 4,
    }
    impl BarType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::SecondBar => "SECOND_BAR",
                Self::MinuteBar => "MINUTE_BAR",
                Self::DailyBar => "DAILY_BAR",
                Self::WeeklyBar => "WEEKLY_BAR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SECOND_BAR" => Some(Self::SecondBar),
                "MINUTE_BAR" => Some(Self::MinuteBar),
                "DAILY_BAR" => Some(Self::DailyBar),
                "WEEKLY_BAR" => Some(Self::WeeklyBar),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Direction {
        First = 1,
        Last = 2,
    }
    impl Direction {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::First => "FIRST",
                Self::Last => "LAST",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FIRST" => Some(Self::First),
                "LAST" => Some(Self::Last),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum TimeOrder {
        Forwards = 1,
        Backwards = 2,
    }
    impl TimeOrder {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Forwards => "FORWARDS",
                Self::Backwards => "BACKWARDS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FORWARDS" => Some(Self::Forwards),
                "BACKWARDS" => Some(Self::Backwards),
                _ => None,
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseTimeBarReplay {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, optional, tag = "132758")]
    pub request_key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "132764")]
    pub rq_handler_rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration = "response_time_bar_replay::BarType", optional, tag = "119200")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "119112")]
    pub period: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "119100")]
    pub marker: ::core::option::Option<i32>,
    #[prost(uint64, optional, tag = "119109")]
    pub num_trades: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "119110")]
    pub volume: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "119117")]
    pub bid_volume: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "119118")]
    pub ask_volume: ::core::option::Option<u64>,
    #[prost(double, optional, tag = "100019")]
    pub open_price: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "100021")]
    pub close_price: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "100012")]
    pub high_price: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "100013")]
    pub low_price: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "100070")]
    pub settlement_price: ::core::option::Option<f64>,
    #[prost(bool, optional, tag = "149138")]
    pub has_settlement_price: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "154571")]
    pub must_clear_settlement_price: ::core::option::Option<bool>,
}
/// Nested message and enum types in `ResponseTimeBarReplay`.
pub mod response_time_bar_replay {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum BarType {
        SecondBar = 1,
        MinuteBar = 2,
        DailyBar = 3,
        WeeklyBar = 4,
    }
    impl BarType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::SecondBar => "SECOND_BAR",
                Self::MinuteBar => "MINUTE_BAR",
                Self::DailyBar => "DAILY_BAR",
                Self::WeeklyBar => "WEEKLY_BAR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SECOND_BAR" => Some(Self::SecondBar),
                "MINUTE_BAR" => Some(Self::MinuteBar),
                "DAILY_BAR" => Some(Self::DailyBar),
                "WEEKLY_BAR" => Some(Self::WeeklyBar),
                _ => None,
            }
        }
    }
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestTimeBarUpdate {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_SYMBOL
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_EXCHANGE
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_REQUEST
    #[prost(enumeration = "request_time_bar_update::Request", optional, tag = "100000")]
    pub request: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_DATA_BAR_TYPE
    #[prost(enumeration = "request_time_bar_update::BarType", optional, tag = "119200")]
    pub bar_type: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_TIME_BAR_PERIOD
    #[prost(int32, optional, tag = "119112")]
    pub bar_type_period: ::core::option::Option<i32>,
}
/// Nested message and enum types in `RequestTimeBarUpdate`.
pub mod request_time_bar_update {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum BarType {
        SecondBar = 1,
        MinuteBar = 2,
        DailyBar = 3,
        WeeklyBar = 4,
    }
    impl BarType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::SecondBar => "SECOND_BAR",
                Self::MinuteBar => "MINUTE_BAR",
                Self::DailyBar => "DAILY_BAR",
                Self::WeeklyBar => "WEEKLY_BAR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SECOND_BAR" => Some(Self::SecondBar),
                "MINUTE_BAR" => Some(Self::MinuteBar),
                "DAILY_BAR" => Some(Self::DailyBar),
                "WEEKLY_BAR" => Some(Self::WeeklyBar),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Request {
        Subscribe = 1,
        Unsubscribe = 2,
    }
    impl Request {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Subscribe => "SUBSCRIBE",
                Self::Unsubscribe => "UNSUBSCRIBE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SUBSCRIBE" => Some(Self::Subscribe),
                "UNSUBSCRIBE" => Some(Self::Unsubscribe),
                _ => None,
            }
        }
    }
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseTimeBarUpdate {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_USER_MSG
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_RESPONSE_CODE
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestVolumeProfileMinuteBars {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "119215")]
    pub bar_type_period: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "153002")]
    pub start_index: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "153003")]
    pub finish_index: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "154020")]
    pub user_max_count: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "153642")]
    pub resume_bars: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseVolumeProfileMinuteBars {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, optional, tag = "132758")]
    pub request_key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "132764")]
    pub rq_handler_rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "119215")]
    pub period: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "119100")]
    pub marker: ::core::option::Option<i32>,
    #[prost(uint64, optional, tag = "119204")]
    pub num_trades: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "119205")]
    pub volume: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "119213")]
    pub bid_volume: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "119214")]
    pub ask_volume: ::core::option::Option<u64>,
    #[prost(double, optional, tag = "100019")]
    pub open_price: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "100021")]
    pub close_price: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "100012")]
    pub high_price: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "100013")]
    pub low_price: ::core::option::Option<f64>,
    #[prost(double, repeated, packed = "false", tag = "119216")]
    pub profile_price: ::prost::alloc::vec::Vec<f64>,
    #[prost(int32, repeated, packed = "false", tag = "119217")]
    pub profile_no_aggressor_volume: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "119218")]
    pub profile_bid_volume: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "119219")]
    pub profile_ask_volume: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "119220")]
    pub profile_no_aggressor_trades: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "119221")]
    pub profile_bid_aggressor_trades: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "119222")]
    pub profile_ask_aggressor_trades: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestResumeBars {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "132758")]
    pub request_key: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseResumeBars {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TickBar {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_SYMBOL
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_EXCHANGE
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_DATA_BAR_TYPE
    #[prost(enumeration = "tick_bar::BarType", optional, tag = "119200")]
    pub r#type: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_DATA_BAR_SUB_TYPE
    #[prost(enumeration = "tick_bar::BarSubType", optional, tag = "119208")]
    pub sub_type: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_CATEGORY_SPECIFIC_INFO
    #[prost(string, optional, tag = "148162")]
    pub type_specifier: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_DATA_BAR_NUM_TRADES
    #[prost(uint64, optional, tag = "119204")]
    pub num_trades: ::core::option::Option<u64>,
    /// PB_OFFSET + MNM_DATA_BAR_TRADE_VOLUME
    #[prost(uint64, optional, tag = "119205")]
    pub volume: ::core::option::Option<u64>,
    /// PB_OFFSET + MNM_DATA_BAR_BID_VOLUME
    #[prost(uint64, optional, tag = "119213")]
    pub bid_volume: ::core::option::Option<u64>,
    /// PB_OFFSET + MNM_DATA_BAR_ASK_VOLUME
    #[prost(uint64, optional, tag = "119214")]
    pub ask_volume: ::core::option::Option<u64>,
    /// PB_OFFSET + MNM_OPEN_PRICE
    #[prost(double, optional, tag = "100019")]
    pub open_price: ::core::option::Option<f64>,
    /// PB_OFFSET + MNM_CLOSE_TRADE_PRICE
    #[prost(double, optional, tag = "100021")]
    pub close_price: ::core::option::Option<f64>,
    /// PB_OFFSET + MNM_HIGH_PRICE
    #[prost(double, optional, tag = "100012")]
    pub high_price: ::core::option::Option<f64>,
    /// PB_OFFSET + MNM_LOW_PRICE
    #[prost(double, optional, tag = "100013")]
    pub low_price: ::core::option::Option<f64>,
    /// PB_OFFSET + MNM_CUSTOM_SESSION_OPEN_SSM
    #[prost(int32, optional, tag = "119209")]
    pub custom_session_open_ssm: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_DATA_BAR_SSBOE
    #[prost(int32, repeated, packed = "false", tag = "119202")]
    pub data_bar_ssboe: ::prost::alloc::vec::Vec<i32>,
    /// PB_OFFSTE + MNM_DATA_BAR_USECS
    #[prost(int32, repeated, packed = "false", tag = "119203")]
    pub data_bar_usecs: ::prost::alloc::vec::Vec<i32>,
}
/// Nested message and enum types in `TickBar`.
pub mod tick_bar {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum BarType {
        TickBar = 1,
        RangeBar = 2,
        VolumeBar = 3,
    }
    impl BarType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::TickBar => "TICK_BAR",
                Self::RangeBar => "RANGE_BAR",
                Self::VolumeBar => "VOLUME_BAR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TICK_BAR" => Some(Self::TickBar),
                "RANGE_BAR" => Some(Self::RangeBar),
                "VOLUME_BAR" => Some(Self::VolumeBar),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum BarSubType {
        Regular = 1,
        Custom = 2,
    }
    impl BarSubType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Regular => "REGULAR",
                Self::Custom => "CUSTOM",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "REGULAR" => Some(Self::Regular),
                "CUSTOM" => Some(Self::Custom),
                _ => None,
            }
        }
    }
}
/// PB_OFFSET = 100000, is the offset added for each MNM field id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeBar {
    /// PB_OFFSET + MNM_TEMPLATE_ID
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    /// PB_OFFSET + MNM_SYMBOL
    #[prost(string, optional, tag = "110100")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_EXCHANGE
    #[prost(string, optional, tag = "110101")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_DATA_BAR_TYPE
    #[prost(enumeration = "time_bar::BarType", optional, tag = "119200")]
    pub r#type: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_TIME_BAR_PERIOD
    #[prost(string, optional, tag = "119112")]
    pub period: ::core::option::Option<::prost::alloc::string::String>,
    /// PB_OFFSET + MNM_TIME_BAR_MARKER
    #[prost(int32, optional, tag = "119100")]
    pub marker: ::core::option::Option<i32>,
    /// PB_OFFSET + MNM_TIME_BAR_NUM_TRADES
    #[prost(uint64, optional, tag = "119109")]
    pub num_trades: ::core::option::Option<u64>,
    /// PB_OFFSET + MNM_TIME_BAR_TRADE_VOLUME
    #[prost(uint64, optional, tag = "119110")]
    pub volume: ::core::option::Option<u64>,
    /// PB_OFFSET + MNM_TIME_BAR_BID_VOLUME
    #[prost(uint64, optional, tag = "119117")]
    pub bid_volume: ::core::option::Option<u64>,
    /// PB_OFFSET + MNM_TIME_BAR_ASK_VOLUME
    #[prost(uint64, optional, tag = "119118")]
    pub ask_volume: ::core::option::Option<u64>,
    /// PB_OFFSET + MNM_OPEN_PRICE
    #[prost(double, optional, tag = "100019")]
    pub open_price: ::core::option::Option<f64>,
    /// PB_OFFSET + MNM_CLOSE_TRADE_PRICE
    #[prost(double, optional, tag = "100021")]
    pub close_price: ::core::option::Option<f64>,
    /// PB_OFFSET + MNM_HIGH_PRICE
    #[prost(double, optional, tag = "100012")]
    pub high_price: ::core::option::Option<f64>,
    /// PB_OFFSET + MNM_LOW_PRICE
    #[prost(double, optional, tag = "100013")]
    pub low_price: ::core::option::Option<f64>,
    /// PB_OFFSET + MNM_SETTLEMENT_PRICE
    #[prost(double, optional, tag = "100070")]
    pub settlement_price: ::core::option::Option<f64>,
    /// PB_OFFSET + MNM_PRICING_INDICATOR
    #[prost(bool, optional, tag = "149138")]
    pub has_settlement_price: ::core::option::Option<bool>,
    /// PB_OFFSET + MNM_DISPLAY_INDICATOR
    #[prost(bool, optional, tag = "154571")]
    pub must_clear_settlement_price: ::core::option::Option<bool>,
}
/// Nested message and enum types in `TimeBar`.
pub mod time_bar {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum BarType {
        SecondBar = 1,
        MinuteBar = 2,
        DailyBar = 3,
        WeeklyBar = 4,
    }
    impl BarType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::SecondBar => "SECOND_BAR",
                Self::MinuteBar => "MINUTE_BAR",
                Self::DailyBar => "DAILY_BAR",
                Self::WeeklyBar => "WEEKLY_BAR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SECOND_BAR" => Some(Self::SecondBar),
                "MINUTE_BAR" => Some(Self::MinuteBar),
                "DAILY_BAR" => Some(Self::DailyBar),
                "WEEKLY_BAR" => Some(Self::WeeklyBar),
                _ => None,
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestListUnacceptedAgreements {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseListUnacceptedAgreements {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "132764")]
    pub rq_handler_rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154013")]
    pub fcm_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154014")]
    pub ib_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "153406")]
    pub agreement_title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "153407")]
    pub agreement_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "153430")]
    pub agreement_acceptance_request: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestListAcceptedAgreements {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseListAcceptedAgreements {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "132764")]
    pub rq_handler_rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154013")]
    pub fcm_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "154014")]
    pub ib_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "153427")]
    pub agreement_acceptance_ssboe: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "153426")]
    pub agreement_acceptance_status: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "153430")]
    pub agreement_acceptance_request: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "153406")]
    pub agreement_title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "153407")]
    pub agreement_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestAcceptAgreement {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "153407")]
    pub agreement_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Professional or Non-Professional
    #[prost(string, optional, tag = "153431")]
    pub market_data_usage_capacity: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseAcceptAgreement {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestSetRithmicMrktDataSelfCertStatus {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "153407")]
    pub agreement_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "153431")]
    pub market_data_usage_capacity: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseSetRithmicMrktDataSelfCertStatus {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestShowAgreement {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "153407")]
    pub agreement_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseShowAgreement {
    #[prost(int32, required, tag = "154467")]
    pub template_id: i32,
    #[prost(string, repeated, tag = "132760")]
    pub user_msg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "132764")]
    pub rq_handler_rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "132766")]
    pub rp_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "153406")]
    pub agreement_title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "153407")]
    pub agreement_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "153405")]
    pub agreement: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", optional, tag = "153432")]
    pub agreement_html: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag = "153410")]
    pub agreement_mandatory_flag: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "153415")]
    pub agreement_status: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "153430")]
    pub agreement_acceptance_request: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
}
