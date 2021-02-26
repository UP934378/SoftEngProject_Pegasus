#ifndef _SCROLL_TEXT_H_
#define _SCROLL_TEXT_H_

#include <Arduino.h>
#include "SSD1306Wire.h"

class Scroll_Text{
  const char *str;
  int16_t x, y, text_width, offset ;
  uint8_t wait = 4;
  SSD1306Wire *display;

  public:
    Scroll_Text(SSD1306Wire *display, const char *str, int16_t x, int16_t y);
    void draw();
};

#endif