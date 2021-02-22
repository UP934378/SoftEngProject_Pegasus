#include <Arduino.h>
#include <unity.h>

#include "SSD1306Wire.h"   
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

void setup(){
    delay(5000);
    UNITY_BEGIN();
    
    RUN_TEST(test_display);
    
    UNITY_END();
    display.displayOff();
}

void loop(){

}