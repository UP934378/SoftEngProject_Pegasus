#include <Arduino.h>
#include <EEPROM.h>
#include <string.h>
#include <WiFi.h>
#include <WiFiClient.h>
#include <WebServer.h>
#include <WiFiAP.h>
#include <Wire.h>               // Only needed for Arduino 1.6.5 and earlier
#include "SSD1306Wire.h"
#include "scroll_text.hpp"
#include "wifi_creds.hpp"
// #include <BLEDevice.h>

SSD1306Wire display(0x3c, SDA, SCL, GEOMETRY_128_32);

void setup() {
  EEPROM.begin(512);
  Serial.begin(115200);
  display.init();
  display.flipScreenVertically();
  display.setFont(ArialMT_Plain_10);
  display.drawString(0, 0, "Hello world");
  display.display();
  // put your setup code here, to run once:
  bool hasWiFi = EEPROM.read(0) == 1;
  String SSID = "";
  String PSK = "";

  // Read SSID and PSK from EEPROM if wifi credentials are stored
  if (hasWiFi){
    int i = 1;
    for (;EEPROM.read(i) && i < 512; ++i){
      SSID += (char)EEPROM.read(i);
    }
    ++i;
    for(;EEPROM.read(i) && i < 512; ++i){
      PSK += (char)EEPROM.read(i);
    }
  }

  if (hasWiFi){
    display.clear();
    display.drawString(0, 0, SSID);
    display.display();
  }else{
    WiFi_creds creds = get_wifi_creds(display);
    //Serial.println(creds.SSID);
    save_wifi_creds(creds);
  }
}

void loop() {
  // web_server.handleClient();
}
