// Inspiration taken from the Python API: https://github.com/valhalla/valhalla/blob/d2914a4527f2aeb7495dfdbacea60fdc0b92c6a0/src/bindings/python/python.cc

#include <boost/make_shared.hpp>
#include <boost/noncopyable.hpp>
#include <boost/optional.hpp>
#include <boost/property_tree/ptree.hpp>
#include <boost/shared_ptr.hpp>
#include <sstream>
#include <string>

#include <valhalla/baldr/rapidjson_utils.h>
#include <valhalla/midgard/logging.h>
#include <valhalla/midgard/util.h>
#include <valhalla/tyr/actor.h>

boost::property_tree::ptree configure(const std::string& config) {
  static boost::property_tree::ptree pt;
  try {
      // parse the config
      boost::property_tree::ptree temp_pt;
      rapidjson::read_json(config, temp_pt);
      pt = temp_pt;

      // // configure logging
      // boost::optional<boost::property_tree::ptree&> logging_subtree =
      //     pt->get_child_optional("tyr.logging");
      // if (logging_subtree) {
      //   auto logging_config = valhalla::midgard::ToMap<const boost::property_tree::ptree&,
      //                                                  std::unordered_map<std::string, std::string>>(
      //       logging_subtree.get());
      //   valhalla::midgard::logging::Configure(logging_config);
      // }
    } catch (...) { throw std::runtime_error("Failed to load config from: " + config); }

  return pt;
}

struct simplified_actor_t : public valhalla::tyr::actor_t {
  simplified_actor_t(const std::string& config)
      : valhalla::tyr::actor_t::actor_t(configure(config), true) {
  }

  std::string route(const std::string& request_str) {
    return valhalla::tyr::actor_t::route(request_str, nullptr, nullptr);
  };
  std::string locate(const std::string& request_str) {
    return valhalla::tyr::actor_t::locate(request_str, nullptr, nullptr);
  };
  std::string optimized_route(const std::string& request_str) {
    return valhalla::tyr::actor_t::optimized_route(request_str, nullptr, nullptr);
  };
  std::string matrix(const std::string& request_str) {
    return valhalla::tyr::actor_t::matrix(request_str, nullptr, nullptr);
  };
  std::string isochrone(const std::string& request_str) {
    return valhalla::tyr::actor_t::isochrone(request_str, nullptr, nullptr);
  };
  std::string trace_route(const std::string& request_str) {
    return valhalla::tyr::actor_t::trace_route(request_str, nullptr, nullptr);
  };
  std::string trace_attributes(const std::string& request_str) {
    return valhalla::tyr::actor_t::trace_attributes(request_str, nullptr, nullptr);
  };
  std::string height(const std::string& request_str) {
    return valhalla::tyr::actor_t::height(request_str, nullptr, nullptr);
  };
  std::string transit_available(const std::string& request_str) {
    return valhalla::tyr::actor_t::transit_available(request_str, nullptr, nullptr);
  };
  std::string expansion(const std::string& request_str) {
    return valhalla::tyr::actor_t::expansion(request_str, nullptr, nullptr);
  };
  std::string centroid(const std::string& request_str) {
    return valhalla::tyr::actor_t::centroid(request_str, nullptr, nullptr);
  };
};
