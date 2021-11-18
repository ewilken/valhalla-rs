#include "valhalla.h"

#include "valhalla/tyr/actor.h"
#include "valhalla/baldr/rapidjson_utils.h"

#include <boost/property_tree/ptree.hpp>
#include <memory>

boost::property_tree::ptree configure(const std::string& config) {
   boost::property_tree::ptree pt;
   try {
	rapidjson::read_json(config, pt);
   } catch (...) {
	   throw;
	//throw std::runtime_error("Failed to load config from: " + config);
   }

   return pt;
}

class ValhallaClient::impl {
   impl(const std::string &json) {
	actor = std::make_unique<valhalla::tyr::actor_t>(valhalla::tyr::actor_t(configure(json), false)); 
   }

   friend ValhallaClient;
   std::unique_ptr<valhalla::tyr::actor_t> actor;

};

ValhallaClient::ValhallaClient(const std::string &json) : impl(new class ValhallaClient::impl(json)) {}

rust::string ValhallaClient::route(const std::string &request) const {
	return this->impl->actor->route(request);

}

std::unique_ptr<ValhallaClient> new_valhalla_client(const std::string &json) {
   return std::unique_ptr<ValhallaClient>(new ValhallaClient(json));
}

