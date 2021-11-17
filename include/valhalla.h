#pragma once
#include <memory>
#include <string>
#include "cxx.h"

class ValhallaClient {
public:
   ValhallaClient(const std::string &json);
   rust::string route(const std::string &request) const;

private:
  class impl;
  std::shared_ptr<impl> impl;
};

std::unique_ptr<ValhallaClient> new_valhalla_client(const std::string &json);

