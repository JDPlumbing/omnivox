┌──────────────┐
│    app/      │  ← composition, config, startup
└──────┬───────┘
       │
┌──────▼───────┐
│   shared/    │  ← interfaces (Source / Context)
└──────┬───────┘
       │
┌──────▼───────┐
│   engine/    │  ← orchestration, rules
└──────┬───────┘
       │
┌──────▼───────┐
│    core/     │  ← truth, ECS, domain
└──────────────┘

infra/ sits off to the side, plugged in by app/
supabasic is a low-level client used only by infra
