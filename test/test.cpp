#include <Arduino.h>
#include <unity.h>

#include "SSD1306Wire.h"   
#include "wifi_creds.hpp"
#include <EEPROM.h>
// #include <BLEDevice.h>

SSD1306Wire display(0x3c, SDA, SCL, GEOMETRY_128_32);

void test_display(){
    display.init();
    display.flipScreenVertically();
    display.setFont(ArialMT_Plain_10);
    display.drawString(0,0,"Hello!");
    display.display();
    delay(2000);
    UNITY_TEST_ASSERT(true, 15, "fail");
}

bool compare_EEPROM(String s, uint16_t address){
    for (const char c : s){
        if (c != EEPROM.read(address)){
            return false;
        }
        ++address;
    }

    // check for null terminator
    return EEPROM.read(address) == 0;
}

void test_wifi_creds(){
    WiFi_creds creds = {
        "network_ssid",
        "network_psk"
    };
    if (EEPROM.read(0) != 0){
        EEPROM.write(0,0);
        EEPROM.commit(); 
    }
    
    save_wifi_creds(creds);
    UNITY_TEST_ASSERT(EEPROM.read(0) == 1, 44, 
                      "save_wifi_creds didn't set the EEPROM byte "
                      "to indicate wifi credentials are saved.");
    UNITY_TEST_ASSERT(compare_EEPROM(creds.SSID, 1), 47,
                      "Stored SSID did not match expected value.");
    UNITY_TEST_ASSERT(compare_EEPROM(creds.PSK, 2 + creds.SSID.length()), 49,
                      "Stored PSK did not match expected value.");
}

void setup(){
    EEPROM.begin(512);
    UNITY_BEGIN();
    
    RUN_TEST(test_display);
    RUN_TEST(test_wifi_creds);
    
    UNITY_END();
    display.displayOff();
}

void loop(){

}