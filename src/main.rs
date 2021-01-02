#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use web_back::*;

use rocket_contrib::helmet::SpaceHelmet;
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;

#[get("/api/me")]
fn get_api_me() -> Json<api::Me> {
	Json(api::Me {
		name: "Charlie".into(),
	})
}

fn main() {
	// If the API_EXPORT env variable is set, only export the API bindings
	if std::env::var("API_EXPORT").is_ok() {
		if let Err(x) = api::export("../web-front/src/API_bindings.ts") {
			panic!("Can't export: {}", x);
		}

		println!("API bindings exported");
		return;
	}

	// Start the server
	rocket::ignite()
		.attach(SpaceHelmet::default())
		.mount("/", routes![get_api_me])
		.mount("/", StaticFiles::from("../web-front/dist/"))
		.launch();
}
