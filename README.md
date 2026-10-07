# OpenDeck W20

Ein kleines natives OpenAction-Plugin für OpenDeck: Ein Tastendruck startet einen animierten **W20 (1–20)**.

## Verhalten

- Taste drücken → der Würfel startet sofort.
- Die angezeigte Zahl wechselt sichtbar zwischen 1 und 20.
- Nach **3 Sekunden** stoppt der Würfel automatisch.
- Das Ergebnis bleibt auf der Taste stehen, bis erneut gedrückt wird.
- Wird während eines laufenden Wurfs nochmals gedrückt, startet der 3-Sekunden-Wurf neu.
- Eine **20** wird grün hervorgehoben.
- Eine **1** wird rot hervorgehoben.

## OpenDeck

Nach der Installation findest du die Aktion unter:

**Spiele → W20 Würfel**

Die Aktion braucht keine Konfiguration.

## Linux bauen und installieren

Voraussetzung: Rust stable.

```bash
./scripts/build-linux.sh
./scripts/install-linux.sh
```

Danach OpenDeck neu starten.

Für Linux ARM64:

```bash
./scripts/build-linux.sh aarch64-unknown-linux-gnu
./scripts/install-linux.sh aarch64-unknown-linux-gnu
```

## GitHub Actions

Der Workflow baut Plugin-ZIPs für:

- Windows x86_64
- macOS x86_64
- macOS ARM64
- Linux x86_64
- Linux ARM64

Der Workflow läuft bei Pull Requests auf `main`, Pushes auf `main`, manuell und bei `v*`-Tags.

## Lizenz

MIT
