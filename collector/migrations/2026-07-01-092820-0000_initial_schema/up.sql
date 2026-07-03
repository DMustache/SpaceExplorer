CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE raw_sources (
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    key TEXT NOT NULL UNIQUE,
    provider TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    base_url TEXT,
    enabled BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE raw_fetch_passes (
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    source_key TEXT NOT NULL REFERENCES raw_sources(key),
    started_at TIMESTAMPTZ NOT NULL,
    finished_at TIMESTAMPTZ,
    status TEXT NOT NULL,
    requested_from TIMESTAMPTZ,
    requested_to TIMESTAMPTZ,
    items_fetched INT NOT NULL DEFAULT 0,
    items_inserted INT NOT NULL DEFAULT 0,
    items_duplicated INT NOT NULL DEFAULT 0,
    error_message TEXT,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb
);

CREATE TABLE raw_observations (
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),

    source_key TEXT NOT NULL REFERENCES raw_sources(key),
    fetch_pass_id UUID NOT NULL REFERENCES raw_fetch_passes(id),

    source_record_id TEXT NOT NULL,
    source_event_type TEXT NOT NULL,

    observed_at TIMESTAMPTZ,
    received_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    payload_hash TEXT NOT NULL,
    payload_json JSONB NOT NULL,

    schema_version TEXT,
    content_version INT NOT NULL DEFAULT 1,

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    UNIQUE (source_key, source_record_id, payload_hash)
);

CREATE INDEX idx_raw_observations_source_received
    ON raw_observations (source_key, received_at DESC);

CREATE INDEX idx_raw_observations_type_observed
    ON raw_observations (source_event_type, observed_at DESC);

CREATE INDEX idx_raw_observations_payload_gin
    ON raw_observations USING GIN (payload_json);


CREATE TABLE normalized_events (
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),

    domain TEXT NOT NULL,
    event_type TEXT NOT NULL,

    title TEXT NOT NULL,
    summary TEXT,

    occurred_at TIMESTAMPTZ,
    detected_at TIMESTAMPTZ,
    normalized_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    severity TEXT NOT NULL DEFAULT 'info',
    confidence DOUBLE PRECISION NOT NULL DEFAULT 1.0,

    attributes JSONB NOT NULL DEFAULT '{}'::jsonb,

    dedup_key TEXT NOT NULL,
    algorithm_version TEXT NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    UNIQUE (dedup_key, algorithm_version)
);

CREATE INDEX idx_norm_events_domain_time
    ON normalized_events (domain, occurred_at DESC);

CREATE INDEX idx_norm_events_type_time
    ON normalized_events (event_type, occurred_at DESC);

CREATE INDEX idx_norm_events_attributes_gin
    ON normalized_events USING GIN (attributes);

CREATE TABLE normalized_event_sources (
    normalized_event_id UUID NOT NULL REFERENCES normalized_events(id),
    raw_observation_id UUID NOT NULL REFERENCES raw_observations(id),
    role TEXT NOT NULL,

    PRIMARY KEY (normalized_event_id, raw_observation_id, role)
);

CREATE TABLE application_outbox_events (
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    event_type TEXT NOT NULL,
    aggregate_type TEXT NOT NULL,
    aggregate_id UUID NOT NULL,

    payload JSONB NOT NULL,

    status TEXT NOT NULL DEFAULT 'pending',
    attempts INT NOT NULL DEFAULT 0,

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    processed_at TIMESTAMPTZ
);

CREATE INDEX idx_outbox_pending
    ON application_outbox_events (status, created_at);
