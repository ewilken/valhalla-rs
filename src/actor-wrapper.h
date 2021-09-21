#ifndef VALHALLA_TYR_ACTOR_WRAPPER_H_
#define VALHALLA_TYR_ACTOR_WRAPPER_H_

#include <string>

#include <valhalla/tyr/actor.h>

struct simplified_actor_t {
  simplified_actor_t(const std::string& config);

  std::string route(const std::string& request_str);
  std::string locate(const std::string& request_str);
  std::string optimized_route(const std::string& request_str);
  std::string matrix(const std::string& request_str);
  std::string isochrone(const std::string& request_str);
  std::string trace_route(const std::string& request_str);
  std::string trace_attributes(const std::string& request_str);
  std::string height(const std::string& request_str);
  std::string transit_available(const std::string& request_str);
  std::string expansion(const std::string& request_str);
  std::string centroid(const std::string& request_str);
};

#endif // VALHALLA_TYR_ACTOR_WRAPPER_H_
