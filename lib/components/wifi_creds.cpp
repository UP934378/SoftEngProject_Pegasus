#include "wifi_creds.hpp"
#include <WebServer.h>
#include "files.hpp"
#include "scroll_text.hpp"
#include <EEPROM.h>


WiFi_creds get_wifi_creds(SSD1306Wire &display){
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
  
  
  #ifdef DEV_PSK
  // WiFi PSK of hotspot
  char key[] = DEV_PSK;
  #else
  char key[9] = {0};
  uint32_t key_raw = esp_random();
  snprintf(key, 9, "%08X", key_raw);
  #endif
  Serial.println(SSID);
  Serial.println(key);
  WiFi.softAP(SSID.c_str(), key);

  display.clear();
  char buffer[32];
  display.drawStringf(0, 0, buffer, "SSID: %s", SSID.c_str());
  display.drawStringf(0, 10, buffer, "PSK: %s", key);
  display.drawString(0, 20, "This is a very long string to test what happens");
  display.display();
  web_server.on("/", [&web_server]() {
    web_server.send(200, "text/html", wifi_form_html);
  });
  web_server.on("/form", [&web_server, &new_SSID, &new_PSK](){
    new_SSID = web_server.arg("SSID");
    new_PSK = web_server.arg("PSK");
    web_server.send(200, "text/plain", "");
  });
  web_server.begin();
  unsigned long draw_time = millis();
  const int16_t DRAW_INTERVAL = 200;
  const int16_t SSID_OFFSET = display.getStringWidth("SSID: ", 6);
  Scroll_Text ssid_scroll = Scroll_Text(&display, SSID.c_str(), SSID_OFFSET, 0);
  while (new_SSID.length() == 0 || new_PSK.length() < 8){
    web_server.handleClient();
    unsigned long current_time = millis();

    if (current_time > draw_time + DRAW_INTERVAL){
      draw_time += DRAW_INTERVAL;
      display.clear();
      ssid_scroll.draw();
      display.setColor(BLACK);
      display.fillRect(0, 0, SSID_OFFSET, 15);
      display.setColor(WHITE);
      display.drawString(0,0, "SSID:");
      display.drawStringf(0, 10, buffer, "PSK: %s", key);
      display.display();
    }
  }
  WiFi_creds creds;
  creds.SSID = new_SSID;
  creds.PSK = new_PSK;
  return creds;
}

// Saves wifi credentials to EEPROM
void save_wifi_creds(WiFi_creds creds) {
    if (creds.SSID.length() == 0 || creds.PSK.length() < 8){
      EEPROM.write(0,0);
      EEPROM.write(1,0);
      EEPROM.write(2,0);
      return;
    }
    EEPROM.write(0, 1);

    int addr = 1;

    for (const char c : creds.SSID){
      EEPROM.write(addr, c);
      addr += 1;
    }

    EEPROM.write(addr, 0);
    ++addr;

    for (const char c : creds.PSK) {
      EEPROM.write(addr, c);
      addr += 1;
    }
    EEPROM.write(addr, 0);
    
    EEPROM.commit();
}
