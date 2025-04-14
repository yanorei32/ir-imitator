# IR Imitator
A simple IR imitator (overkill way).

![image](https://github.com/user-attachments/assets/e6441ff6-ce10-4197-ac49-8e65ca5d674f)

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
