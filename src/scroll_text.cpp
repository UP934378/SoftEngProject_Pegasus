#include "scroll_text.hpp"

Scroll_Text::Scroll_Text(SSD1306Wire *display, const char *str, int16_t x, int16_t y){
    this->display = display;
    this->str = str;
    this->x = x;
    this->y = y;
    text_width = display->getStringWidth(str, strlen(str));
}

void Scroll_Text::draw(){
    if (wait){
    --wait;
    }else if (x + text_width - offset > display->width()){
    ++offset;
    if(x + text_width - offset == display->width()){
        wait = 4;
    }
    }else{
    offset = 0;
    wait = 4;
    }
    display->drawString(x - offset, y, str);
}