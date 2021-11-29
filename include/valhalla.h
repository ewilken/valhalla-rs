#pragma once
#include "cxx.h"
#include <memory>
#include <string>

class ValhallaClient {
public:
  ValhallaClient(const std::string &json);
  rust::string route(const std::string &request) const;
  rust::string route_proto(const uint8_t* request_pointer, const size_t request_size) const;
  rust::string locate(const std::string &request) const;
  rust::string optimized_route(const std::string &request) const;
  rust::string matrix(const std::string &request) const;
  rust::string isochrone(const std::string &request) const;
  rust::string trace_route(const std::string &request) const;
  rust::string trace_attributes(const std::string &request) const;
  rust::string height(const std::string &request) const;
  rust::string transit_available(const std::string &request) const;
  rust::string expansion(const std::string &request) const;
  rust::string centroid(const std::string &request) const;

private:
  class impl;
  std::shared_ptr<impl> impl;
};

std::unique_ptr<ValhallaClient> new_valhalla_client(const std::string &json);

