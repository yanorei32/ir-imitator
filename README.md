# IR Imitator
Customizable simple IR imitator (overkill way).

Currently, only simple single ir transmission is supported.

![image](https://github.com/user-attachments/assets/e6441ff6-ce10-4197-ac49-8e65ca5d674f)

## Example Configuration
```xml
<Controllers>
	<Controller id="HK9811">
		<VBox>
			<HBox>
				<Button label="全灯" action="./hk9811/1_on_max_brightness.csv.0.json" />
				<Button label="消灯" action="./hk9811/1_off.csv.0.json" />
			</HBox>
			<HBox>
				<Button label="普段" action="./hk9811/1_on_normal.csv.0.json" />
				<Button label="常夜灯" action="./hk9811/1_on_nightlight.csv.0.json" />
			</HBox>
			<HBox>
				<VBox>
					<Button label="白い色" action="./hk9811/1_temperature_cold.csv.0.json" />
					<Button label="温かい色" action="./hk9811/1_temperature_warm.csv.0.json" />
				</VBox>
				<VBox>
					<Button label="明るい" action="./hk9811/1_brightness_light.csv.0.json" />
					<Button label="暗い" action="./hk9811/1_brightness_dark.csv.0.json" />
				</VBox>
			</HBox>
		</VBox>
	</Controller>
</Controllers>
```

## Architecture
```
Web Browser
   `- HTTP POST -> web-ir-remote
                        `- UDP Packet -> ESP8266 IR Sender
```

## How to setup
1. Capture IR Packet with Saleae Logic 2
  - Don't include 38kHz baseband. (LPF is required)
1. Export Raw as CSV (ISO8601 format)
1. Extract IR Packet to JSON with ir-signal-extractor
   - Active-high and active-low are automatically detected.
1. Write Controller layout in XML
1. Startup Web Server
