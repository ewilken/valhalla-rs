#include "valhalla.h"

#include "valhalla/baldr/rapidjson_utils.h"
#include "valhalla/tyr/actor.h"

#include <boost/property_tree/ptree.hpp>
#include <memory>

boost::property_tree::ptree configure(const std::string &config) {
  boost::property_tree::ptree pt;
  try {
    rapidjson::read_json(config, pt);
  } catch (...) {
    throw;
    // throw std::runtime_error("Failed to load config from: " + config);
  }

  return pt;
}

class ValhallaClient::impl {
  impl(const std::string &json) {
    actor = std::make_unique<valhalla::tyr::actor_t>(
        valhalla::tyr::actor_t(configure(json), false));
  }

  friend ValhallaClient;
  std::unique_ptr<valhalla::tyr::actor_t> actor;
};

ValhallaClient::ValhallaClient(const std::string &json)
    : impl(new class ValhallaClient::impl(json)) {}

rust::string ValhallaClient::route(const std::string &request) const {
  return this->impl->actor->route(request);
}

rust::string ValhallaClient::locate(const std::string &request) const {
  return this->impl->actor->locate(request);
}

rust::string ValhallaClient::optimized_route(const std::string &request) const {
  return this->impl->actor->optimized_route(request);
}

rust::string ValhallaClient::matrix(const std::string &request) const {
  return this->impl->actor->matrix(request);
}

rust::string ValhallaClient::isochrone(const std::string &request) const {
  return this->impl->actor->isochrone(request);
}

rust::string ValhallaClient::trace_route(const std::string &request) const {
  return this->impl->actor->trace_route(request);
}

rust::string ValhallaClient::trace_attributes(const std::string &request) const {
  return this->impl->actor->trace_attributes(request);
}

rust::string ValhallaClient::height(const std::string &request) const {
  return this->impl->actor->height(request);
}

rust::string ValhallaClient::transit_available(const std::string &request) const {
  return this->impl->actor->transit_available(request);
}

rust::string ValhallaClient::expansion(const std::string &request) const {
  return this->impl->actor->expansion(request);
}

rust::string ValhallaClient::centroid(const std::string &request) const {
  return this->impl->actor->centroid(request);
}

std::unique_ptr<ValhallaClient> new_valhalla_client(const std::string &json) {
  return std::unique_ptr<ValhallaClient>(new ValhallaClient(json));
}

