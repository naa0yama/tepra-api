# Askama Templates

HTML templates live in `crates/tepra/templates/` and are compiled at build time
by the [Askama](https://djc.github.io/askama/) template engine.

## Directory Structure

```
crates/tepra/templates/
  shells/
    dashboard.html      # L1 shell (base layout)
  pages/
    index.html          # Printer list page (GET /ui/)
    printer_detail.html # Per-printer detail page (GET /ui/printers/{name})
  partials/
    job_card.html       # HTMX job-status polling card (GET /ui/jobs/{printer}/{id})
  components/
    alert.html          # Reusable alert macros
```

## Template Roles

### shells/dashboard.html

Base layout used by all page templates via `{% extends %}`:

- Loads `/static/app.css` (Tailwind 4 + DaisyUI 5 bundle served by `tepra-web`)
- Loads `/static/htmx.min.js` (deferred, no CDN)
- DaisyUI theme: `data-theme="corporate"` (light default)
- Accessibility: skip-to-content link, `<main id="main" tabindex="-1">`
- Responsive drawer nav (collapses to hamburger on mobile)
- Toast container: `#toast-container` (DaisyUI toast, `aria-live="polite"`)
- Exposes `{% block title %}` and `{% block body %}` blocks

### pages/index.html

Extends `shells/dashboard.html`. Bound to `IndexTemplate` in `views.rs`.

- Shows a DaisyUI menu list of known printer names
- Renders `components::error_alert` when `error: Option<String>` is set
- Empty-state hero when `printers` is empty

### pages/printer_detail.html

Extends `shells/dashboard.html`. Bound to `PrinterDetailTemplate` in `views.rs`.

- Shows per-printer metadata and job history
- Each job is rendered as a `job_card.html` partial via HTMX out-of-band swap

### partials/job_card.html

Standalone partial, not extending any shell. Bound to `JobCardTemplate`.

- `<div id="job-{job_id}">` — HTMX target for OOB swaps
- Polls `GET /ui/jobs/{printer}/{job_id}` every 1 s while job is in-flight
- Stops polling when `job_end=true` or `canceled=true` (removes `hx-trigger`)
- States: waiting (no progress), in-progress (percent), completed, cancelled

### components/alert.html

Macro file (no `{% extends %}`):

```jinja
{% macro error_alert(message) %} … {% endmacro %}
```

Import with `{% import "components/alert.html" as components %}`.

## Rust Bindings (`crates/tepra/src/views.rs`)

| Struct                  | Template path               |
| ----------------------- | --------------------------- |
| `IndexTemplate`         | `pages/index.html`          |
| `PrinterDetailTemplate` | `pages/printer_detail.html` |
| `JobCardTemplate`       | `partials/job_card.html`    |

All three implement `askama::Template` and are wrapped in `HtmlTemplate<T>` for
axum `IntoResponse` compatibility.

## Related

- `docs/specs/architecture/pwa-asset-pipeline.md` — how CSS/JS assets are built and served
- `docs/adr/latest/0003-server-rendered-ui-with-askama-and-htmx.md`
- `docs/adr/latest/0007-ui-testing-strategy.md`
