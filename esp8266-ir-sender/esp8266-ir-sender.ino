#include <Arduino.h>
#if defined(ESP8266)
#include <ESP8266WiFi.h>
#include <WiFiUdp.h>
#endif  // ESP8266
#if defined(ESP32)
#include <WiFi.h>
#include <WiFiUdp.h>
#endif  // ESP32
#include <IRremoteESP8266.h>
#include <IRsend.h>
#include <WiFiClient.h>
#include <WiFiServer.h>

#include "wificred.h"

#define UDPBUF_SIZE 2048
char udpBuffer[UDPBUF_SIZE];

#define UDP_PORT 6464
WiFiUDP Server;

#define IR_LED 3
IRsend irsend(IR_LED);

void setup() {
  Serial.begin(115200, SERIAL_8N1, SERIAL_TX_ONLY);
  delay(100);

  Serial.println();
  Serial.println("IR UDP Server");

  WiFi.begin(SSID, PSK);

  while (WiFi.status() != WL_CONNECTED) {
    delay(250);
    Serial.print(".");
  }

  Serial.println(WiFi.localIP().toString());
  Server.begin(UDP_PORT);
  irsend.begin();
}

inline bool isValidPacket(uint16_t *buffer, uint16_t dataLen) {
  uint16_t sum = 0;
  while (--dataLen) sum += *buffer++;
  return sum == *buffer;
}

void loop() {
  uint16_t packetLen = Server.parsePacket();
  if (!packetLen) return;

  Serial.print("Packet (len: ");
  Serial.print(packetLen);
  Serial.print(") from ");
  Serial.print(Server.remoteIP());
  Serial.print(":");
  Serial.print(Server.remotePort());
  Serial.println();

  if (packetLen & 1) {
    Serial.println("Ignore invalid packet length");
    return;
  }

  if (packetLen > UDPBUF_SIZE) {
    Serial.println("Ignore too long packet");
    return;
  }

  packetLen = Server.read(udpBuffer, UDPBUF_SIZE);

  if (packetLen & 1) {
    Serial.println("Ignore invalid packet length");
    return;
  }

  // divide by 2 (bytes)
  uint16_t dataLen = packetLen >> 1;

  if (!(dataLen >> 2)) {
    Serial.println("Ignore too short data");
    return;
  }

  if (!isValidPacket((uint16_t *)udpBuffer, dataLen)) {
    Serial.println("Invalid checksum");
    return;
  }

  // remove checksum
  dataLen--;

  // remove frequency
  dataLen--;

  irsend.sendRaw(
    &((uint16_t *)udpBuffer)[1],
    dataLen,
    ((uint16_t *)udpBuffer)[0]
  );
}
