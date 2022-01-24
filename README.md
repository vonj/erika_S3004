# Erika s3004 Computer Interface

Tested on a SIGMA SM 8200i, which is supposedly the same hardware just under the brand name used in Western Germany.

This project is based on information from [Chaostreff Potsdam](https://github.com/Chaostreff-Potsdam/erika3004).

## Hardware
- A 5V USB TTL adapter (I used [this](https://www.amazon.de/USB-TTL-Konverter-Modul-mit-eingebautem-CP2102/dp/B00AFRXKFU) one, which I still had lying around)
- A few pieces of wire, to connect the pins of the TTL USB Adapter with the typewriter.

| USB TTL | Erika |
|---------|-------|
| RX      | TX    |
| TX      | RX    |
| 5V      | 5V    |
| CTS     | RTS   |
| GND     | GND   |
| RTS     | DTD   |

![Connectors](doc/connectors.svg)


![Cable wiring](doc/interface.JPG)

## Software

### Building and running

Building:
```
cargo build --release
```

Printing some text:
```
./target/release/erikad < text.txt
```

Reading keyboard input from the typewriter:
```
./target/release/erikad
```
