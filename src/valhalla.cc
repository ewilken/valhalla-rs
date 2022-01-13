#include "valhalla.h"

#include "valhalla/baldr/rapidjson_utils.h"
#include "valhalla/tyr/actor.h"
#include "valhalla/loki/worker.h"
#include "valhalla/odin/worker.h"
#include "valhalla/thor/worker.h"
#include "valhalla/proto/api.pb.h"

#include <boost/property_tree/ptree.hpp>
#include <memory>
#include <string>

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
  impl(const boost::property_tree::ptree & config) 
      : actor(new valhalla::tyr::actor_t(config, false)),
        reader(new valhalla::baldr::GraphReader(config.get_child("mjolnir"))),
	loki_worker(config, reader),
	thor_worker(config, reader),
	odin_worker(config) {
  }

  friend ValhallaClient;
  std::unique_ptr<valhalla::tyr::actor_t> actor;

  std::shared_ptr<valhalla::baldr::GraphReader> reader;
  valhalla::loki::loki_worker_t loki_worker;
  valhalla::thor::thor_worker_t thor_worker;
  valhalla::odin::odin_worker_t odin_worker;
};

ValhallaClient::ValhallaClient(const std::string &json)
    : impl(new class ValhallaClient::impl(configure(json))) {}

std::string ValhallaClient::proto_route(const std::string &request_str) const {
  // parse the request
  valhalla::Api request;
  if (!request.ParseFromString(request_str)) {
      return request_str;
  }

  request.mutable_options()->set_action(valhalla::Options::route);
  //valhalla::ParseApi(request_str, valhalla::Options::route, request);
  // check the request and locate the locations in the graph
  impl->loki_worker.route(request);
  // route between the locations in the graph to find the best path
  impl->thor_worker.route(request);
  // get some directions back from them and serialize
  impl->odin_worker.narrate(request);

  std::string bytes;
  request.SerializeToString(&bytes);

  return bytes;
}

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

