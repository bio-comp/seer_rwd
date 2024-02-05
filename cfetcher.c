#include <curl/curl.h>

/**
 * @brief Initializes cURL
 * 
 */
void initialize(){
  curl_global_init(CURL_GLOBAL_DEFAULT);
}

/**
 * @brief Main body orchestration
 * 
 * @return int Status of main execution
 */
int main(){
  initialize();
  return(1);
}