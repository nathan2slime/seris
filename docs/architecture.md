# Architecture

Seris follows a small, layered flow:

```mermaid
flowchart LR
  Discord[Discord Gateway] --> Handler[Serenity handler]
  Handler --> Plugins[Plugin registry]
  Plugins --> Commands[Slash commands]
  Commands --> Database[SQLite persistence]
  Commands --> Embeds[Embed builders]
  Commands --> Services[Service clients]
  Services --> Jikan[Jikan API]
  Services --> NASA[NASA APOD API]
  Handler --> Dashboard[TUI dashboard]
  Handler --> Health[Health state]
  Health --> HealthHTTP[Health server]
```

## Runtime pieces

* `src/main.rs` boots the bot, TUI dashboard, and health server.
* `src/utils.rs` maps Serenity events into readiness state.
* `src/plugins.rs` groups slash commands into internal plugins.
* `src/database.rs` stores persistent command usage in SQLite.
* `src/dashboard.rs` renders the terminal dashboard.
* `src/commands/` contains the slash command entry points.
* `src/services/` handles external API calls and retry/circuit behavior.
* `src/embeds/` turns service payloads into Discord messages.

## Request flow

1. Discord dispatches an interaction.
2. Serenity hands it to the event handler or slash command.
3. The plugin registry groups commands by capability.
4. Commands record usage in SQLite when appropriate.
5. Commands call a service client when data is needed.
6. The service client fetches or caches the payload.
7. An embed builder formats the response for Discord.
