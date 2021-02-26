#include <Arduino.h>
#include <unity.h>

#include "SSD1306Wire.h"   
#include "wifi_creds.hpp"
#include <EEPROM.h>
#include "environment.hpp"
#include <rapidcheck.h>
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

void test_save_wifi_creds(){
    UNITY_TEST_ASSERT(rc::check([](std::string SSID, std::string PSK){
        // Don't test for strings that are too long to store in the EEPROM
        RC_PRE((SSID.length() + PSK.length() + 3 <= 512));

        WiFi_creds creds = {
            SSID.c_str(),
            PSK.c_str()
        };
        save_wifi_creds(creds);
        // If SSID or PSK are invalid
        if (SSID.length() == 0 || PSK.length() < 8){
            // save_wifi_creds should set the first 3 bytes of EEPROM to 0 in the case of invalid SSID or PSK
            for (int i = 0; i < 3; ++i){
                if (EEPROM.read(i)){
                    return false;
                }
            }
            return true;
        }

        if (EEPROM.read(0) != 1){
            return false;
        }
        if (!compare_EEPROM(SSID.c_str(), 1)){
            return false;
        }
        if (!compare_EEPROM(PSK.c_str(), 2 + SSID.length())){
            return false;
        }
        return true;
    }), 69, "Rapidcheck test_save_wifi_creds fail");
}

void setup(){
    // set configuration variable for rapidcheck. This must be done before running
    // any tests with it or it will be ignored, even in subsequent tests.
    setenv("RC_PARAMS", "max_success=10");

    EEPROM.begin(512);
    UNITY_BEGIN();
    
    RUN_TEST(test_display);
    RUN_TEST(test_save_wifi_creds);
    
    
    UNITY_END();
    display.displayOff();
}

void loop(){

}