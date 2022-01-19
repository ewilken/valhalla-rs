#include "valhalla.h"

#include "valhalla/baldr/rapidjson_utils.h"
#include "valhalla/loki/worker.h"
#include "valhalla/odin/worker.h"
#include "valhalla/proto/api.pb.h"
#include "valhalla/thor/worker.h"
#include "valhalla/tyr/actor.h"

#include <boost/property_tree/ptree.hpp>
#include <memory>
#include <string>

boost::property_tree::ptree configure(const std::string &config) {
  boost::property_tree::ptree pt;
  try {
    rapidjson::read_json(config, pt);
  } catch (...) {
    throw;
  }

  return pt;
}

class ValhallaJsonClient::impl {
  impl(const boost::property_tree::ptree &config)
      : actor(new valhalla::tyr::actor_t(config, false)) {}

  friend ValhallaJsonClient;
  std::unique_ptr<valhalla::tyr::actor_t> actor;
};

ValhallaJsonClient::ValhallaJsonClient(const std::string &json)
    : impl(new class ValhallaJsonClient::impl(configure(json))) {}

rust::string ValhallaJsonClient::route(const std::string &request) const {
  return impl->actor->route(request);
}

rust::string ValhallaJsonClient::locate(const std::string &request) const {
  return impl->actor->locate(request);
}

rust::string
ValhallaJsonClient::optimized_route(const std::string &request) const {
  return impl->actor->optimized_route(request);
}

rust::string ValhallaJsonClient::matrix(const std::string &request) const {
  return impl->actor->matrix(request);
}

rust::string ValhallaJsonClient::isochrone(const std::string &request) const {
  return impl->actor->isochrone(request);
}

rust::string ValhallaJsonClient::trace_route(const std::string &request) const {
  return impl->actor->trace_route(request);
}

rust::string
ValhallaJsonClient::trace_attributes(const std::string &request) const {
  return impl->actor->trace_attributes(request);
}

rust::string ValhallaJsonClient::height(const std::string &request) const {
  return impl->actor->height(request);
}

rust::string
ValhallaJsonClient::transit_available(const std::string &request) const {
  return impl->actor->transit_available(request);
}

rust::string ValhallaJsonClient::expansion(const std::string &request) const {
  return impl->actor->expansion(request);
}

rust::string ValhallaJsonClient::centroid(const std::string &request) const {
  return impl->actor->centroid(request);
}

std::unique_ptr<ValhallaJsonClient> new_valhalla_json(const std::string &json) {
  return std::unique_ptr<ValhallaJsonClient>(new ValhallaJsonClient(json));
}

class ValhallaProtobufClient::impl {
  impl(const boost::property_tree::ptree &config)
      : reader(new valhalla::baldr::GraphReader(config.get_child("mjolnir"))),
        loki_worker(config, reader), thor_worker(config, reader),
        odin_worker(config) {}

  friend ValhallaProtobufClient;

  std::shared_ptr<valhalla::baldr::GraphReader> reader;
  valhalla::loki::loki_worker_t loki_worker;
  valhalla::thor::thor_worker_t thor_worker;
  valhalla::odin::odin_worker_t odin_worker;
};

ValhallaProtobufClient::ValhallaProtobufClient(const std::string &json)
    : impl(new class ValhallaProtobufClient::impl(configure(json))) {}

