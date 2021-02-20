#include <Arduino.h>
#include <EEPROM.h>
#include <string.h>
#include <WiFi.h>
#include <WiFiClient.h>
#include <WebServer.h>
#include <WiFiAP.h>
#include <Wire.h>               // Only needed for Arduino 1.6.5 and earlier
#include "SSD1306Wire.h"   
// #include <BLEDevice.h>

SSD1306Wire display(0x3c, SDA, SCL);

// HTML source code c-string built into program
extern const char wifi_form_html[] asm("_binary_src_wifi_form_html_start");

struct WiFi_creds{
    String SSID;
    String PSK;
};

WiFi_creds get_wifi_creds();

void setup() {
  Serial.begin(115200);
  display.init();
  display.flipScreenVertically();
  display.setFont(ArialMT_Plain_24);
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

  }else{
    get_wifi_creds();
  }
}

void loop() {
  // web_server.handleClient();
}


WiFi_creds get_wifi_creds(){
  // WiFi SSID of hotspot
  String SSID;
  // WebServer used to serve form for user to provide new WiFi credentials
  WebServer web_server(80);

  // WiFi SSID to connect to
  String new_SSID = "";
  // WiFi PSK to connect with
  String new_PSK = "";

  unsigned char mac[6];
  WiFi.macAddress(mac);
  char id[7] = {0};
  SSID = "Pegassas-Probe-";
  snprintf(id, 7, "%02X%02X%02X", mac[3], mac[4], mac[5]);
  SSID += id;
  uint32_t key_raw = esp_random();
  char key[9] = {0};
  snprintf(key, 9, "%08X", key_raw);
  Serial.println(SSID);
  Serial.println(key);
  #ifdef DEV_PSK
  WiFi.softAP(SSID.c_str(), DEV_PSK);
  #else
  WiFi.softAP(SSID.c_str(), key);
  #endif
  web_server.on("/", [&web_server]() {
    web_server.send(200, "text/html", wifi_form_html);
  });
  web_server.on("/form", [&web_server](){
    Serial.println(web_server.args());
    for (int i = 0; i < web_server.args(); ++i){
      Serial.print(web_server.argName(i));
      Serial.print(": ");
      Serial.println(web_server.arg(i));
    }
    web_server.send(200, "text/html", "");
  });
  web_server.begin();
  while (new_SSID.length() == 0 || new_PSK.length() < 8){
    web_server.handleClient();
  }
  WiFi_creds creds;
  creds.SSID = new_SSID;
  creds.PSK = new_PSK;
  return creds;
}

