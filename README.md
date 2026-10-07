# OpenDeck W20

**OpenDeck W20** ist ein kleines natives OpenAction-Plugin für [OpenDeck](https://github.com/nekename/OpenDeck), das einen digitalen 20-seitigen Würfel direkt auf einer Deck-Taste bereitstellt.

Ein Tastendruck startet den Wurf. Während der nächsten **3 Sekunden** wechseln die Zahlen sichtbar zwischen **1 und 20** und werden zum Ende hin etwas langsamer. Anschließend bleibt das gewürfelte Ergebnis auf der Taste stehen, bis der nächste Wurf gestartet wird.

## Funktionen

- **W20 mit gleichmäßigem Wertebereich von 1 bis 20**
- **3 Sekunden sichtbare Würfelanimation**
- Ergebnis bleibt nach dem Wurf dauerhaft auf der Taste stehen
- Neuer Tastendruck startet sofort einen neuen Wurf
- Ein Tastendruck während eines laufenden Wurfs startet die 3-Sekunden-Sequenz neu
- Farbige Ergebnisdarstellung:
  - **20** → grün
  - **1** → rot
  - **2–19** → blau/violett
- Keine Konfiguration oder Property-Inspector-Einstellungen erforderlich
- Eigenständiges natives Rust/OpenAction-Plugin
- Builds für Windows, macOS und Linux

## Bedienung

Nach der Installation findest du die Aktion in OpenDeck unter:

**Spiele → W20 Würfel**

Ablauf:

1. W20-Aktion auf eine Deck-Taste legen.
2. Taste drücken.
3. Der Würfel läuft für 3 Sekunden durch verschiedene Werte.
4. Danach wird das finale Ergebnis angezeigt.
5. Das Ergebnis bleibt stehen, bis du erneut drückst.

## Darstellung

Im Ruhezustand zeigt die Taste einen stilisierten W20 mit **„DRÜCKEN“**.

Während des Wurfs wird der aktuelle Zwischenwert zusammen mit **„WÜRFELT…“** angezeigt. Nach dem Stoppen erscheint die endgültige Zahl mit **„ERGEBNIS“**.

Die Darstellung wird direkt als SVG erzeugt und anschließend von OpenDeck auf der Taste dargestellt. Es werden keine externen Grafiken, Server oder Netzwerkverbindungen benötigt.

## Technische Details

- Sprache: **Rust**
- OpenAction: **2.8**
- Zufallswerte: **rand 0.8**
- Asynchrone Laufzeit: **Tokio**
- Action UUID: `de.spliter90.w20.roll`
- Plugin-ID: `de.spliter90.w20.sdPlugin`
- Aktuelle Version: **0.1.0**

Die Würfelanimation läuft in einem eigenen asynchronen Task pro Aktionsinstanz. Wird während eines laufenden Wurfs erneut gedrückt, wird der vorherige Task beendet und unmittelbar ein neuer Wurf gestartet.

## Unterstützte Plattformen

GitHub Actions erzeugt Plugin-Builds für:

| Plattform | Ziel |
| --- | --- |
| Windows | x86_64-pc-windows-msvc |
| macOS Intel | x86_64-apple-darwin |
| macOS Apple Silicon | aarch64-apple-darwin |
| Linux x86_64 | x86_64-unknown-linux-gnu |
| Linux ARM64 | aarch64-unknown-linux-gnu |

## Linux lokal bauen

Voraussetzung ist eine aktuelle stabile Rust-Toolchain.

```bash
git clone https://github.com/spliter90/OpenDesk-W20.git
cd OpenDesk-W20
./scripts/build-linux.sh
```

Das fertige Plugin liegt anschließend unter:

```text
dist/de.spliter90.w20.sdPlugin
```

### Direkt installieren

```bash
./scripts/install-linux.sh
```

Das Installationsskript erkennt sowohl eine normale OpenDeck-Installation als auch die Flatpak-Konfiguration. Anschließend OpenDeck neu starten und die Aktion **Spiele → W20 Würfel** auf eine Taste ziehen.

### Linux ARM64

```bash
./scripts/build-linux.sh aarch64-unknown-linux-gnu
./scripts/install-linux.sh aarch64-unknown-linux-gnu
```

## Automatische Builds

Der Workflow unter `.github/workflows/build.yml` führt Tests und Release-Builds aus und verpackt jede Zielplattform als eigenes Plugin-ZIP.

Er wird ausgeführt bei:

- Pull Requests auf `main`
- Pushes auf `main`
- manueller Ausführung
- Tags nach dem Muster `v*`

## Projektstruktur

```text
.
├── .github/workflows/build.yml
├── assets/
│   ├── action.svg
│   ├── icon.svg
│   └── manifest.json
├── scripts/
│   ├── build-linux.sh
│   └── install-linux.sh
├── src/
│   └── main.rs
├── Cargo.toml
└── README.md
```

## Lizenz

Dieses Projekt steht unter der **MIT-Lizenz**.
