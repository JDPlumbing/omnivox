# Time API

This module exposes **read-only, deterministic time utilities** over HTTP.

All endpoints are **engine-backed** (`TimeEngine`) and do **not** depend on databases, Supabase, or wall-clock state beyond initial seeding.

---

## Design Principles

* **Deterministic core**: Time is represented as `i128` nanoseconds since the Unix epoch.
* **Engine-driven**: API handlers call `TimeEngine`; they do not call core behavior directly.
* **Pure queries**: No mutation, persistence, or side effects.
* **JSON-safe**: Large integers are serialized as strings.

---

## Types

### `SimTime`

* Absolute simulation time
* Monotonic nanoseconds (`i128`) since Unix epoch
* Negative values represent times before epoch

### `SimDuration`

* Signed duration in nanoseconds
* Supports human-readable formatting

### `TimeFormat`

Enum selecting output format:

* `RawNs`
* `Rfc3339`
* `SimDate`
* `SimMonth`
* `SimWeek`
* `SimDay`

---

## Endpoints

### `GET /api/time/simtime/now`

Returns the current simulation time.

**Response**

```json
{
  "simtime_ns": "1700000000000000000",
  "datetime": "2023-11-14T00:00:00.000Z"
}
```

---

### `GET /api/time/format?ns=<string>&fmt=<TimeFormat>`

Formats a simulation timestamp.

**Query**

* `ns`: nanoseconds as string
* `fmt`: format enum

**Response**

```json
{
  "formatted": "2024-01-01",
  "format": "SimDate",
  "ns": "1704067200000000000"
}
```

---

### `POST /api/time/delta`

Computes duration between two timestamps.

**Body**

```json
{
  "start_ns": "1700000000000000000",
  "end_ns": "1700003600000000000"
}
```

**Response**

```json
{
  "delta_ns": "3600000000000",
  "human": "1.000 h"
}
```

---

### `GET /api/time/duration/human?ns=<string>`

Formats a duration into a human-readable string.

**Response**

```json
{
  "human": "2.500 hours",
  "ns": "9000000000000"
}
```

---

### `GET /api/time/simdate/from_ns?ns=<string>`

Converts a timestamp to deterministic simulation date.

**Response**

```json
{
  "simdate": "2024-01-01",
  "ns": "1704067200000000000"
}
```

---

### `GET /api/time/julian/from_ns?ns=<string>`

Returns Julian Day Number and Julian Date.

**Response**

```json
{
  "julian_date": [2460309.5, 2460309.7347222223],
  "ns": "1704067200000000000"
}
```

---

## Notes

* All endpoints are **stateless**.
* No authentication required.
* Time scaling, pausing, or time travel (future) will be implemented in `TimeEngine` without changing this API.

---

## Status

Stable. Engine-backed. Architecture-compliant.
