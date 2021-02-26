#ifndef _ENVIRONMENT_H_
#define _ENVIRONMENT_H_

// Sets an environment variable that can be access via std::getenv
void setenv(const char *key, const char *value);

#endif
