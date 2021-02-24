#ifndef _WIFI_CREDS_H_
#define _WIFI_CREDS_H_
#include <Arduino.h>
#include "SSD1306Wire.h"

struct WiFi_creds{
    String SSID;
    String PSK;
};

WiFi_creds get_wifi_creds(SSD1306Wire &display);
void save_wifi_creds(WiFi_creds creds);

#endif
