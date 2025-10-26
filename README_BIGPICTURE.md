# Omnivox — Big Picture

## Purpose
This repository (omnivox) is a Rust backend simulation framework that integrates modules for each major aspect of a realistic digital-twin simulation.

## Big-picture goal
Provide "GitHub for physical reality": users supply known values about an asset and the system constructs a digital twin that runs realistic systems over time within their spatial context to estimate failure rates, lifespans, locations, diffs (remodels, property mapping), and generate BOMs.

## Concept
- ECS-style simulation: entities, components, and systems.
- Primary difference from other simulated worlds: strive to closely mimic real-world behavior to be practically useful.
- Not a Cartesian-space simulation. Location/identity uses UvoxID rather than raw coordinates.

## UvoxID
- Compact identity format: four 64-bit integers (4 × u64).
- Designed to represent spatial/semantic identity across the simulation without relying on continuous Cartesian coordinates.

## Core features
- Time-based execution of realistic systems and interactions.
- Spatial context-aware degradation, failure prediction, and lifecycle estimation.
- Change diffs: remodels, layout changes, and BOM generation.
- Material-aware simulation influenced by environment (location, water quality, etc.).

## Target users & applications
- Homeowners
    - Living dashboard for cost of ownership and maintenance planning.
    - Estimate maintenance/replacement timing and cost from build date and component materials.
- Contractors
    - Run a property to get "things to watch out for", maintenance schedules, and repair vs. replace cost analysis.
    - Training: simulate symptoms, repairs, and safety scenarios.
- Property managers / real estate investors
    - Aggregate dashboards across multiple properties with the same predictive insights.
- Insurers
    - Improve underwriting by using real-time property data, materials, and construction history to calibrate rates.
- Innovators
    - Research and improve digital-twin techniques and AI spatial/material reasoning to reduce hallucination and capture trade conventions.

## Implementation notes
- Material degradation models driven by environmental factors and history.
- Focus on producing actionable outputs (alerts, schedules, BOMs, cost estimates) rather than purely visual simulations.
- Modular architecture to tailor UX for different user types and use-cases.

<!-- End of big-picture summary -->