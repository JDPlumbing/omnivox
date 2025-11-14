# Solar Simulation Architecture

## 1. Overview
This document describes how Omnivox simulates sunlight, solar motion, irradiance, UV exposure, and UV‑driven material degradation. The Sun is represented as a normal Objex with additional components that drive its physical behavior and its effects on other objects in the world.

The solar pipeline flows like this:

```
Sun (Objex)
   │
   ▼
SunEmitter (component)
   │
   ▼
SolarMotionSystem → updates UvoxID (lat/lon)
   │
   ▼
SolarRaycastSystem → computes irradiance, writes SunlightComponent
   │
   ▼
SolarExposureSystem → accumulates energy & UV (SolarExposureData)
   │
   ▼
UVDegradationSystem → computes degradation severity
```

---

## 2. Components

### **SunEmitter**
Attached only to the Sun Objex.  
Defines the physical behavior of the star.

**Fields:**
- `luminosity_w` — Watt output of the star  
- `uv_fraction` — fraction of emitted energy that is ultraviolet  
- `orbital_motion` — struct controlling seasonal tilt + daily orbit  
  - `lon_step` — longitudinal motion per simulation tick  
  - `lat_amp` — maximum latitude swing for seasonal variation  
  - `tilt_dir` — ±1 for seasonal tilt direction  
  - `orbit_dir` — ±1 for daily rotation direction  

---

### **SunlightComponent**
Attached to any object that is currently lit by the sun.

**Fields:**
- `irradiance_w_m2` — sunlight intensity reaching the object  
- `uv_index` — UV index equivalent derived from emitter UV fraction  

This component exists only while the object is lit. It is removed automatically at night.

---

### **SolarExposureData**
Persistent accumulator for each object.

**Fields:**
- `energy_j_m2` — total received energy  
- `uv_j_m2` — total received UV energy  

Values accumulate over time based on `SunlightComponent`.

---

### **UVDegradationData**
Tracks long‑term UV material damage.

**Fields:**
- `cumulative_uv_j_m2` — total lifetime UV dose  
- `severity` — degradation severity (0 → 1)  
- `rate_m_per_year` — potential physical weakening (future use)  

---

## 3. Systems

### **SolarMotionSystem**
Updates the Sun’s UvoxID coordinates.

- Applies daily rotation via `lon_step`  
- Applies seasonal oscillation via `lat_amp`  
- Clamps latitude to realistic tilt bounds  
- Emits a `SolarPositionUpdate` ChronoEvent

This uses Earth-centered coordinates (frame_id = Earth), meaning the Sun “orbits” Earth because of the chosen frame of reference. Physics is still correct.

---

### **SolarRaycastSystem**
Computes which objects are lit.

Steps:
1. Converts sun & object lat/lon to radians  
2. Computes solar zenith angle  
3. If zenith < 90°, the object is lit  
4. Computes irradiance:

```
irradiance = luminosity_w / (4πr²)
```

5. Writes a `SunlightComponent` to lit objects  
6. Removes it from objects in darkness  
7. Emits `SolarRaycastUpdate` events

---

### **SolarExposureSystem**
Consumes `SunlightComponent` and accumulates exposure.

Adds per tick:

```
energy_j_m2 += irradiance * dt
uv_j_m2     += irradiance * uv_fraction * dt
```

Writes to each object’s `SolarExposureData`.

Emits `SolarExposureUpdate`.

---

### **UVDegradationSystem**
Takes cumulative UV dose and converts it into a degradation severity, scaled against the material’s UV resistance.

```
severity = min( dose / (resistance * scale_factor), 1.0 )
```

Emits:
- `UVDegradationProgress`  
- or `UVDegradationFailure` if severity == 1

---

## 4. Coordinate Model (UvoxID)

Each Objex location is expressed in:

- `frame_id` — which world reference frame the coordinates belong to  
- `r_um` — radial distance in micrometers  
- `lat_code`, `lon_code` — scaled integers at 1e‑11 resolution

Earth is the center frame for the main simulation.  
The Sun appears to orbit Earth because of this frame selection.  
Switching frame_id enables alternate perspectives (Heliocentric, selenocentric, etc.)

---

## 5. Units & Physics Notes
- `W/m²` for irradiance  
- `J/m²` for energy accumulation  
- Solar constant at Earth = **1361 W/m²**  
- UV fraction ~3–4% of total solar output  
- Plasma (photosphere) is modeled as a special MatCat category  

---

## 6. Future Extensions
- Atmospheric attenuation & scattering  
- Horizon / terrain-based occlusion  
- Shadows & ray casting against geometry  
- Weather (clouds reduce irradiance)  
- Diffuse lighting  
- Moon albedo lighting  

---

Last updated: generated automatically by ChatGPT.
