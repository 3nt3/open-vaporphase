# Hardware
## Requirements
- Active cooling

## BOM
| Qty | Part Description                                            | Part Number        | Source        |
| --- | ----------------------------------------------------------- | ------------------ | ------------- |
| 1   | METRO Professional GN-Behälter 1/3, 200 mm, Edelstahl 18/10 | METRO 221430       | METRO         |
| 2   | 200 x 100 Silicone Heating Pad 220 V, 80 W                  | N/A                | Aliexpress    |
| 1   | Controller PCB (ESP-32 C3 based maybe?)                     | TODO               | DIY           |

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

