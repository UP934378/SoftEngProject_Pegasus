#ifndef _NETWORK_DISCOVERY_H_
#define _NETWORK_DISCOVERY_H_
#include <WebServer.h>
#include "wifi_creds.hpp"
#include "SSD1306Wire.h"
#include "ESP32SSDP.h"
void setUpSSDP(WebServer* server, SSD1306Wire* display, WiFi_creds creds);

#endif