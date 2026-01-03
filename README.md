# Open Vaporphase

![Open Vaporphase Logo](img/logo.png)

# Hardware

## Requirements

- Active cooling
- Ventilation
- Lid switch

## BOM

| Qty | Part Description | Part Number | Source | Price |
| --- | ----------------------------------------------------------- | ------------------ | ------------- | --- |
| 1 | METRO Professional GN-Behälter 1/3, 200 mm, Edelstahl 18/10 | METRO 221430 | METRO | |
| 1 | RS PRO Mica-Heizungspolster 300 W +260°C, 230 V ac | 790-4833 | RS Components | 60 |
| 1 | Controller PCB (ESP-32 C3 based maybe?) | TODO | DIY | |
| 1 | RS PRO Schwarz Kunststoff Griff, L 112mm H 28 mm B 22mm, Lochabst. 92mm | 456-589 | RS Components | 8 |
| 1 | Schurter IEC-Steckverbinder C22 250 V, Gerade, Tafelmontage, Stecker / 16A, Schrauben, 3-polig | 866-2640 | RS Components | 3 |
| 2 | DIN 7991 3 x 10 | N/A | N/A |
| 1 | M6 Temperature Sensor | N/A | Aliexpress | 2 |
| 1 | 3.5" IPS TFT LCD Module in 320x480,OPTL TouchScreen,w/ILI9488 | N/A | BuyDisplay or Aliexpress | 1 |
| 2 | Limitor SF/R Thermosicherung +229°C +200°C 15 A 250V ac axial, Geh.ø 4mm, 8.5mm x 4mm x 8.5mm | 797-6030 | RS Components | 1 |
| 1 | Thermal paste | N/A | Aliexpress | 5 |
| 1 | Phoenix Contact Stahl Hutschiene Ungelocht, H. 8mm B. 35mm, L. 250mm | 129-652 | RS Components | 4 |

## Safety

We try to follow relevant EU legislature Directive 2006/42/EC (Machinery Directive) as a source of best practices.

### Hazards

- Thermal: High-temperature fluids and surfaces
- Chemical: Vapor phase fluids (PFPE, Galden, etc.)
- Electrical

### Ideas

- Heater control fails OFF
- Loss of sensor input disables heating
- Prevention of runoff heating when liquid is missing
- Lid-open condition prevents heating
- Manual emergency stop immediately removes power
