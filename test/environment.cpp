#include "environment.hpp"
#include <map>
#include <string>

std::map<std::string, std::string> env_variables;

namespace std {
    extern "C" {
        char* getenv(const char * __string){
            auto i = env_variables.find(__string);
            if (i == env_variables.end()){
                return nullptr;
            }
            return (char*) i->second.c_str();
        }

    }
}

void setenv(const char *key, const char *value){
    env_variables[key] = value;
}
