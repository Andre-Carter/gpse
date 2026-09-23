- time system
- logs, logs, logs
- notes, ledger, cli
- - can put cli::commands name, description into structs
- gpse mission control... system status... ONLINE... STANDBY
- gpse scientific console... INSTRUMENTS... calc, measure, constants, spectrum, convert, simulate, analyze
- NAVIGATION SYSTEM... position, distance, bearing, orbit, body, map
- GSPE TELEMETRY... CPU, MEMORY, SIMULATION, ENTITES, LEDGER... (SIMULATION VARIABLES)
- GPSE ARCHIVE: ledger, constants, research, entities, missions, datasets
- GAME/CLI IMAGES: BATTLESHIP, MINESWEEPER, ROUGE/GRID EXPLORER, TERMINAL STAR-MAP, TERMINAL DATABASE
- Chess
  ↓
Board system

Battleship
  ↓
Coordinate system

Conway
  ↓
Simulation system

Rogue
  ↓
Entity + world system

Star Map
  ↓
Celestial/spatial system

      ↓↓↓

        GPSE

- > read

READ SYSTEM

  bible
  manual
  textbook
  blueprint
  schematic
  research
  help
  back

  > read bible

BIBLE

  book
  chapter
  verse
  search
  parse
  index
  help
  back

  READ    → retrieve / navigate information
CALC    → perform calculations
SIM     → run simulations
MAP     → inspect spatial information
GAME    → interact with game systems
LOG     → record information

- COMMUNICATIONS
COMMS
│
├── Messages
│   ├── send
│   ├── receive
│   ├── reply
│   ├── inbox
│   └── history
│
├── Addresses / IDs
│   ├── sender
│   ├── recipient
│   └── message ID
│
├── Channels
│   ├── local
│   ├── simulation
│   └── network (future)
│
├── Telemetry
│   ├── status
│   ├── measurements
│   └── events
│
└── Sensors
    ├── temperature
    ├── pressure
    ├── distance
    ├── light
    ├── acceleration
    └── future custom sensors

