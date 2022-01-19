#pragma once
#include "cxx.h"
#include <memory>
#include <string>

class ValhallaJsonClient {
public:
  ValhallaJsonClient(const std::string &json);
  rust::string proto_route(const std::string &request) const;
  rust::string route(const std::string &request) const;
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

std::unique_ptr<ValhallaJsonClient>
new_valhalla_json_client(const std::string &json);

class ValhallaProtobufClient {
public:
  ValhallaProtobufClient(const std::string &json);
  std::string route(const std::string &request) const;
  std::string locate(const std::string &request) const;
  std::string optimized_route(const std::string &request) const;
  std::string matrix(const std::string &request) const;
  std::string isochrone(const std::string &request) const;
  std::string trace_route(const std::string &request) const;
  std::string trace_attributes(const std::string &request) const;
  std::string height(const std::string &request) const;
  std::string transit_available(const std::string &request) const;
  std::string expansion(const std::string &request) const;
  std::string centroid(const std::string &request) const;

private:
  class impl;
  std::shared_ptr<impl> impl;
};

std::unique_ptr<ValhallaProtobufClient>
new_valhalla_protobuf_client(const std::string &json);

