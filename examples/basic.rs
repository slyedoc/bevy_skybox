use bevy::prelude::*;
use bevy_skybox::{SkyboxPlugin, SkyboxCamera};

fn setup(mut commands: Commands) {
	commands
		.spawn_bundle(PerspectiveCameraBundle::default())
		.insert(SkyboxCamera);
}

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_startup_system(setup.system())
		.add_plugin(SkyboxPlugin::from_image_file("sky1.png"))
		.run();
}
