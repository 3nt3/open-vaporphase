# Hardware

## Requirements

- Active cooling
- Ventilation

## BOM

| Qty | Part Description | Part Number | Source | Price |
| --- | ----------------------------------------------------------- | ------------------ | ------------- |
| 1 | METRO Professional GN-Behälter 1/3, 200 mm, Edelstahl 18/10 | METRO 221430 | METRO | |
| 2 | 200 x 100 Silicone Heating Pad 220 V, 80 W | N/A | Aliexpress | |
| 1 | Controller PCB (ESP-32 C3 based maybe?) | TODO | DIY | |
| 1 | RS PRO Schwarz Kunststoff Griff, L 112mm H 28 mm B 22mm, Lochabst. 92mm | 456-589 | RS Components | 8 |
| 1 | Schurter IEC-Steckverbinder C22 250 V, Gerade, Tafelmontage, Stecker / 16A, Schrauben, 3-polig | 866-2640 | RS Components | 3 |
| 2 | DIN 7991 3 x 10 | N/A | N/A |
| 1 | M6 Temperature Sensor | N/A | Aliexpress | 2 |

# Safety

We try to follow relevant EU legislature Directive 2006/42/EC (Machinery Directive) as a source of best practices.

## Hazards

- Thermal: High-temperature fluids and surfaces
- Chemical: Vapor phase fluids (PFPE, Galden, etc.)
- Electrical

## Ideas

- Heater control fails OFF
- Loss of sensor input disables heating
- Prevention of runoff heating when liquid is missing
- Lid-open condition prevents heating
- Manual emergency stop immediately removes power
