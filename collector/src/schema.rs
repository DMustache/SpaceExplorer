// @generated automatically by Diesel CLI.

diesel::table! {
    application_outbox_events (id) {
        id -> Uuid,
        event_type -> Text,
        aggregate_type -> Text,
        aggregate_id -> Uuid,
        payload -> Jsonb,
        status -> Text,
        attempts -> Int4,
        created_at -> Timestamptz,
        processed_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    normalized_event_sources (normalized_event_id, raw_observation_id, role) {
        normalized_event_id -> Uuid,
        raw_observation_id -> Uuid,
        role -> Text,
    }
}

diesel::table! {
    normalized_events (id) {
        id -> Uuid,
        domain -> Text,
        event_type -> Text,
        title -> Text,
        summary -> Nullable<Text>,
        occurred_at -> Nullable<Timestamptz>,
        detected_at -> Nullable<Timestamptz>,
        normalized_at -> Timestamptz,
        severity -> Text,
        confidence -> Float8,
        attributes -> Jsonb,
        dedup_key -> Text,
        algorithm_version -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    raw_fetch_passes (id) {
        id -> Uuid,
        source_key -> Text,
        started_at -> Timestamptz,
        finished_at -> Nullable<Timestamptz>,
        status -> Text,
        requested_from -> Nullable<Timestamptz>,
        requested_to -> Nullable<Timestamptz>,
        items_fetched -> Int4,
        items_inserted -> Int4,
        items_duplicated -> Int4,
        error_message -> Nullable<Text>,
        metadata -> Jsonb,
    }
}

diesel::table! {
    raw_observations (id) {
        id -> Uuid,
        source_key -> Text,
        fetch_pass_id -> Uuid,
        source_record_id -> Text,
        source_event_type -> Text,
        observed_at -> Nullable<Timestamptz>,
        received_at -> Timestamptz,
        payload_hash -> Text,
        payload_json -> Jsonb,
        schema_version -> Nullable<Text>,
        content_version -> Int4,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    raw_sources (id) {
        id -> Uuid,
        key -> Text,
        provider -> Text,
        name -> Text,
        base_url -> Nullable<Text>,
        enabled -> Bool,
        created_at -> Timestamptz,
    }
}

diesel::joinable!(normalized_event_sources -> normalized_events (normalized_event_id));
diesel::joinable!(normalized_event_sources -> raw_observations (raw_observation_id));
diesel::joinable!(raw_observations -> raw_fetch_passes (fetch_pass_id));

diesel::allow_tables_to_appear_in_same_query!(
    application_outbox_events,
    normalized_event_sources,
    normalized_events,
    raw_fetch_passes,
    raw_observations,
    raw_sources,
);
