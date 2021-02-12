#include <Arduino.h>
#include <EEPROM.h>
#include <string.h>
#include <WiFi.h>
#include <WiFiClient.h>
#include <WiFiAP.h>
// #include <BLEDevice.h>

WiFiServer server;


void setup() {
  Serial.begin(115200);
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
    unsigned char mac[6];
    WiFi.macAddress(mac);
    char id[7] = {0};
    SSID = "Pegassas-Probe-";
    snprintf(id, 7, "%02X%02X%02X", mac[3], mac[4], mac[5]);
    SSID += id;
    // BLEDevice::init("SSID");
    // if (!BLEDevice::getInitialized()){
    //   Serial.println("Bluetooth not running");
    // }
    uint32_t key_raw = esp_random();
    char key[9] = {0};
    snprintf(key, 9, "%08X", key_raw);
    Serial.println(SSID);
    Serial.println(key);
    WiFi.softAP(SSID.c_str(), key);
    server = WiFiServer(80);
  }
}

void loop() {
  // put your main code here, to run repeatedly:
}