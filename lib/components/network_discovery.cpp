#include "network_discovery.hpp"


void setUpSSDP(WebServer* server, SSD1306Wire* display, WiFi_creds creds){
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
    display->clear();
    display->drawString(0, 0, creds.SSID);
    display->drawString(0, 10, localIp);
    display->display();
    
    server->on("/description.xml", HTTP_GET, [server]() {
      SSDP.schema(server->client());
    });
    server->on("/data.json", HTTP_GET, [server]() {
      server->send(200, "application/json", "{\"name\" : \"cell_id\"}");
    });
    server->begin();
    SSDP.setSchemaURL("description.xml");
    SSDP.setHTTPPort(80);
    SSDP.setName(deviceName);
    SSDP.setURL("data.json");
    SSDP.setDeviceType("urn:pegassas:service:data-probe:1");
    SSDP.begin();
}