std::string
ValhallaProtobufClient::route(const std::string &request_str) const {
  // parse the request
  valhalla::Api request;
  if (!request.ParseFromString(request_str)) {
    return request_str;
  }

  request.mutable_options()->set_action(valhalla::Options::route);
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

std::string
ValhallaProtobufClient::locate(const std::string &request_str) const {
  // parse the request
  valhalla::Api request;
  if (!request.ParseFromString(request_str)) {
    return request_str;
  }
  // check the request and locate the locations in the graph
  impl->loki_worker.locate(request);

  std::string bytes;
  request.SerializeToString(&bytes);

  return bytes;
}

std::string
ValhallaProtobufClient::matrix(const std::string &request_str) const {
  // parse the request
  valhalla::Api request;
  if (!request.ParseFromString(request_str)) {
    return request_str;
  }
  // check the request and locate the locations in the graph
  impl->loki_worker.matrix(request);
  // compute the matrix
  impl->thor_worker.matrix(request);

  std::string bytes;
  request.SerializeToString(&bytes);

  return bytes;
}

std::string
ValhallaProtobufClient::optimized_route(const std::string &request_str) const {
  // parse the request
  valhalla::Api request;
  if (!request.ParseFromString(request_str)) {
    return request_str;
  }
  // check the request and locate the locations in the graph
  impl->loki_worker.matrix(request);
  // compute compute all pairs and then the shortest path through them all
  impl->thor_worker.optimized_route(request);
  // get some directions back from them and serialize
  impl->odin_worker.narrate(request);

  std::string bytes;
  request.SerializeToString(&bytes);

  return bytes;
}

std::string
ValhallaProtobufClient::isochrone(const std::string &request_str) const {
  // parse the request
  valhalla::Api request;
  if (!request.ParseFromString(request_str)) {
    return request_str;
  }
  // check the request and locate the locations in the graph
  impl->loki_worker.isochrones(request);
  // compute the isochrones
  impl->thor_worker.isochrones(request);

  std::string bytes;
  request.SerializeToString(&bytes);

  return bytes;
}

std::string
ValhallaProtobufClient::trace_route(const std::string &request_str) const {
  // parse the request
  valhalla::Api request;
  if (!request.ParseFromString(request_str)) {
    return request_str;
  }
  // check the request and locate the locations in the graph
  impl->loki_worker.trace(request);
  // route between the locations in the graph to find the best path
  impl->thor_worker.trace_route(request);
  // get some directions back from them
  impl->odin_worker.narrate(request);

  std::string bytes;
  request.SerializeToString(&bytes);

  return bytes;
}

std::string
ValhallaProtobufClient::trace_attributes(const std::string &request_str) const {
  // parse the request
  valhalla::Api request;
  if (!request.ParseFromString(request_str)) {
    return request_str;
  }
  // check the request and locate the locations in the graph
  impl->loki_worker.trace(request);
  // get the path and turn it into attribution along it
  impl->thor_worker.trace_attributes(request);

  std::string bytes;
  request.SerializeToString(&bytes);

  return bytes;
}

std::string
ValhallaProtobufClient::height(const std::string &request_str) const {
  // parse the request
  valhalla::Api request;
  if (!request.ParseFromString(request_str)) {
    return request_str;
  }
  // get the height at each point
  impl->loki_worker.height(request);

  std::string bytes;
  request.SerializeToString(&bytes);

  return bytes;
}

std::string ValhallaProtobufClient::transit_available(
    const std::string &request_str) const {
  // parse the request
  valhalla::Api request;
  if (!request.ParseFromString(request_str)) {
    return request_str;
  }
  // check the request and locate the locations in the graph
  impl->loki_worker.transit_available(request);

  std::string bytes;
  request.SerializeToString(&bytes);

  return bytes;
}

std::string
ValhallaProtobufClient::expansion(const std::string &request_str) const {
  // parse the request
  valhalla::Api request;
  if (!request.ParseFromString(request_str)) {
    return request_str;
  }
  // check the request and locate the locations in the graph
  impl->loki_worker.route(request);
  // route between the locations in the graph to find the best path
  impl->thor_worker.expansion(request);

  std::string bytes;
  request.SerializeToString(&bytes);

  return bytes;
}

std::string
ValhallaProtobufClient::centroid(const std::string &request_str) const {
  // parse the request
  valhalla::Api request;
  if (!request.ParseFromString(request_str)) {
    return request_str;
  }
  // check the request and locate the locations in the graph
  impl->loki_worker.route(request);
  // route between the locations in the graph to find the best path
  impl->thor_worker.centroid(request);
  // get some directions back from them and serialize
  impl->odin_worker.narrate(request);

  std::string bytes;
  request.SerializeToString(&bytes);

  return bytes;
}

std::unique_ptr<ValhallaProtobufClient>
new_valhalla_protobuf_client(const std::string &json) {
  return std::unique_ptr<ValhallaProtobufClient>(
      new ValhallaProtobufClient(json));
}

