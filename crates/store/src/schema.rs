// @generated automatically by Diesel CLI.

diesel::table! {
    indicators (name, timestamp) {
        #[max_length = 128]
        name -> Varchar,
        #[max_length = 64]
        category -> Varchar,
        timestamp -> Date,
        value -> Nullable<Float8>,
        #[max_length = 64]
        source -> Nullable<Varchar>,
    }
}

diesel::table! {
    market_data (asset_symbol, timestamp) {
        #[max_length = 16]
        asset_symbol -> Varchar,
        timestamp -> Date,
        price_usd -> Float8,
        volume_usd -> Nullable<Float8>,
        market_cap_usd -> Nullable<Float8>,
        dominance -> Nullable<Float8>,
    }
}

diesel::table! {
    market_metrics (name, timestamp) {
        #[max_length = 128]
        name -> Varchar,
        timestamp -> Date,
        value -> Nullable<Float8>,
        #[max_length = 64]
        source -> Nullable<Varchar>,
    }
}

diesel::table! {
    strategy_signals (id) {
        id -> Uuid,
        #[max_length = 16]
        asset_symbol -> Varchar,
        timestamp -> Timestamptz,
        #[max_length = 64]
        signal_type -> Varchar,
        value -> Nullable<Float8>,
        description -> Nullable<Text>,
        #[max_length = 64]
        source -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    indicators,
    market_data,
    market_metrics,
    strategy_signals,
);
