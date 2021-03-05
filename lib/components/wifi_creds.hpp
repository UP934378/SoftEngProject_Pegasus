#ifndef _WIFI_CREDS_H_
#define _WIFI_CREDS_H_
#include <Arduino.h>
#include "SSD1306Wire.h"

struct WiFi_creds{
    String SSID;
    String PSK;
};

/**
 * Creates a WiFi hotspot and serves a web form for the user to connect and
 * provide new WiFi credentials
 */
WiFi_creds get_wifi_creds(SSD1306Wire &display);

/**
 * Saves WiFi credentials to EEPROM
 */ 
void save_wifi_creds(WiFi_creds creds);

/**
 * Loads WiFi credentials from EEPROM
 */
WiFi_creds load_wifi_creds();

#endif
