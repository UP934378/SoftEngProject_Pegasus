#include <Arduino.h>
#include <EEPROM.h>
#include <string.h>
#include <WiFi.h>
#include <WiFiClient.h>
#include <WebServer.h>
#include <WiFiAP.h>
#include <Wire.h> // Only needed for Arduino 1.6.5 and earlier
#include "SSD1306Wire.h"
#include "scroll_text.hpp"
#include "wifi_creds.hpp"
#include "ESPmDNS.h"
// #include <BLEDevice.h>

SSD1306Wire display(0x3c, SDA, SCL, GEOMETRY_128_32);

void setup()
{
  EEPROM.begin(512);
  Serial.begin(115200);
  display.init();
  display.flipScreenVertically();
  display.setFont(ArialMT_Plain_10);
  display.drawString(0, 0, "Hello world");
  display.display();
  // put your setup code here, to run once:
  bool hasWiFi = EEPROM.read(0) == 1;
  WiFi_creds creds;

  // Read SSID and PSK from EEPROM if wifi credentials are stored
  if (hasWiFi)
  {
    creds = load_wifi_creds();
  }

  if (hasWiFi)
  {
    //**Needs to be moved to separate component file** 
    unsigned char mac[6];
    WiFi.macAddress(mac);
    String deviceName;
    deviceName = "Pegassas-Probe-";
    char id[7] = {0};
    snprintf(id, 7, "%02X%02X%02X", mac[3], mac[4], mac[5]);
    deviceName += id; 
    WiFi.begin(creds.SSID.c_str(), creds.PSK.c_str());
    while (WiFi.status() != WL_CONNECTED)
    {
      delay(500);
      Serial.print(".");
    }
    String localIp = WiFi.localIP().toString();
    display.clear();
    display.drawString(0, 0, creds.SSID);
    display.drawString(0, 10, localIp);
    display.display();
    MDNS.begin(deviceName.c_str());
    MDNS.addService("pegassas", "tcp", 23);
    Serial.println("mDNS responder started");
  }
  else
  {
    WiFi_creds creds = get_wifi_creds(display);
    //Serial.println(creds.SSID);
    save_wifi_creds(creds);
  }
}

void loop()
{
  // web_server.handleClient();
}
