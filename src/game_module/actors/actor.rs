use std::rc::Rc;
use nalgebra::Vector3;

use rust_engine_3d::renderer::render_object::{RenderObjectData};
use rust_engine_3d::renderer::transform_object::TransformObjectData;
use rust_engine_3d::utilities::bounding_box::BoundingBox;
use rust_engine_3d::utilities::system::{RcRefCell, ptr_as_mut};
use crate::application::project_scene_manager::ProjectSceneManager;
use crate::game_module::game_client::GameClient;
use crate::game_module::ship::ship::{ShipInstance, ShipData};
use crate::game_module::ship::ship_controller::{ ShipController };

pub struct ActorData {
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ActionState {
    Idle,
    Punch,
    Fire,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MoveState {
    Idle,
    Walk
}

// ActorController
pub struct ActorController {
    pub _id: u64,
    pub _actor_data: ActorData,
    pub _ship: ShipInstance,
    pub _action_state: ActionState,
    pub _move_state: MoveState,
    pub _is_player_actor: bool
}

impl ActorController {
    pub fn create_actor_controller(
        id: u64,
        ship_data: &RcRefCell<ShipData>,
        render_object: &RcRefCell<RenderObjectData>,
        is_player_actor: bool
    ) -> Rc<ActorController> {
        Rc::new(ActorController {
            _id: id,
            _actor_data: ActorData {},
            _ship: ShipInstance::create_ship_instance(ship_data, render_object),
            _action_state: ActionState::Idle,
            _move_state: MoveState::Idle,
            _is_player_actor: is_player_actor
        })
    }

    pub fn initialize_actor(&mut self, project_scene_manager: &mut ProjectSceneManager) {
        self._ship.initialize_ship_instance(self, project_scene_manager);
    }
    pub fn remove_actor(&mut self, project_scene_manager: &mut ProjectSceneManager) {
        self._ship.remove_ship_instance(project_scene_manager);
    }
    pub fn get_actor_id(&self) -> u64 {
        self._id
    }
    pub fn is_player_actor(&self) -> bool {
        self._is_player_actor
    }
    pub fn get_actor_data(&self) -> &ActorData {
        &self._actor_data
    }
    pub fn get_actor_data_mut(&mut self) -> &mut ActorData {
        &mut self._actor_data
    }
    pub fn get_ship(&self) -> &ShipInstance {
        &self._ship
    }
    pub fn get_ship_mut(&mut self) -> &mut ShipInstance {
        &mut self._ship
    }
    pub fn get_controller(&self) -> &ShipController {
        &self._ship._controller
    }
    pub fn get_controller_mut(&mut self) -> &mut ShipController {
        &mut self._ship._controller
    }
    pub fn get_bound_box(&self) -> &BoundingBox {
        self._ship.get_bound_box()
    }
    pub fn get_transform(&self) -> &TransformObjectData {
        self._ship.get_transform()
    }
    pub fn get_transform_mut(&self) -> &mut TransformObjectData {
        self._ship.get_transform_mut()
    }
    pub fn get_velocity(&self) -> &Vector3<f32> {
        self.get_controller().get_velocity()
    }

    pub fn set_command_actor_fire(&mut self) {
        self._action_state = ActionState::Fire;
    }

    pub fn set_command_actor_walk(&mut self) {
        self._move_state = MoveState::Walk;
    }

    pub fn set_command_actor_stop(&mut self) {
        self._move_state = MoveState::Idle;
    }

    pub fn set_command_actor_idle(&mut self) {
        self._action_state = ActionState::Idle;
    }

    pub fn update_action_state(&mut self, game_client: &GameClient, _delta_time: f32) {
        if ActionState::Fire == self._action_state {
            self._ship.ship_fire(game_client);
            self.set_command_actor_idle();
        }
    }

    pub fn update_move_state(&mut self, _game_client: &GameClient, _delta_time: f32) {
        if MoveState::Walk == self._move_state {
            self.get_ship_mut()._controller.acceleration_forward(1.0);
            self.set_command_actor_stop();
        }
    }

    pub fn update_actor_controller(&mut self, game_client: &GameClient, delta_time: f32) {
        if self._is_player_actor {
            self.update_action_state(game_client, delta_time);
            self.update_move_state(game_client, delta_time);
        } else {
            let ship_controller = ptr_as_mut(&self.get_ship()._controller);
            ship_controller.set_velocity_yaw(1.0);
            ship_controller.acceleration_forward(1.0);
        }

        // update ship
        self.get_ship_mut().update_ship(game_client, delta_time);
    }
}
