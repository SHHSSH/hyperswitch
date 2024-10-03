CREATE TABLE refund_queue (
    `internal_reference_id` String,
    `refund_id` String,
    `payment_id` String,
    `merchant_id` String,
    `connector_transaction_id` String,
    `connector` LowCardinality(Nullable(String)),
    `connector_refund_id` Nullable(String),
    `external_reference_id` Nullable(String),
    `refund_type` LowCardinality(String),
    `total_amount` Nullable(UInt32),
    `currency` LowCardinality(String),
    `refund_amount` Nullable(UInt32),
    `refund_status` LowCardinality(String),
    `sent_to_gateway` Bool,
    `refund_error_message` Nullable(String),
    `refund_arn` Nullable(String),
    `attempt_id` String,
    `description` Nullable(String),
    `refund_reason` Nullable(String),
    `refund_error_code` Nullable(String),
    `created_at` DateTime,
    `modified_at` DateTime,
    `organization_id` String,
    `profile_id` String,
    `sign_flag` Int8
) ENGINE = Kafka SETTINGS kafka_broker_list = 'kafka0:29092',
kafka_topic_list = 'hyperswitch-refund-events',
kafka_group_name = 'hyper',
kafka_format = 'JSONEachRow',
kafka_handle_error_mode = 'stream';

CREATE TABLE refunds (
    `internal_reference_id` String,
    `refund_id` String,
    `payment_id` String,
    `merchant_id` LowCardinality(String),
    `connector_transaction_id` String,
    `connector` LowCardinality(Nullable(String)),
    `connector_refund_id` Nullable(String),
    `external_reference_id` Nullable(String),
    `refund_type` LowCardinality(String),
    `total_amount` Nullable(UInt32),
    `currency` LowCardinality(String),
    `refund_amount` Nullable(UInt32),
    `refund_status` LowCardinality(String),
    `sent_to_gateway` Bool,
    `refund_error_message` Nullable(String),
    `refund_arn` Nullable(String),
    `attempt_id` String,
    `description` Nullable(String),
    `refund_reason` Nullable(String),
    `refund_error_code` Nullable(String),
    `created_at` DateTime DEFAULT now() CODEC(T64, LZ4),
    `modified_at` DateTime DEFAULT now() CODEC(T64, LZ4),
    `inserted_at` DateTime DEFAULT now() CODEC(T64, LZ4),
    `organization_id` String,
    `profile_id` String,
    `sign_flag` Int8,
    INDEX connectorIndex connector TYPE bloom_filter GRANULARITY 1,
    INDEX refundTypeIndex refund_type TYPE bloom_filter GRANULARITY 1,
    INDEX currencyIndex currency TYPE bloom_filter GRANULARITY 1,
    INDEX statusIndex refund_status TYPE bloom_filter GRANULARITY 1
) ENGINE = CollapsingMergeTree(sign_flag) PARTITION BY toStartOfDay(created_at)
ORDER BY
    (created_at, merchant_id, refund_id) TTL created_at + toIntervalMonth(18) SETTINGS index_granularity = 8192;

CREATE MATERIALIZED VIEW refund_mv TO refunds (
    `internal_reference_id` String,
    `refund_id` String,
    `payment_id` String,
    `merchant_id` String,
    `connector_transaction_id` String,
    `connector` LowCardinality(Nullable(String)),
    `connector_refund_id` Nullable(String),
    `external_reference_id` Nullable(String),
    `refund_type` LowCardinality(String),
    `total_amount` Nullable(UInt32),
    `currency` LowCardinality(String),
    `refund_amount` Nullable(UInt32),
    `refund_status` LowCardinality(String),
    `sent_to_gateway` Bool,
    `refund_error_message` Nullable(String),
    `refund_arn` Nullable(String),
    `attempt_id` String,
    `description` Nullable(String),
    `refund_reason` Nullable(String),
    `refund_error_code` Nullable(String),
    `created_at` DateTime64(3),
    `modified_at` DateTime64(3),
    `inserted_at` DateTime64(3),
    `organization_id` String,
    `profile_id` String,
    `sign_flag` Int8
) AS
SELECT
    internal_reference_id,
    refund_id,
    payment_id,
    merchant_id,
    connector_transaction_id,
    connector,
    connector_refund_id,
    external_reference_id,
    refund_type,
    total_amount,
    currency,
    refund_amount,
    refund_status,
    sent_to_gateway,
    refund_error_message,
    refund_arn,
    attempt_id,
    description,
    refund_reason,
    refund_error_code,
    created_at,
    modified_at,
    now() AS inserted_at,
    organization_id,
    profile_id,
    sign_flag
FROM
    refund_queue
WHERE
    length(_error) = 0;